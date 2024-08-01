use crate::generator::util::convert_to_gate_expression;
use crate::{engine::DEFAULT_INSTANCE_COLUMN_NAME, system::SimplifiedConstraitSystem};
use proc_macro2::TokenStream;
use quote::quote;

pub fn get_circuit_configure(cs: &SimplifiedConstraitSystem) -> TokenStream {
    let instance_push = get_circuit_instances_push(cs);

    let type_pushes = get_circuit_type_pushes(cs);

    let gate_creates = get_circuit_gate_creates(cs);

    let lookup_creates = get_circuit_lookup_creates(cs);

    quote! {
        fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
            let mut advices = Vec::new();
            let mut fixeds = Vec::new();
            let mut selectors = Vec::new();
            let mut instances = Vec::new();
            let mut lookups = Vec::new();
            let acells = Vec::new();

            #instance_push

            /// build columns
            #(#type_pushes;)*

            /// enable_equality
            for c in advices.clone() {
                meta.enable_equality(c.1);
            }
            for c in instances.clone() {
                meta.enable_equality(c.1);
            }

            let config = CommonConfig {
                advices,
                fixeds,
                selectors,
                instances,
                lookups,
                acells,
                _marker: PhantomData,
            };

            /// build gates
            #(#gate_creates;)*

            /// build lookups
            #(#lookup_creates;)*

            config
        }
    }
}

fn get_circuit_instances_push(cs: &SimplifiedConstraitSystem) -> TokenStream {
    let default_instance_column_name = DEFAULT_INSTANCE_COLUMN_NAME;
    if cs.signals.len() > 0 {
        quote! {
            instances.push((
                #default_instance_column_name.to_string(),
                meta.instance_column(),
            ));
        }
    } else {
        quote! {}
    }
}

fn get_circuit_type_pushes(cs: &SimplifiedConstraitSystem) -> Vec<TokenStream> {
    cs.columns
        .iter()
        .map(|col| {
            let name = col.name.as_str();
            match col.ctype {
                crate::system::ColumnType::Advice => {
                    quote! { advices.push((#name.to_string(), meta.advice_column()))}
                }
                crate::system::ColumnType::Selector => {
                    quote! { selectors.push((#name.to_string(), meta.selector()))}
                }
                crate::system::ColumnType::Fixed => {
                    quote! { fixeds.push((#name.to_string(), meta.fixed_column()))}
                }
                crate::system::ColumnType::Instance => {
                    quote! { instances.push((#name.to_string(), meta.instance_column()))}
                }
                crate::system::ColumnType::ComplexSelector => {
                    quote! { selectors.push((#name.to_string(), meta.complex_selector()))}
                }
                crate::system::ColumnType::TableLookup => {
                    quote! { lookups.push((#name.to_string(), meta.lookup_table_column()))}
                }
            }
        })
        .collect()
}

fn get_circuit_gate_creates(cs: &SimplifiedConstraitSystem) -> Vec<TokenStream> {
    cs.gates
        .iter()
        .map(|(gname, _, _, gate)| {
            let sgname = gname.as_str();

            let ge = convert_to_gate_expression(gate)
                .expect(format!("cannot convert gate expression of {}", sgname).as_str());
            quote! { meta.create_gate(#sgname, |meta| { vec![#ge] }); }
        })
        .collect()
}

fn get_circuit_lookup_creates(cs: &SimplifiedConstraitSystem) -> Vec<TokenStream> {
    cs.lookups
        .iter()
        .map(|lookup| {
            let name = lookup.name.as_str();
            let map = lookup.map.iter().map(|(exp, col)| {
                let colname = col.name.as_str();
                let ge = convert_to_gate_expression(exp)
                    .expect(format!("cannot convert lookup expression of {}", name).as_str());
                quote! { (#ge, config.get_table_lookup(&#colname).unwrap()) }
            });
            quote! {
                meta.lookup(|meta| {
                    vec![#(#map),*]
                });
            }
        })
        .collect()
}
