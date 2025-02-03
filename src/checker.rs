use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use colored::*;
use yaml_rust::YamlLoader;

use crate::service::HarrawReport;



pub fn hrw_compare(list_reports: &[Vec<HarrawReport>], filepath: &str, threshold: &str) -> Result<(), i32> {
    let threshold_value = match threshold.parse::<f64>() {
        Ok(v) => v,
        _ => panic!("arrrgh"),
    };
    let path = Path::new(filepath);
    let display = path.display();
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let mut content = String::new();

    if let Err(why) = file.read_to_string(&mut content) {
        panic!("couldn't read {}: {}", display, why);
    }
    let docs = YamlLoader::load_from_str(content.as_str()).unwrap();
    let doc = &docs[0];
    let items = doc.as_vec().unwrap();
    let mut slow_counter = 0;
    println!();

    for report in list_reports {
        for (i, report_item) in report.iter().enumerate() {
            let recorded_duration = items[i]["duration"].as_f64().unwrap();
            let delta_ms = report_item.duration - recorded_duration;

            if delta_ms > threshold_value {
                println!("{:width$} is {}{} slower than before", report_item.name.green(), delta_ms.round().to_string().red(), "ms".red(), width = 25);
                slow_counter += 1;
            }
        }
    }

    if slow_counter == 0 {
        Ok(())
    } else {
        Err(slow_counter)
    }
}