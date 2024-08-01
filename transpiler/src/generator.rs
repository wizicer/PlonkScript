use std::collections::HashMap;
use std::io;

use crate::system::CellExpression;
use crate::{engine::DEFAULT_INSTANCE_COLUMN_NAME, system::SimplifiedConstraitSystem};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn generate_rust_code(cs: &SimplifiedConstraitSystem) -> String {
    let header = get_header();
    let circuit_name = "MyCircuit";
    let structs = get_structs(circuit_name, cs);
    let impls = get_circuit_impl(circuit_name, cs);
    let config_impl = get_config_impl();
    let test = get_test(circuit_name, cs);
    let output = quote! {
        #header
        #structs
        #impls
        #config_impl
        #test
    };

    // println!("{}", output);
    let syntax_tree = syn::parse2(output).unwrap();
    let formatted = prettyplease::unparse(&syntax_tree);
    formatted
}

fn get_header() -> TokenStream {
    quote! {
        #![allow(unused_imports)]
        #![allow(dead_code)]
        #![allow(unused_mut)]
        #![allow(unused_doc_comments)]
        use std::marker::PhantomData;
        use std::{collections::HashMap, io};

        use halo2_proofs::{
            circuit::{floor_planner::V1, *},
            pasta::group::ff::PrimeField,
            plonk::*,
            poly::Rotation,
        };
    }
}

fn get_structs(circuit_name: &str, _cs: &SimplifiedConstraitSystem) -> TokenStream {
    let name = format_ident!("{}", circuit_name);
    quote! {
        #[derive(Default, Debug)]
        pub struct #name<F: PrimeField> {
            pub _marker: PhantomData<F>,
        }


        #[derive(Debug, Clone)]
        pub struct CommonConfig<F: PrimeField> {
            advices: Vec<(String, Column<Advice>)>,
            fixeds: Vec<(String, Column<Fixed>)>,
            selectors: Vec<(String, Selector)>,
            instances: Vec<(String, Column<Instance>)>,
            lookups: Vec<(String, TableColumn)>,
            acells: Vec<(String, AssignedCell<F, F>)>,
            _marker: PhantomData<F>,
        }
    }
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

