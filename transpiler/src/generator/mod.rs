use crate::generator::configure::get_circuit_configure;
use crate::generator::synthesize::get_circuit_synthesize;
use crate::system::SimplifiedConstraitSystem;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use self::config_entity::get_config_entity;
use self::header::get_header;
use self::test::get_test;

mod config_entity;
mod configure;
mod header;
mod synthesize;
mod test;
mod util;

pub fn generate_rust_code(cs: &SimplifiedConstraitSystem) -> String {
    let header = get_header();
    let circuit_name = "MyCircuit";
    let impls = get_circuit_impl(circuit_name, cs);
    let config = get_config_entity(circuit_name, cs);
    let test = get_test(circuit_name, cs);
    let output = quote! {
        #header
        #impls
        #config
        #test
    };

    // println!("{}", output);
    let syntax_tree = syn::parse2(output).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);
    formatted
}

fn get_circuit_impl(circuit_name: &str, cs: &SimplifiedConstraitSystem) -> TokenStream {
    let circuit_name = format_ident!("{}", circuit_name);
    let configure = get_circuit_configure(cs);
    let synthesize = get_circuit_synthesize(cs);
    quote! {

        impl<F: PrimeField> Circuit<F> for #circuit_name<F> {
            type Config = CommonConfig<F>;
            type FloorPlanner = V1;

            fn without_witnesses(&self) -> Self {
                Self::default()
            }

            #configure
            #synthesize
        }
    }
}
