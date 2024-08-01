use std::collections::HashMap;

use crate::generator::util::convert_to_value;
use crate::system::CellExpression;
use crate::system::SimplifiedConstraitSystem;
use proc_macro2::TokenStream;
use quote::quote;

pub fn get_circuit_synthesize(cs: &SimplifiedConstraitSystem) -> TokenStream {
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
