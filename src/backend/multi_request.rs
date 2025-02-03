use rand::seq::SliceRandom;
use rand::thread_rng;
use yaml_rust::Yaml;

use super::hrw_pick;
use crate::service::request::HarrawRequest;
use crate::benchmark::Benchmark;
use crate::interpolator::INTERPOLATION_REGEX;

pub fn hrw_is_that_you(item: &Yaml) -> bool {
  item["request"].as_hash().is_some() && item["with_items"].as_vec().is_some()
}

pub fn hrw_expand(item: &Yaml, benchmark: &mut Benchmark) {
  if let Some(with_items) = item["with_items"].as_vec() {
    let mut with_items_list = with_items.clone();

    if let Some(shuffle) = item["shuffle"].as_bool() {
      if shuffle {
        let mut rng = thread_rng();
        with_items_list.shuffle(&mut rng);
      }
    }

    let pick = hrw_pick(item, &with_items_list);
    for (index, with_item) in with_items_list.iter().take(pick).enumerate() {
      let index = index as u32;

      let value: &str = with_item.as_str().unwrap_or("");

      if INTERPOLATION_REGEX.is_match(value) {
        panic!("Interpolations not supported in 'with_items' children!");
      }

      benchmark.push(Box::new(HarrawRequest::new(item, Some(with_item.clone()), Some(index))));
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hrw_expand_multi() {
    let text = "---\nname: foobar\nrequest:\n  url: /api/{{ item }}\nwith_items:\n  - 1\n  - 2\n  - 3";
    let docs = yaml_rust::YamlLoader::load_from_str(text).unwrap();
    let doc = &docs[0];
    let mut benchmark: Benchmark = Benchmark::new();

    hrw_expand(doc, &mut benchmark);

    assert!(hrw_is_that_you(doc));
    assert_eq!(benchmark.len(), 3);
  }

  #[test]
  fn hrw_expand_multi_should_limit_requests_using_the_pick_option() {
    let text = "---\nname: foobar\nrequest:\n  url: /api/{{ item }}\npick: 2\nwith_items:\n  - 1\n  - 2\n  - 3";
    let docs = yaml_rust::YamlLoader::load_from_str(text).unwrap();
    let doc = &docs[0];
    let mut benchmark: Benchmark = Benchmark::new();

    hrw_expand(doc, &mut benchmark);

    assert!(hrw_is_that_you(doc));
    assert_eq!(benchmark.len(), 2);
  }

  #[test]
  fn hrw_expand_multi_should_work_with_pick_and_shuffle() {
    let text = "---\nname: foobar\nrequest:\n  url: /api/{{ item }}\npick: 1\nshuffle: true\nwith_items:\n  - 1\n  - 2\n  - 3";
    let docs = yaml_rust::YamlLoader::load_from_str(text).unwrap();
    let doc = &docs[0];
    let mut benchmark: Benchmark = Benchmark::new();

    hrw_expand(doc, &mut benchmark);

    assert!(hrw_is_that_you(doc));
    assert_eq!(benchmark.len(), 1);
  }

  #[test]
  #[should_panic]
  fn hrw_runtime_expand() {
    let text = "---\nname: foobar\nrequest:\n  url: /api/{{ item }}\nwith_items:\n  - 1\n  - 2\n  - foo{{ memory }}";
    let docs = yaml_rust::YamlLoader::load_from_str(text).unwrap();
    let doc = &docs[0];
    let mut benchmark: Benchmark = Benchmark::new();

    hrw_expand(doc, &mut benchmark);
  }
}