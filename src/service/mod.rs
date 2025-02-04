pub mod assert;
pub mod assign;
pub mod delay;
pub mod exec;
pub mod request;

use async_trait::async_trait;
use yaml_rust::Yaml;



use crate::benchmark::{Context, Pool, Reports};
use crate::config::HarrawConfig;

use std::fmt;



#[async_trait]
pub trait HarrawRunnable {
  async fn hrw_execute(&self, context: &mut Context, reports: &mut Reports, pool: &Pool, config: &HarrawConfig);
}

#[derive(Clone)]
pub struct HarrawReport {
  pub name: String,
  pub duration: f64,
  pub status: u16,
}

impl fmt::Debug for HarrawReport {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "\n- name: {}\n  duration: {}\n", self.name, self.duration)
  }
}

impl fmt::Display for HarrawReport {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "\n- name: {}\n  duration: {}\n  status: {}\n", self.name, self.duration, self.status)
  }
}

pub fn hrw_extract_optional<'a>(item: &'a Yaml, attr: &'a str) -> Option<String> {
  if let Some(s) = item[attr].as_str() {
    Some(s.to_string())
  } else if item[attr].as_hash().is_some() {
    panic!("`{}` needs to be a string. Try adding quotes", attr);
  } else {
    None
  }
}

pub fn hrw_extract<'a>(item: &'a Yaml, attr: &'a str) -> String {
  if let Some(s) = item[attr].as_i64() {
    s.to_string()
  } else if let Some(s) = item[attr].as_str() {
    s.to_string()
  } else if item[attr].as_hash().is_some() {
    panic!("`{}` is required needs to be a string. Try adding quotes", attr);
  } else {
    panic!("Unknown node `{}` => {:?}", attr, item[attr]);
  }
}