use crate::{system::*, CONTEXT};

pub mod bind;
pub mod custom_type;
pub mod gate;
pub mod io;
pub mod operator;
pub mod resolver;

use self::{
    bind::register_bind, custom_type::register_custom_type, io::register_io,
    operator::register_operator, resolver::register_module_resolver,
};
pub use io::DEFAULT_INSTANCE_COLUMN_NAME;

pub trait EngineExt {
    fn register_plonk_script(&mut self);
}

impl EngineExt for rhai::Engine {
    #[warn(unused_must_use)]
    fn register_plonk_script(&mut self) {
        // when expression is complex, may occur ExprTooDeep error
        self.set_max_expr_depths(320, 320);

        let _ = &mut self
        .register_fn("define_region", define_region)
        // .register_indexer_set(TestStruct::set_field)
        ;
        register_io(self);
        register_bind(self);
        register_custom_type(self);
        register_operator(self);

        register_module_resolver(self);

        define_region("default".to_string());
    }
}

fn define_region(v: String) {
    // println!("define_region({})", v);
    unsafe {
        CONTEXT.regions.push(Region {
            name: v,
            id: CONTEXT.regions.len() as i64,
            instructions: vec![],
        });
    }
    ()
}
