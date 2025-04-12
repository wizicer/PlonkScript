use std::fs;
use std::collections::HashMap;

use rhai::EvalAltResult;
use transpiler::try_run;

fn resolve_lib_modules() -> HashMap<String, String> {
    let current_dir = std::env::current_dir().unwrap();
    let dir_name = "plonk/lib";
    let lib_dir = current_dir.join(dir_name);

    if !lib_dir.exists() {
        panic!("The '{dir_name}' directory does not exist in the current directory.");
    }
    
    let mut modules = HashMap::new();
    for entry in fs::read_dir(lib_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("plonk") {
            let code = fs::read_to_string(path.clone()).expect("cannot read library");
            let name = path.file_stem().unwrap().to_str().unwrap().to_string();
            modules.insert(name, code);
        }
    }
    modules
}

#[allow(unreachable_code)]
pub fn main() -> Result<(), Box<EvalAltResult>> {
    let modules = resolve_lib_modules();
    let code = fs::read_to_string("plonk/src/simple_demo.plonk").expect("read file failed");
    let output = try_run(code, modules);
    match output {
        Ok(_) => {
            println!("Done");
        }
        Err(e) => {
            println!("Script Error: {:#?}", e);
        }
    }
    Ok(())
}
