use async_trait::async_trait;
use colored::*;
use tokio::time::sleep;
use yaml_rust::Yaml;

use crate::service::hrw_extract;
use crate::service::HarrawRunnable;
use crate::benchmark::{Context, Pool, Reports};
use crate::config::HarrawConfig;

use std::convert::TryFrom;
use std::time::Duration;

#[derive(Clone)]
pub struct HarrawDelay {
  name: String,
  seconds: u64,
}


impl HarrawDelay {
    pub fn hrw_is_that_you(item: &Yaml) -> bool {
      item["delay"].as_hash().is_some()
    }
  
    pub fn new(item: &Yaml, _with_item: Option<Yaml>) -> HarrawDelay {
      let name = hrw_extract(item, "name");
      let seconds = u64::try_from(item["delay"]["seconds"].as_i64().unwrap()).expect("Invalid number of seconds");
  
      HarrawDelay {
        name,
        seconds,
      }
    }
  }
  
  #[async_trait]
  impl HarrawRunnable for HarrawDelay {
    async fn hrw_execute(&self, _context: &mut Context, _reports: &mut Reports, _pool: &Pool, config: &HarrawConfig) {
      sleep(Duration::from_secs(self.seconds)).await;
  
      if !config.quiet {
        println!("{:width$} {}{}", self.name.green(), self.seconds.to_string().cyan().bold(), "s".magenta(), width = 25);
      }
    }
  }