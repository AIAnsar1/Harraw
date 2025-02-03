use async_trait::async_trait;
use colored::*;
use serde_json::json;
use yaml_rust::Yaml;

use crate::service::hrw_extract;
use crate::service::HarrawRunnable;
use crate::benchmark::{Context, Pool, Reports};
use crate::config::HarrawConfig;

#[derive(Clone)]
pub struct HarrawAssign {
  name: String,
  key: String,
  value: String,
}

impl HarrawAssign {
  pub fn hrw_is_that_you(item: &Yaml) -> bool {
    item["assign"].as_hash().is_some()
  }

  pub fn new(item: &Yaml, _with_item: Option<Yaml>) -> HarrawAssign {
    let name = hrw_extract(item, "name");
    let key = hrw_extract(&item["assign"], "key");
    let value = hrw_extract(&item["assign"], "value");

    HarrawAssign { name, key, value }
  }
}

#[async_trait]
impl HarrawRunnable for HarrawAssign {
  async fn hrw_execute(&self, context: &mut Context, _reports: &mut Reports, _pool: &Pool, config: &HarrawConfig) {
    if !config.quiet {
      println!("{:width$} {}={}", self.name.green(), self.key.cyan().bold(), self.value.magenta(), width = 25);
    }
    context.insert(self.key.to_owned(), json!(self.value.to_owned()));
  }
}