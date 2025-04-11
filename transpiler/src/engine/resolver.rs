use rhai::{module_resolvers::StaticModuleResolver, Engine, Module, Scope};
use std::collections::HashMap;

use crate::transpiler::transpile;

pub fn register_module_resolver(engine: &mut Engine, modules: HashMap<String, String>) {
    let mut resolver = StaticModuleResolver::new();
    
    for (name, code) in modules {
        let script = transpile(code);
        let ast = engine.compile(script.as_str()).unwrap();
        let module = Module::eval_ast_as_new(Scope::new(), &ast, &engine).unwrap();
        resolver.insert(&name, module);
        println!("Loaded module: {}", name);
    }

    engine.set_module_resolver(resolver);
}
