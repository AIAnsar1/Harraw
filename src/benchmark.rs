use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use futures::stream::{self, StreamExt};
use serde_json::{json, Map, Value};
use tokio::{runtime, time::sleep};
use reqwest::Client;
use colored::*;

use crate::service::{HarrawReport, HarrawRunnable};
use crate::config::HarrawConfig;
use crate::backend::include;
use crate::tags::HarrawTags;
use crate::writer;


pub type Benchmark = Vec<Box<(dyn HarrawRunnable + Sync + Send)>>;
pub type Context = Map<String, Value>;
pub type Reports = Vec<HarrawReport>;
pub type PoolStore = HashMap<String, Client>;
pub type Pool = Arc<Mutex<PoolStore>>;

pub struct HarrawBenchmarkResult {
  pub reports: Vec<Reports>,
  pub duration: f64,
}



async fn hrw_run_iteration(benchmark: Arc<Benchmark>, pool: Pool, config: Arc<HarrawConfig>, iterations: i64) -> Vec<HarrawReport> {
    if config.rampup > 0 {
        let delay = config.rampup / config.iterations;
        sleep(Duration::new((delay * iterations) as u64, 0)).await;
      }
    
      let mut context: Context = Context::new();
      let mut reports: Vec<HarrawReport> = Vec::new();
    
      context.insert("iterations".to_string(), json!(iterations.to_string()));
      context.insert("base".to_string(), json!(config.base.to_string()));
    
      for item in benchmark.iter() {
        item.hrw_execute(&mut context, &mut reports, &pool, &config).await;
      }
      reports
}   


fn hrw_join<S: ToString>(l: Vec<S>, sep: &str) -> String {
    l.iter().fold("".to_string(), |a,b| if !a.is_empty() {a+sep} else {a} + &b.to_string())
}


#[allow(clippy::too_many_arguments)]
pub fn hrw_execute(benchmark_path: &str, report_path_option: Option<&str>, relaxed_interpolations: bool, no_check_certificate: bool, quiet: bool, nanosec: bool, timeout: Option<&str>, verbose: bool, tags: &HarrawTags) -> HarrawBenchmarkResult {
    let config = Arc::new(HarrawConfig::new(benchmark_path, relaxed_interpolations, no_check_certificate, quiet, nanosec, timeout.map_or(10, |t| t.parse().unwrap_or(10)), verbose));

    if report_path_option.is_some() {
        println!("{}: {}. Ignoring {} and {} properties...", "Report mode".yellow(), "on".purple(), "concurrency".yellow(), "iterations".yellow());
    } else {
        println!("{} {}", "Concurrency".yellow(), config.concurrency.to_string().purple());
        println!("{} {}", "Iterations".yellow(), config.iterations.to_string().purple());
        println!("{} {}", "Rampup".yellow(), config.rampup.to_string().purple());
    }
    println!("{} {}", "Base URL".yellow(), config.base.purple());
    println!();
    let threads = std::cmp::min(num_cpus::get(), config.concurrency as usize);
    let rt = runtime::Builder::new_multi_thread().enable_all().worker_threads(threads).build().unwrap();
    rt.block_on(async  {
        let mut benchmark: Benchmark = Benchmark::new();
        let pool_store: PoolStore = PoolStore::new();
        include::hrw_expand_from_filepath(benchmark_path, &mut benchmark, Some("plan"), tags);

        if benchmark.is_empty() {
            eprintln!("Empty benchmark. Exiting.");
            std::process::exit(1);
        }
        let benchmark = Arc::new(benchmark);
        let pool = Arc::new(Mutex::new(pool_store));

        if let Some(report_path) = report_path_option {
            let reports = hrw_run_iteration(benchmark.clone(), pool.clone(), config, 0).await;
            writer::hrw_write_file(report_path, hrw_join(reports, ""));
            HarrawBenchmarkResult {
                reports: vec![],
                duration: 0.0,
            }
        } else {
            let children = (0..config.iterations).map(|iteration| hrw_run_iteration(benchmark.clone(), pool.clone(), config.clone(), iteration));
            let buffered = stream::iter(children).buffer_unordered(config.concurrency as usize);
            let begin = Instant::now();
            let reports: Vec<Vec<HarrawReport>> = buffered.collect::<Vec<_>>().await;
            let duration = begin.elapsed().as_secs_f64();

            HarrawBenchmarkResult { reports, duration }
        }
    })
}