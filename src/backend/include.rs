use std::path::Path;
use yaml_rust::{Yaml, YamlEmitter};

use crate::interpolator::INTERPOLATION_REGEX;

use crate::service;
use crate::benchmark::Benchmark;
use crate::backend::{include, multi_csv_request, multi_file_request, multi_iter_request, multi_request};
use crate::tags::HarrawTags;

use crate::reader;

pub fn hrw_is_that_you(item: &Yaml) -> bool {
  item["include"].as_str().is_some()
}

pub fn hrw_expand(parent_path: &str, item: &Yaml, benchmark: &mut Benchmark, tags: &HarrawTags) {
  let include_path = item["include"].as_str().unwrap();

  if INTERPOLATION_REGEX.is_match(include_path) {
    panic!("Interpolations not supported in 'include' property!");
  }

  let include_filepath = Path::new(parent_path).with_file_name(include_path);
  let final_path = include_filepath.to_str().unwrap();

  hrw_expand_from_filepath(final_path, benchmark, None, tags);
}

pub fn hrw_expand_from_filepath(parent_path: &str, benchmark: &mut Benchmark, accessor: Option<&str>, tags: &HarrawTags) {
  let docs = reader::hrw_read_file_as_yml(parent_path);
  let items = reader::hrw_read_yaml_doc_accessor(&docs[0], accessor);

  for item in items {
    if include::hrw_is_that_you(item) {
      include::hrw_expand(parent_path, item, benchmark, tags);

      continue;
    }

    if tags.hrw_should_skip_item(item) {
      continue;
    }
    if multi_request::hrw_is_that_you(item) {
      multi_request::hrw_expand(item, benchmark);
    } else if multi_iter_request::hrw_is_that_you(item) {
      multi_iter_request::hrw_expand(item, benchmark);
    } else if multi_csv_request::hrw_is_that_you(item) {
      multi_csv_request::hrw_expand(parent_path, item, benchmark);
    } else if multi_file_request::hrw_is_that_you(item) {
      multi_file_request::hrw_expand(parent_path, item, benchmark);
    } else if service::delay::HarrawDelay::hrw_is_that_you(item) {
      benchmark.push(Box::new(service::delay::HarrawDelay::new(item, None)));
    } else if service::exec::HarrawExec::hrw_is_that_you(item) {
      benchmark.push(Box::new(service::exec::HarrawExec::new(item, None)));
    } else if service::assign::HarrawAssign::hrw_is_that_you(item) {
      benchmark.push(Box::new(service::assign::HarrawAssign::new(item, None)));
    } else if service::assert::HarrawAssert::hrw_is_that_you(item) {
      benchmark.push(Box::new(service::assert::HarrawAssert::new(item, None)));
    } else if service::request::HarrawRequest::hrw_is_that_you(item) {
      benchmark.push(Box::new(service::request::HarrawRequest::new(item, None, None)));
    } else {
      let mut out_str = String::new();
      let mut emitter = YamlEmitter::new(&mut out_str);
      emitter.dump(item).unwrap();
      panic!("Unknown node:\n\n{}\n\n", out_str);
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::benchmark::Benchmark;
  use crate::backend::include::{hrw_expand, hrw_is_that_you};
  use crate::tags::HarrawTags;

  #[test]
  fn hrw_expand_include() {
    let text = "---\nname: Include comment\ninclude: comments.yml";
    let docs = yaml_rust::YamlLoader::load_from_str(text).unwrap();
    let doc = &docs[0];
    let mut benchmark: Benchmark = Benchmark::new();

    hrw_expand("example/benchmark.yml", doc, &mut benchmark, &HarrawTags::new(None, None));

    assert!(hrw_is_that_you(doc));
    assert_eq!(benchmark.len(), 2);
  }

  #[test]
  #[should_panic]
  fn hrw_invalid_expand() {
    let text = "---\nname: Include comment\ninclude: {{ memory }}.yml";
    let docs = yaml_rust::YamlLoader::load_from_str(text).unwrap();
    let doc = &docs[0];
    let mut benchmark: Benchmark = Benchmark::new();

    hrw_expand("example/benchmark.yml", doc, &mut benchmark, &HarrawTags::new(None, None));
  }
}