use async_trait::async_trait;
use colored::*;
use serde_json::json;
use std::process::Command;
use yaml_rust::Yaml;


use crate::interpolator;
use crate::config::*;
use crate::benchmark::{Context, Pool, Reports};
use crate::service::HarrawRunnable;

use super::hrw_extract;
use super::hrw_extract_optional;


#[derive(Clone)]
pub struct HarrawExec {
    name: String,
    command: String,
    pub assign: Option<String>
}

impl HarrawExec {
    pub fn hrw_is_that_you(item: &Yaml) -> bool {
        item["exec"].as_hash().is_some()
    }

    pub fn new(item: &Yaml, _with_item: Option<Yaml>) -> HarrawExec {
        let name = hrw_extract(item, "name");
        let command = hrw_extract(&item["exec"], "command");
        let assign = hrw_extract_optional(item, "assign");
    
        HarrawExec { name, command, assign, }
      }
}


#[async_trait]
impl HarrawRunnable for HarrawExec {
  async fn hrw_execute(&self, context: &mut Context, _reports: &mut Reports, _pool: &Pool, config: &HarrawConfig) {
    if !config.quiet {
      println!("{:width$} {}", self.name.green(), self.command.cyan().bold(), width = 25);
    }

    let final_command = interpolator::HarrawInterpolator::new(context).hrw_resolve(&self.command, !config.relaxed_interpolations);

    let args = vec!["bash", "-c", "--", final_command.as_str()];

    let execution = Command::new(args[0]).args(&args[1..]).output().expect("Couldn't run it");

    let output: String = String::from_utf8_lossy(&execution.stdout).into();
    let output = output.trim_end().to_string();

    if let Some(ref key) = self.assign {
      context.insert(key.to_owned(), json!(output));
    }
  }
}