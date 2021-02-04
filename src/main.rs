mod cli;

mod policy_evaluator;
use policy_evaluator::PolicyEvaluator;

mod validation_response;

use anyhow::Result;
use std::process;

use std::fs::File;
use std::io::BufReader;

fn main() {
    let matches = cli::app().get_matches();

    let policy_file = String::from(matches.value_of("policy").unwrap());
    let request_file = matches.value_of("request-file").unwrap();
    let settings_str = matches.value_of("settings").unwrap();

    let settings = match serde_json::from_str(&settings_str) {
        Ok(s) => s,
        Err(e) => {
            return fatal_error(format!("Error parsing settings: {:?}", e));
        }
    };

    let mut policy_evaluator = match PolicyEvaluator::new(policy_file, settings) {
        Ok(p) => p,
        Err(e) => {
            return fatal_error(format!("Error creating policy evaluator: {:?}", e));
        }
    };

    let request = match read_request_file(request_file) {
        Ok(r) => r,
        Err(e) => {
            return fatal_error(format!(
                "Error reading request from file {}: {:?}",
                request_file, e
            ));
        }
    };

    let vr = policy_evaluator.validate(request);
    println!("{:?}", vr);
}

fn read_request_file(path: &str) -> Result<serde_json::Value> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let v = serde_json::from_reader(reader)?;

    Ok(v)
}

fn fatal_error(msg: String) {
    println!("{}", msg);
    process::exit(1);
}
