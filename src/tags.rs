use crate::reader;
use colored::*;
use std::collections::HashSet;
use yaml_rust::{Yaml, YamlEmitter};


#[derive(Debug)]
pub struct HarrawTags<'a> {
    pub tags: Option<HashSet<&'a str>>,
    pub skip_tags: Option<HashSet<&'a str>>,
}


impl <'a> HarrawTags<'a> {
    pub fn new(tags_option: Option<&'a str>, skip_tags_option: Option<&'a str>) -> Self {
        let tags: Option<HashSet<&str>> = tags_option.map(|m| m.split(',').map(|s|s.trim()).collect());
        let skip_tags: Option<HashSet<&str>> = skip_tags_option.map(|m| m.split(',').map(|s| s.trim()).collect());

        if let (Some(t), Some(s)) = (&tags, &skip_tags) {
            if !t.is_disjoint(s) {
                panic!("`tags` and `skip-tags` must not contain the same values!");
            }
        }
        HarrawTags {
            tags,
            skip_tags,
        }
    }

    pub fn hrw_should_skip_item(&self, item: &Yaml) -> bool {
        match item["tags"].as_vec() {
            Some(item_tags_raw) => {
                let item_tags: HashSet<&str> = item_tags_raw.iter().map(|t|t.as_str().unwrap()).collect();
                if let Some(s) = &self.skip_tags {
                    if !s.is_disjoint(&item_tags) {
                        return true;
                    }
                }

                if let Some(t) = &self.tags {
                    if item_tags.contains("never") && !t.contains("never") {
                        return true;
                    }

                    if !t.is_disjoint(&item_tags) {
                        return true;
                    }
                }

                if item_tags.contains("always") {
                    return false;
                }
                if item_tags.contains("always") {
                    return true;
                }
                self.tags.is_some()
            }
            None => self.tags.is_some(),
        }
    }
}

pub fn hrw_list_benchmark_file_tasks(benchmark_file: &str, tags: &HarrawTags) {
    let docs = reader::hrw_read_file_as_yml(benchmark_file);
  let items = reader::hrw_read_yaml_doc_accessor(&docs[0], Some("plan"));

  println!();

  if let Some(tags) = &tags.tags {
    let mut tags: Vec<_> = tags.iter().collect();
    tags.sort();
    println!("{:width$} {:width2$?}", "Tags".green(), &tags, width = 15, width2 = 25);
  }
  if let Some(tags) = &tags.skip_tags {
    let mut tags: Vec<_> = tags.iter().collect();
    tags.sort();
    println!("{:width$} {:width2$?}", "Skip-Tags".green(), &tags, width = 15, width2 = 25);
  }
  let items: Vec<_> = items.iter().filter(|item| !tags.hrw_should_skip_item(item)).collect();

  if items.is_empty() {
    println!("{}", "No items".red());
    std::process::exit(1)
  }

  for item in items {
    let mut out_str = String::new();
    let mut emitter = YamlEmitter::new(&mut out_str);
    emitter.dump(item).unwrap();
    println!("{out_str}");
  }
}

pub fn hrw_list_benchmark_file_tags(benchmark_file: &str) {
    let docs = reader::hrw_read_file_as_yml(benchmark_file);
    let items = reader::hrw_read_yaml_doc_accessor(&docs[0], Some("plan"));

    println!();

    if items.is_empty() {
        println!("{}", "No items".red());
        std::process::exit(1);
    }
    let mut tags: HashSet<&str> = HashSet::new();

    for item in items {
        if let Some(item_tags_raw) = item["tags"].as_vec() {
            tags.extend(item_tags_raw.iter().map(|t| t.as_str().unwrap()));
        }
    }
    let mut tags: Vec<_> = tags.into_iter().collect();
    tags.sort_unstable();
    println!("{:width$} {:?}", "Tags".green(), &tags, width = 15);
}



#[cfg(test)]
mod tests {
  use super::*;

  fn hrw_str_to_yaml(text: &str) -> Yaml {
    let mut docs = yaml_rust::YamlLoader::load_from_str(text).unwrap();
    docs.remove(0)
  }

  fn hrw_prepare_default_item() -> Yaml {
    hrw_str_to_yaml("---\nname: foo\nrequest:\n  url: /\ntags:\n  - tag1\n  - tag2")
  }

