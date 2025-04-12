#![allow(unused_variables)]

use serde::{Deserialize, Serialize};
use std::panic;

#[derive(Serialize, Deserialize)]
pub struct TryRunRequest {
    pub code: String,
    #[serde(default)]
    pub modules: std::collections::HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct TryRunResult {
    pub prover_result: String,
    pub transpiled_script: String,
    pub context_debug: String,
}

#[allow(dead_code)]
fn main() {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(start)]
    pub fn initialization() {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
    }

    #[wasm_bindgen]
    extern "C" {
        fn alert(s: &str);

        // Use `js_namespace` here to bind `console.log(..)` instead of just
        // `log(..)`
        #[wasm_bindgen(js_namespace = console)]
        fn log(s: &str);
    }

    #[wasm_bindgen]
    pub fn greet(name: &str) {
        log(&format!("Hello log, {}!", name));
        // alert(&format!("Hello alert, {}!", name));
    }

    #[wasm_bindgen]
    pub fn try_run(request: JsValue) -> Result<JsValue, JsValue> {
        let req: TryRunRequest = serde_wasm_bindgen::from_value(request)?;
        // log(&format!("try_run!"));
        
        match transpiler::try_run(req.code, req.modules) {
            Ok(result) => Ok(serde_wasm_bindgen::to_value(&TryRunResult {
                prover_result: result.prover_result,
                transpiled_script: result.transpiled_script,
                context_debug: result.context_debug,
            }).unwrap()),
            Err(d) => Err(JsValue::from_str(d.to_string().as_str())),
        }
    }
}
