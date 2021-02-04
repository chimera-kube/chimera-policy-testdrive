use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;

use serde_json::json;

use crate::validation_response::ValidationResponse;

use wapc::WapcHost;
use wasmtime_provider::WasmtimeEngineProvider;

fn host_callback(
    id: u64,
    bd: &str,
    ns: &str,
    op: &str,
    payload: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    println!(
        "Guest {} invoked '{}->{}:{}' with payload of {}",
        id,
        bd,
        ns,
        op,
        ::std::str::from_utf8(payload).unwrap()
    );
    Ok(b"Host result".to_vec())
}

pub(crate) struct PolicyEvaluator {
    wapc_host: WapcHost,
    settings: serde_json::Value,
}

impl PolicyEvaluator {
    pub(crate) fn new(wasm_file: String, settings: serde_json::Value) -> Result<PolicyEvaluator> {
        let mut f = File::open(&wasm_file)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;

        let engine = WasmtimeEngineProvider::new(&buf, None);
        let host = WapcHost::new(Box::new(engine), host_callback)?;

        Ok(PolicyEvaluator {
            wapc_host: host,
            settings: settings,
        })
    }

    pub(crate) fn validate(&mut self, request: serde_json::Value) -> ValidationResponse {
        let validate_params = json!({
            "request": request,
            "settings": self.settings,
        });
        let validate_str = match serde_json::to_string(&validate_params) {
            Ok(s) => s,
            Err(e) => {
                println!("Cannot serialize validation params: {}", e);
                return ValidationResponse {
                    accepted: false,
                    message: Some(String::from("internal server error")),
                    code: Some(500),
                };
            }
        };

        match self.wapc_host.call("validate", validate_str.as_bytes()) {
            Ok(res) => {
                let val_resp: ValidationResponse = serde_json::from_slice(&res)
                    .map_err(|e| {
                        //TODO: proper logging
                        println!("Cannot deserialize response: {}", e);
                        ValidationResponse {
                            accepted: false,
                            message: Some(String::from("internal server error")),
                            code: Some(500),
                        }
                    })
                    .unwrap();
                val_resp
            }
            Err(e) => {
                //TODO: proper logging
                println!("Something went wrong with waPC: {}", e);
                ValidationResponse {
                    accepted: false,
                    message: Some(String::from("internal server error")),
                    code: Some(500),
                }
            }
        }
    }
}