fn get_circuit_configure(cs: &SimplifiedConstraitSystem) -> TokenStream {
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

fn get_circuit_synthesize_regions(cs: &SimplifiedConstraitSystem) -> Vec<TokenStream> {
    cs.regions
        .iter()
        .map(|region| {
            let ins = region.instructions.iter().map(|ins| match ins {
                crate::system::Instruction::EnableSelector(c) => {
                    let colname = c.column.name.as_str();
                    let idx = c.index as usize;
                    quote! {
                        config.get_selector(&#colname)?
                            .enable(&mut region, #idx)?;
                    }
                }
                crate::system::Instruction::AssignFixed(f, exp) => {
                    let colname = f.column.name.as_str();
                    let cellname = f.name.as_str();
                    let idx = f.index as usize;
                    let exp = convert_to_value(exp);
                    quote! {
                        let acell = region.assign_fixed(
                            || "fixed",
                            config.get_fixed(&#colname)?,
                            #idx,
                            || #exp,
                        )?;
                        config.acells.push((#cellname.to_string(), acell));
                    }
                }
                crate::system::Instruction::AssignAdvice(a, exp) => {
                    let colname = a.column.name.as_str();
                    let cellname = a.name.as_str();
                    let idx = a.index as usize;
                    let exp = convert_to_value(exp);
                    quote! {
                        let acell = region.assign_advice(
                            || "advice",
                            config.get_advice(&#colname)?,
                            #idx ,
                            || #exp,
                        )?;
                        config.acells.push((#cellname.to_string(), acell));
                    }
                }
                crate::system::Instruction::AssignAdviceFromInstance(a, b) => {
                    let colname_a = a.column.name.as_str();
                    let idx_a = a.index as usize;
                    let colname_b = b.column.name.as_str();
                    let idx_b = b.index as usize;
                    let cellname = a.name.as_str();
                    quote! {
                        let acell = region.assign_advice_from_instance(
                            || "instance",
                            config.get_instance(&#colname_b)?,
                            #idx_b ,
                            config.get_advice(&#colname_a)?,
                            #idx_a ,
                        )?;
                        config.acells.push((#cellname.to_string(), acell));
                    }
                }
                crate::system::Instruction::ConstrainEqual(a, b) => {
                    let cellname_a = a.name.as_str();
                    let cellname_b = b.name.as_str();
                    quote! {
                        let acell = config.get_assigned_cell(#cellname_a);
                        let bcell = config.get_assigned_cell(#cellname_b);
                        region.constrain_equal(acell.cell(), bcell.cell())?;
                    }
                }
                crate::system::Instruction::AssignCell(_, _) => todo!("illegal instruction"),
                crate::system::Instruction::AssignAdviceFromConstant(_, _) => todo!(),
                crate::system::Instruction::ConstrainConstant() => todo!(),
            });

            let region_name = region.name.clone();
            quote! {
                layouter.assign_region(
                    || #region_name,
                    |mut region| {
                        #(#ins;)*
                        Ok(())
                    }
                )?
            }
        })
        .collect()
}

fn get_circuit_synthesize_tables(cs: &SimplifiedConstraitSystem) -> Vec<TokenStream> {
    let mut max_indexes = HashMap::<&str, usize>::new();
    cs.tables
        .iter()
        .map(|t| {
            let ins = t.instructions.iter().map(|ins| match ins {
                crate::system::Instruction::AssignCell(a, b) => {
                    let idx = max_indexes.get_mut(a.name.as_str());
                    let idx = match idx {
                        Some(i) => {
                            *i += 1;
                            *i
                        }
                        None => {
                            let name = Box::leak(a.name.clone().into_boxed_str());
                            max_indexes.insert(name, 0);
                            0
                        }
                    };

                    let cell_name = a.name.clone();
                    let exp = convert_to_value(&CellExpression::Constant(b.clone()));
                    quote! {
                    table.assign_cell(
                        || #cell_name,
                        config.get_table_lookup(#cell_name)?,
                        #idx,
                        || #exp,
                    )?;

                    }
                }
                _ => todo!("illegal instruction"),
            });

            let table_name = t.name.clone();

            quote! {
                layouter.assign_table(
                    || #table_name,
                    |mut table| {
                        #(#ins;)*
                        Ok(())
                    },
                )?;
            }
        })
        .collect()
}

fn get_circuit_synthesize(cs: &SimplifiedConstraitSystem) -> TokenStream {
    let regions = get_circuit_synthesize_regions(cs);

    let tables = get_circuit_synthesize_tables(cs);

    quote! {
        fn synthesize(
            &self,
            mut config: Self::Config,
            mut layouter: impl Layouter<F>,
        ) -> Result<(), Error> {

            #(#regions;)*

            #(#tables;)*

            Ok(())
        }
    }
}

fn get_config_impl() -> TokenStream {
    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ColumnType {
            Selector,
            Advice,
            Fixed,
            Instance,
            ComplexSelector,
            TableLookup,
        }

        impl<F: PrimeField> CommonConfig<F> {
            fn get_selector(&self, name: &str) -> Result<Selector, io::Error> {
                Self::get_column(&self.selectors, name)
            }

            fn get_advice(&self, name: &str) -> Result<Column<Advice>, io::Error> {
                Self::get_column(&self.advices, name)
            }

            fn get_fixed(&self, name: &str) -> Result<Column<Fixed>, io::Error> {
                Self::get_column(&self.fixeds, name)
            }

            fn get_instance(&self, name: &str) -> Result<Column<Instance>, io::Error> {
                Self::get_column(&self.instances, name)
            }

            fn get_table_lookup(&self, name: &str) -> Result<TableColumn, io::Error> {
                Self::get_column(&self.lookups, name)
            }

            fn get_column<T>(columns: &Vec<(String, T)>, name: &str) -> Result<T, io::Error>
            where
                T: Clone,
            {
                columns
                    .iter()
                    .filter(|x| x.0 == *name)
                    .nth(0)
                    .map(|x| x.1.clone())
                    .ok_or(io::Error::new(
                        io::ErrorKind::NotFound,
                        format!(
                            "cannot find column of {} in columns ({})",
                            name,
                            columns
                                .iter()
                                .map(|x| x.0.clone())
                                .collect::<Vec<String>>()
                                .join(",")
                        ),
                    ))
            }

            fn get_assigned_cell(&self, name: &str) -> AssignedCell<F, F> {
                if let Some(acell) = self.acells.iter().filter(|x| x.0 == name).nth(0) {
                    return acell.1.clone();
                }
                panic!("cannot find assigned cell of {}", name)
            }

            pub fn query_column(
                &self,
                meta: &mut VirtualCells<F>,
                col_type: ColumnType,
                col_name: &str,
                idx: i64,
            ) -> Result<Expression<F>, io::Error> {
                match col_type {
                    ColumnType::Selector => self
                        .get_selector(&col_name)
                        .map(|x| meta.query_selector(x)),
                    ColumnType::Advice => self.get_advice(&col_name).map(|x| {
                        meta.query_advice(
                            x,
                            match idx {
                                -1 => Rotation::prev(),
                                0 => Rotation::cur(),
                                1 => Rotation::next(),
                                x @ 2..=5 => Rotation(x as i32),
                                x => panic!("too big the rotation: {}", x),
                            },
                        )
                    }),
                    ColumnType::Fixed => {
                        self.get_fixed(&col_name).map(|x| meta.query_fixed(x))
                    }
                    ColumnType::Instance => todo!(),
                    ColumnType::ComplexSelector => self
                        .get_selector(&col_name)
                        .map(|x| meta.query_selector(x)),
                    ColumnType::TableLookup => todo!(),
                }
            }
        }
    }
}

fn get_test(circuit_name: &str, cs: &SimplifiedConstraitSystem) -> TokenStream {
    let circuit_name = format_ident!("{}", circuit_name);
    let k = cs
        .inputs
        .get("k")
        .or(Some(&"8".to_string()))
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let public_input = cs
        .signals
        .clone()
        .into_iter()
        .map(|x| match x.value {
            Some(x) => x
                .to_quote_field()
                .expect(format!("Decoding failed: {x}").as_str()),
            None => panic!("No value for signal [{}]", x.name),
        })
        .collect::<Vec<TokenStream>>();

    quote! {
        #[cfg(test)]
        mod tests {
            use super::*;
            use halo2_proofs::{dev::MockProver, pasta::Fp as F};

            #[test]
            fn test_simple() {
                let circuit = #circuit_name {
                    _marker: std::marker::PhantomData,
                };

                let prover = MockProver::run(#k, &circuit, vec![vec![#(#public_input),*]]).unwrap();
                if cfg!(debug_assertions) {
                    let d = format!("{:#?}", prover);
                    let mut file = std::fs::File::create("visualization.rust").unwrap();
                    std::io::Write::write_all(&mut file, d.as_bytes()).unwrap();
                }

                prover.assert_satisfied();
            }
        }
    }
}

fn convert_to_gate_expression(exp: &CellExpression) -> Result<TokenStream, io::Error> {
    match exp {
        CellExpression::Calculated(c) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("calculated value {} is allowed in gate", c),
        )),
        CellExpression::Constant(c) => c
            .to_quote_field()
            .ok_or(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("cannot convert {} to field element", c),
            ))
            .map(|x| quote! { Expression::Constant(#x) }),
        CellExpression::CellValue(c) => {
            let col_type = match c.column.ctype {
                crate::system::ColumnType::Advice => quote! {ColumnType::Advice},
                crate::system::ColumnType::Selector => quote! {ColumnType::Selector},
                crate::system::ColumnType::Fixed => quote! {ColumnType::Fixed},
                crate::system::ColumnType::Instance => quote! {ColumnType::Instance},
                crate::system::ColumnType::ComplexSelector => quote! {ColumnType::ComplexSelector},
                crate::system::ColumnType::TableLookup => quote! {ColumnType::TableLookup},
            };
            let col_name = c.column.name.clone();
            let idx = c.index;

            match c.column.ctype {
                crate::system::ColumnType::Selector => {
                    Ok(quote! { config.query_column(meta, #col_type, #col_name, #idx).unwrap() })
                }
                crate::system::ColumnType::Advice => {
                    Ok(quote! { config.query_column(meta, #col_type, #col_name, #idx).unwrap() })
                }
                crate::system::ColumnType::Fixed => {
                    Ok(quote! { config.query_column(meta, #col_type, #col_name, #idx).unwrap() })
                }
                crate::system::ColumnType::Instance => todo!(),
                crate::system::ColumnType::ComplexSelector => {
                    Ok(quote! { config.query_column(meta, #col_type, #col_name, #idx).unwrap() })
                }
                crate::system::ColumnType::TableLookup => todo!(),
            }
        }
        CellExpression::Negated(n) => convert_to_gate_expression(n).map(|x| quote! { (-#x) }),
        CellExpression::Product(a, b) => convert_to_gate_expression(a)
            .and_then(|a| convert_to_gate_expression(b).map(|b| quote! {(#a * #b)})),
        CellExpression::Sum(a, b) => convert_to_gate_expression(a)
            .and_then(|a| convert_to_gate_expression(b).map(|b| quote! {(#a + #b)})),
        CellExpression::Scaled(a, b) => convert_to_gate_expression(a).and_then(|a| {
            b.to_quote_field()
                .map(|b| quote! {(#a * #b)})
                .ok_or(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("cannot convert {} to field element", b),
                ))
        }),
    }
}

pub trait ToQuoteField {
    fn to_quote_field(&self) -> Option<TokenStream>;
}

impl ToQuoteField for String {
    fn to_quote_field(&self) -> Option<TokenStream> {
        match self {
            s if s.starts_with("0x") => {
                // let mut bytes = F::Repr::default();
                // let mut view = bytes.as_mut();

                let mut view = Vec::<u8>::new();
                hex::decode_to_slice(&s[2..], &mut view).ok();
                view.reverse();
                // let fixview = view.as_ref::<u8>();
                Some(quote! { F::from_repr_vartime([#(#view,)*]) })
            }
            s => {
                let s = s.parse::<u64>().unwrap();
                Some(quote! {F::from(#s)})
            }
        }
    }
}

fn convert_to_value(exp: &CellExpression) -> TokenStream {
    match exp {
        CellExpression::Calculated(c) => {
            let val = c.to_quote_field();
            quote! { Value::known(#val)}
        }
        CellExpression::Constant(c) => {
            let val = c.to_quote_field();
            quote! { Value::known(#val)}
        }
        CellExpression::CellValue(c) => {
            let name = c.name.as_str();
            match c.column.ctype {
                crate::system::ColumnType::Selector => {
                    quote! {config.get_assigned_cell(#name).value().copied()}
                }
                crate::system::ColumnType::Advice => {
                    quote! {config.get_assigned_cell(#name).value().copied()}
                }
                crate::system::ColumnType::Fixed => {
                    quote! {config.get_assigned_cell(#name).value().copied()}
                }
                crate::system::ColumnType::Instance => quote! {todo!()},
                crate::system::ColumnType::ComplexSelector => {
                    quote! { config.get_assigned_cell(#name).value().copied() }
                }
                crate::system::ColumnType::TableLookup => quote! {todo!()},
            }
        }
        CellExpression::Negated(n) => {
            let val = convert_to_value(n);
            quote! { -#val }
        }
        CellExpression::Product(a, b) => {
            let val1 = convert_to_value(a);
            let val2 = convert_to_value(b);
            quote! { #val1 * #val2 }
        }
        CellExpression::Sum(a, b) => {
            let val1 = convert_to_value(a);
            let val2 = convert_to_value(b);
            quote! { #val1 + #val2 }
        }
        CellExpression::Scaled(a, b) => {
            let val1 = convert_to_value(a);
            let val2 = convert_to_value(&CellExpression::Constant(b.clone()));
            quote! { #val1 * #val2 }
        }
    }
}