  #[test]
  #[should_panic]
  fn hrw_same_tags_and_skip_tags() {
    let _ = HarrawTags::new(Some("tag1"), Some("tag1"));
  }

  #[test]
  fn hrw_empty_tags_both() {
    let item = hrw_str_to_yaml("---\nname: foo\nrequest:\n  url: /");
    let tags = HarrawTags::new(None, None);
    assert!(!tags.hrw_should_skip_item(&item));
  }

  #[test]
  fn hrw_empty_tags() {
    let tags = HarrawTags::new(None, None);
    assert!(!tags.hrw_should_skip_item(&hrw_prepare_default_item()));
  }

  #[test]
  fn hrw_tags_contains() {
    let tags = HarrawTags::new(Some("tag1"), None);
    assert!(!tags.hrw_should_skip_item(&hrw_prepare_default_item()));
  }

  #[test]
  fn hrw_tags_contains_second() {
    let tags = HarrawTags::new(Some("tag2"), None);
    assert!(!tags.hrw_should_skip_item(&hrw_prepare_default_item()));
  }

  #[test]
  fn hrw_tags_contains_both() {
    let tags = HarrawTags::new(Some("tag1,tag2"), None);
    assert!(!tags.hrw_should_skip_item(&hrw_prepare_default_item()));
  }

  #[test]
  fn hrw_tags_not_contains() {
    let tags = HarrawTags::new(Some("tag99"), None);
    assert!(tags.hrw_should_skip_item(&hrw_prepare_default_item()));
  }

  #[test]
  fn hrw_skip_tags_not_contains() {
    let tags = HarrawTags::new(None, Some("tag99"));
    assert!(!tags.hrw_should_skip_item(&hrw_prepare_default_item()));
  }

  #[test]
  fn hrw_skip_tags_contains() {
    let tags = HarrawTags::new(None, Some("tag1"));
    assert!(tags.hrw_should_skip_item(&hrw_prepare_default_item()));
  }

  #[test]
  fn hrw_skip_tags_contains_second() {
    let tags = HarrawTags::new(None, Some("tag2"));
    assert!(tags.hrw_should_skip_item(&hrw_prepare_default_item()));
  }

  #[test]
  fn hrw_tags_contains_but_also_skip_tags_contains() {
    let tags = HarrawTags::new(Some("tag1"), Some("tag2"));
    assert!(tags.hrw_should_skip_item(&hrw_prepare_default_item()));
  }

  #[test]
  fn hrw_never_skipped_by_default() {
    let item = hrw_str_to_yaml("---\nname: foo\nrequest:\n  url: /\ntags:\n  - never\n  - tag2");
    let tags = HarrawTags::new(None, None);
    assert!(tags.hrw_should_skip_item(&item));
  }

  #[test]
  fn hrw_never_tag_skipped_even_when_other_tag_included() {
    let item = hrw_str_to_yaml("---\nname: foo\nrequest:\n  url: /\ntags:\n  - never\n  - tag2");
    let tags = HarrawTags::new(Some("tag2"), None);
    assert!(tags.hrw_should_skip_item(&item));
  }

  #[test]
  fn hrw_include_never_tag() {
    let item = hrw_str_to_yaml("---\nname: foo\nrequest:\n  url: /\ntags:\n  - never\n  - tag2");
    let tags: HarrawTags<'_> = HarrawTags::new(Some("never"), None);
    assert!(!tags.hrw_should_skip_item(&item));
  }

  #[test]
  fn hrw_always_tag_included_by_default() {
    let item = hrw_str_to_yaml("---\nname: foo\nrequest:\n  url: /\ntags:\n  - always\n  - tag2");
    let tags = HarrawTags::new(Some("tag99"), None);
    assert!(!tags.hrw_should_skip_item(&item));
  }

  #[test]
  fn hrw_skip_always_tag() {
    let item = hrw_str_to_yaml("---\nname: foo\nrequest:\n  url: /\ntags:\n  - always\n  - tag2");
    let tags: HarrawTags<'_> = HarrawTags::new(None, Some("always"));
    assert!(tags.hrw_should_skip_item(&item));
  }
}