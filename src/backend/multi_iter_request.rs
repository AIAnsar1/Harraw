use std::convert::TryInto;

use rand::seq::SliceRandom;
use rand::thread_rng;
use yaml_rust::Yaml;

use crate::interpolator::INTERPOLATION_REGEX;

use crate::service::request::HarrawRequest;
use crate::benchmark::Benchmark;

pub fn hrw_is_that_you(item: &Yaml) -> bool {
  item["request"].as_hash().is_some() && item["with_items_range"].as_hash().is_some()
}

pub fn hrw_expand(item: &Yaml, benchmark: &mut Benchmark) {
  if let Some(with_iter_items) = item["with_items_range"].as_hash() {
    let init = Yaml::Integer(1);
    let lstart = Yaml::String("start".into());
    let lstep = Yaml::String("step".into());
    let lstop = Yaml::String("stop".into());
    let vstart: &Yaml = with_iter_items.get(&lstart).expect("Start property is mandatory");
    let vstep: &Yaml = with_iter_items.get(&lstep).unwrap_or(&init);
    let vstop: &Yaml = with_iter_items.get(&lstop).expect("Stop property is mandatory");
    let start: &str = vstart.as_str().unwrap_or("");
    let step: &str = vstep.as_str().unwrap_or("");
    let stop: &str = vstop.as_str().unwrap_or("");

    if INTERPOLATION_REGEX.is_match(start) {
      panic!("Interpolations not supported in 'start' property!");
    }

    if INTERPOLATION_REGEX.is_match(step) {
      panic!("Interpolations not supported in 'step' property!");
    }

    if INTERPOLATION_REGEX.is_match(stop) {
      panic!("Interpolations not supported in 'stop' property!");
    }

    let start: i64 = vstart.as_i64().expect("Start needs to be a number");
    let step: i64 = vstep.as_i64().expect("Step needs to be a number");
    let stop: i64 = vstop.as_i64().expect("Stop needs to be a number");

    let stop = stop + 1; // making stop inclusive

    if stop > start && start > 0 {
      let mut with_items: Vec<i64> = (start..stop).step_by(step as usize).collect();

      if let Some(shuffle) = item["shuffle"].as_bool() {
        if shuffle {
          let mut rng = thread_rng();
          with_items.shuffle(&mut rng);
        }
      }

      if let Some(pick) = item["pick"].as_i64() {
        with_items.truncate(pick.try_into().expect("pick can't be larger than size of range"))
      }

      for (index, value) in with_items.iter().enumerate() {
        let index = index as u32;

        benchmark.push(Box::new(HarrawRequest::new(item, Some(Yaml::Integer(*value)), Some(index))));
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn expand_multi_range() {
    let text = "---\nname: foobar\nrequest:\n  url: /api/{{ item }}\nwith_items_range:\n  start: 2\n  step: 2\n  stop: 20";
    let docs = yaml_rust::YamlLoader::load_from_str(text).unwrap();
    let doc = &docs[0];
    let mut benchmark: Benchmark = Benchmark::new();

    hrw_expand(doc, &mut benchmark);

    assert!(hrw_is_that_you(doc));
    assert_eq!(benchmark.len(), 10);
  }

  #[test]
  fn hrw_expand_multi_range_should_limit_requests_using_the_pick_option() {
    let text = "---\nname: foobar\nrequest:\n  url: /api/{{ item }}\npick: 3\nwith_items_range:\n  start: 2\n  step: 2\n  stop: 20";
    let docs = yaml_rust::YamlLoader::load_from_str(text).unwrap();
    let doc = &docs[0];
    let mut benchmark: Benchmark = Benchmark::new();

    hrw_expand(doc, &mut benchmark);

    assert!(hrw_is_that_you(doc));
    assert_eq!(benchmark.len(), 3);
  }

  #[test]
  #[should_panic]
  fn hrw_invalid_expand() {
    let text = "---\nname: foobar\nrequest:\n  url: /api/{{ item }}\nwith_items_range:\n  start: 1\n  step: 2\n  stop: foo";
    let docs = yaml_rust::YamlLoader::load_from_str(text).unwrap();
    let doc = &docs[0];
    let mut benchmark: Benchmark = Benchmark::new();

    hrw_expand(doc, &mut benchmark);
  }

  #[test]
  #[should_panic]
  fn hrw_runtime_expand() {
    let text = "---\nname: foobar\nrequest:\n  url: /api/{{ item }}\nwith_items_range:\n  start: 1\n  step: 2\n  stop: \"{{ memory }}\"";
    let docs = yaml_rust::YamlLoader::load_from_str(text).unwrap();
    let doc = &docs[0];
    let mut benchmark: Benchmark = Benchmark::new();

    hrw_expand(doc, &mut benchmark);
  }
}