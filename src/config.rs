use yaml_rust::{Yaml, YamlLoader};

use crate::benchmark::Context;
use crate::interpolator;
use crate::reader;


const NITERATIONS: i64 = 1;
const NRAMPUP: i64 = 0;


pub struct HarrawConfig{
    pub base: String,
    pub concurrency: i64,
    pub iterations: i64,
    pub relaxed_interpolations: bool,
    pub no_check_certificate: bool,
    pub rampup: i64,
    pub quiet: bool,
    pub nanosec: bool,
    pub timeout: u64,
    pub verbose: bool,
}


impl HarrawConfig {
    pub fn new(path: &str, relaxed_interpolations: bool, no_check_certificate: bool, quiet: bool, nanosec: bool, timeout: u64, verbose: bool) -> HarrawConfig {
        let config_file = reader::hrw_read_file(path);
        let config_docs = YamlLoader::load_from_str(config_file.as_str()).unwrap();
        let config_doc = &config_docs[0];
        let context: Context = Context::new();
        let interpolator = interpolator::HarrawInterpolator::new(&context);
        let iterations = hrw_read_i64_configuration(config_doc, &interpolator, "iterations", NITERATIONS);
        let concurrency = hrw_read_i64_configuration(config_doc, &interpolator, "concurrency", iterations);
        let rampup = hrw_read_i64_configuration(config_doc, &interpolator, "rampup", NRAMPUP);
        let base = hrw_read_str_configuration(config_doc, &interpolator, "base", "");

        HarrawConfig { base,concurrency,iterations,relaxed_interpolations,no_check_certificate,rampup,quiet,nanosec,timeout,verbose }
    }
}


fn hrw_read_str_configuration(config_doc: &Yaml, interpolator: &interpolator::HarrawInterpolator, name: &str, default: &str) -> String {
    match config_doc[name].as_str() {
        Some(value) => {
            if value.contains("{") {
                interpolator.hrw_resolve(value, true)
            } else {
                value.to_owned()
            }
        }
        None => {
            if config_doc[name].as_str().is_some() {
                println!("Invalid {name} value!");
            }
            default.to_owned()
        }
    }
}

fn hrw_read_i64_configuration(config_doc: &Yaml, interpolator: &interpolator::HarrawInterpolator, name: &str, default: i64) -> i64 {
    let value = if let Some(value) = config_doc[name].as_i64() {
        Some(value)
    } else if let Some(key) = config_doc[name].as_str() {
        interpolator.hrw_resolve(key, false).parse::<i64>().ok()
    } else {
        None
    };

    match value {
        Some(value) => {
          if value < 0 {
            println!("Invalid negative {name} value!");
    
            default
          } else {
            value
          }
        }
        None => {
          if config_doc[name].as_str().is_some() {
            println!("Invalid {name} value!");
          }
    
          default
        }
      }
}   