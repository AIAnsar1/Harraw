use async_trait::async_trait;
use colored::*;
use serde_json::json;
use yaml_rust::Yaml;

use crate::service::hrw_extract;
use crate::service::HarrawRunnable;
use crate::benchmark::{Context, Pool, Reports};
use crate::config::HarrawConfig;
use crate::interpolator;

#[derive(Clone)]
pub struct HarrawAssert {
  name: String,
  key: String,
  value: String,
}

impl HarrawAssert {
  pub fn hrw_is_that_you(item: &Yaml) -> bool {
    item["assert"].as_hash().is_some()
  }

  pub fn new(item: &Yaml, _with_item: Option<Yaml>) -> HarrawAssert {
    let name = hrw_extract(item, "name");
    let key = hrw_extract(&item["assert"], "key");
    let value = hrw_extract(&item["assert"], "value");

    HarrawAssert { name, key, value }
  }
}

#[async_trait]
impl HarrawRunnable for HarrawAssert {
  async fn hrw_execute(&self, context: &mut Context, _reports: &mut Reports, _pool: &Pool, config: &HarrawConfig) {
    if !config.quiet {
      println!("{:width$} {}={}?", self.name.green(), self.key.cyan().bold(), self.value.magenta(), width = 25);
    }
    let interpolator = interpolator::HarrawInterpolator::new(context);
    let eval = format!("{{{{ {} }}}}", &self.key);
    let stored = interpolator.hrw_resolve(&eval, true);
    let assertion = json!(self.value.to_owned());

    if !stored.eq(&assertion) {
      panic!("Assertion mismatched: {} != {}", stored, assertion);
    }
  }
}