use rhai::{module_resolvers::StaticModuleResolver, Engine, Module, Scope};

use std::fs;

use crate::transpiler::transpile;

pub fn register_module_resolver(engine: &mut Engine) {
    let current_dir = std::env::current_dir().unwrap();
    let dir_name = "plonk_lib";
    let lib_dir = current_dir.join(dir_name);

    if !lib_dir.exists() {
        panic!("The '{dir_name}' directory does not exist in the current directory.");
    }

    let mut resolver = StaticModuleResolver::new();
    for entry in fs::read_dir(lib_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("plonk") {
            let code = fs::read_to_string(path.clone()).expect("cannot read library");
            let script = transpile(code);
            let ast = engine.compile(script.as_str()).unwrap();
            let module = Module::eval_ast_as_new(Scope::new(), &ast, &engine).unwrap();
            let name = path.file_stem().unwrap().to_str().unwrap();
            resolver.insert(name, module);
            println!("Loaded module: {}", name);
        }
    }

    engine.set_module_resolver(resolver);
}
