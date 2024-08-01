#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_doc_comments)]
use std::marker::PhantomData;
use std::{collections::HashMap, io};
use halo2_proofs::{
    circuit::{floor_planner::V1, *},
    pasta::group::ff::PrimeField, plonk::*, poly::Rotation,
};
#[derive(Default, Debug)]
pub struct MyCircuit<F: PrimeField> {
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
impl<F: PrimeField> Circuit<F> for MyCircuit<F> {
    type Config = CommonConfig<F>;
    type FloorPlanner = V1;
    fn without_witnesses(&self) -> Self {
        Self::default()
    }
    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let mut advices = Vec::new();
        let mut fixeds = Vec::new();
        let mut selectors = Vec::new();
        let mut instances = Vec::new();
        let mut lookups = Vec::new();
        let acells = Vec::new();
        instances.push(("defins".to_string(), meta.instance_column()));
        /// build columns
        advices.push(("in1_dec".to_string(), meta.advice_column()));
        advices.push(("in2_dec".to_string(), meta.advice_column()));
        advices.push(("out_dec".to_string(), meta.advice_column()));
        selectors.push(("xor_s".to_string(), meta.complex_selector()));
        lookups.push(("xor_in1".to_string(), meta.lookup_table_column()));
        lookups.push(("xor_in2".to_string(), meta.lookup_table_column()));
        lookups.push(("xor_out".to_string(), meta.lookup_table_column()));
        selectors.push(("gate_0".to_string(), meta.selector()));
        selectors.push(("gate_1".to_string(), meta.selector()));
        selectors.push(("gate_2".to_string(), meta.selector()));
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
        meta.create_gate(
            "gate_0",
            |meta| {
                vec![
                    (config.query_column(meta, ColumnType::Selector, "gate_0", 0i64)
                    .unwrap() * ((- config.query_column(meta, ColumnType::Advice,
                    "in1_dec", 0i64).unwrap()) + ((config.query_column(meta,
                    ColumnType::Advice, "in1_dec", 1i64).unwrap() + (config
                    .query_column(meta, ColumnType::Advice, "in1_dec", 2i64).unwrap() *
                    F::from(2u64))) + (config.query_column(meta, ColumnType::Advice,
                    "in1_dec", 3i64).unwrap() * F::from(4u64)))))
                ]
            },
        );
        meta.create_gate(
            "gate_1",
            |meta| {
                vec![
                    (config.query_column(meta, ColumnType::Selector, "gate_1", 0i64)
                    .unwrap() * ((- config.query_column(meta, ColumnType::Advice,
                    "in2_dec", 0i64).unwrap()) + ((config.query_column(meta,
                    ColumnType::Advice, "in2_dec", 1i64).unwrap() + (config
                    .query_column(meta, ColumnType::Advice, "in2_dec", 2i64).unwrap() *
                    F::from(2u64))) + (config.query_column(meta, ColumnType::Advice,
                    "in2_dec", 3i64).unwrap() * F::from(4u64)))))
                ]
            },
        );
        meta.create_gate(
            "gate_2",
            |meta| {
                vec![
                    (config.query_column(meta, ColumnType::Selector, "gate_2", 0i64)
                    .unwrap() * ((- config.query_column(meta, ColumnType::Advice,
                    "out_dec", 0i64).unwrap()) + ((config.query_column(meta,
                    ColumnType::Advice, "out_dec", 1i64).unwrap() + (config
                    .query_column(meta, ColumnType::Advice, "out_dec", 2i64).unwrap() *
                    F::from(2u64))) + (config.query_column(meta, ColumnType::Advice,
                    "out_dec", 3i64).unwrap() * F::from(4u64)))))
                ]
            },
        );
        /// build lookups
        meta.lookup(|meta| {
            vec![
                ((config.query_column(meta, ColumnType::Advice, "in1_dec", 0i64).unwrap()
                * config.query_column(meta, ColumnType::ComplexSelector, "xor_s", 0i64)
                .unwrap()), config.get_table_lookup(& "xor_in1").unwrap()), ((config
                .query_column(meta, ColumnType::Advice, "in2_dec", 0i64).unwrap() *
                config.query_column(meta, ColumnType::ComplexSelector, "xor_s", 0i64)
                .unwrap()), config.get_table_lookup(& "xor_in2").unwrap()), ((config
                .query_column(meta, ColumnType::Advice, "out_dec", 0i64).unwrap() *
                config.query_column(meta, ColumnType::ComplexSelector, "xor_s", 0i64)
                .unwrap()), config.get_table_lookup(& "xor_out").unwrap())
            ]
        });
        config
    }
    fn synthesize(
        &self,
        mut config: Self::Config,
        mut layouter: impl Layouter<F>,
    ) -> Result<(), Error> {
        layouter
            .assign_region(
                || "default",
                |mut region| {
                    let acell = region
                        .assign_advice(
                            || "advice",
                            config.get_advice(&"in1_dec")?,
                            1usize,
                            || Value::known(F::from(1u64)),
                        )?;
                    config.acells.push(("in1_dec[1]_default_0".to_string(), acell));
                    let acell = region
                        .assign_advice(
                            || "advice",
                            config.get_advice(&"in1_dec")?,
                            2usize,
                            || Value::known(F::from(1u64)),
                        )?;
                    config.acells.push(("in1_dec[2]_default_0".to_string(), acell));
                    let acell = region
                        .assign_advice(
                            || "advice",
                            config.get_advice(&"in1_dec")?,
                            3usize,
                            || Value::known(F::from(1u64)),
                        )?;
                    config.acells.push(("in1_dec[3]_default_0".to_string(), acell));
                    let acell = region
                        .assign_advice(
                            || "advice",
                            config.get_advice(&"in1_dec")?,
                            0usize,
                            || {
                                config
                                    .get_assigned_cell("in1_dec[1]_default_0")
                                    .value()
                                    .copied()
                                    + config
                                        .get_assigned_cell("in1_dec[2]_default_0")
                                        .value()
                                        .copied() * Value::known(F::from(2u64))
                                    + config
                                        .get_assigned_cell("in1_dec[3]_default_0")
                                        .value()
                                        .copied() * Value::known(F::from(4u64))
                            },
                        )?;
                    config.acells.push(("in1_dec[0]_default_0".to_string(), acell));
                    config.get_selector(&"gate_0")?.enable(&mut region, 0usize)?;
                    let acell = region
                        .assign_advice(
                            || "advice",
                            config.get_advice(&"in2_dec")?,
                            1usize,
                            || Value::known(F::from(1u64)),
                        )?;
                    config.acells.push(("in2_dec[1]_default_0".to_string(), acell));
                    let acell = region
                        .assign_advice(
                            || "advice",
                            config.get_advice(&"in2_dec")?,
                            2usize,
                            || Value::known(F::from(0u64)),
                        )?;
                    config.acells.push(("in2_dec[2]_default_0".to_string(), acell));
                    let acell = region
                        .assign_advice(
                            || "advice",
                            config.get_advice(&"in2_dec")?,
                            3usize,
                            || Value::known(F::from(0u64)),
                        )?;
                    config.acells.push(("in2_dec[3]_default_0".to_string(), acell));
                    let acell = region
                        .assign_advice(
                            || "advice",
                            config.get_advice(&"in2_dec")?,
                            0usize,
                            || {
                                config
                                    .get_assigned_cell("in2_dec[1]_default_0")
                                    .value()
                                    .copied()
                                    + config
                                        .get_assigned_cell("in2_dec[2]_default_0")
                                        .value()
                                        .copied() * Value::known(F::from(2u64))
                                    + config
                                        .get_assigned_cell("in2_dec[3]_default_0")
                                        .value()
                                        .copied() * Value::known(F::from(4u64))
                            },
                        )?;
                    config.acells.push(("in2_dec[0]_default_0".to_string(), acell));
                    config.get_selector(&"gate_1")?.enable(&mut region, 0usize)?;
                    let acell = region
                        .assign_advice(
                            || "advice",
                            config.get_advice(&"out_dec")?,
                            1usize,
                            || Value::known(F::from(0u64)),
                        )?;
                    config.acells.push(("out_dec[1]_default_0".to_string(), acell));
                    let acell = region
                        .assign_advice(
                            || "advice",
                            config.get_advice(&"out_dec")?,
                            2usize,
                            || Value::known(F::from(1u64)),
                        )?;
                    config.acells.push(("out_dec[2]_default_0".to_string(), acell));
                    let acell = region
                        .assign_advice(
                            || "advice",
                            config.get_advice(&"out_dec")?,
                            3usize,
                            || Value::known(F::from(1u64)),
                        )?;
                    config.acells.push(("out_dec[3]_default_0".to_string(), acell));
                    let acell = region
                        .assign_advice(
                            || "advice",
                            config.get_advice(&"out_dec")?,
                            0usize,
                            || {
                                config
                                    .get_assigned_cell("out_dec[1]_default_0")
                                    .value()
                                    .copied()
                                    + config
                                        .get_assigned_cell("out_dec[2]_default_0")
                                        .value()
                                        .copied() * Value::known(F::from(2u64))
                                    + config
                                        .get_assigned_cell("out_dec[3]_default_0")
                                        .value()
                                        .copied() * Value::known(F::from(4u64))
                            },
                        )?;
                    config.acells.push(("out_dec[0]_default_0".to_string(), acell));
                    config.get_selector(&"gate_2")?.enable(&mut region, 0usize)?;
                    config.get_selector(&"xor_s")?.enable(&mut region, 1usize)?;
                    config.get_selector(&"xor_s")?.enable(&mut region, 2usize)?;
                    config.get_selector(&"xor_s")?.enable(&mut region, 3usize)?;
                    Ok(())
                },
            )?;
        layouter
            .assign_table(
                || "xor_in1",
                |mut table| {
                    table
                        .assign_cell(
                            || "xor_in1",
                            config.get_table_lookup("xor_in1")?,
                            0usize,
                            || Value::known(F::from(0u64)),
                        )?;
                    table
                        .assign_cell(
                            || "xor_in1",
                            config.get_table_lookup("xor_in1")?,
                            1usize,
                            || Value::known(F::from(0u64)),
                        )?;
                    table
                        .assign_cell(
                            || "xor_in1",
                            config.get_table_lookup("xor_in1")?,
                            2usize,
                            || Value::known(F::from(1u64)),
                        )?;
                    table
                        .assign_cell(
                            || "xor_in1",
                            config.get_table_lookup("xor_in1")?,
                            3usize,
                            || Value::known(F::from(1u64)),
                        )?;
                    Ok(())
                },
            )?;
        layouter
            .assign_table(
                || "xor_in2",
                |mut table| {
                    table
                        .assign_cell(
                            || "xor_in2",
                            config.get_table_lookup("xor_in2")?,
                            0usize,
                            || Value::known(F::from(0u64)),
                        )?;
                    table
                        .assign_cell(
                            || "xor_in2",
                            config.get_table_lookup("xor_in2")?,
                            1usize,
                            || Value::known(F::from(1u64)),
                        )?;
                    table
                        .assign_cell(
                            || "xor_in2",
                            config.get_table_lookup("xor_in2")?,
                            2usize,
                            || Value::known(F::from(0u64)),
                        )?;
                    table
                        .assign_cell(
                            || "xor_in2",
                            config.get_table_lookup("xor_in2")?,
                            3usize,
                            || Value::known(F::from(1u64)),
                        )?;
                    Ok(())
                },
            )?;
        layouter
            .assign_table(
                || "xor_out",
                |mut table| {
                    table
                        .assign_cell(
                            || "xor_out",
                            config.get_table_lookup("xor_out")?,
                            0usize,
                            || Value::known(F::from(0u64)),
                        )?;
                    table
                        .assign_cell(
                            || "xor_out",
                            config.get_table_lookup("xor_out")?,
                            1usize,
                            || Value::known(F::from(1u64)),
                        )?;
                    table
                        .assign_cell(
                            || "xor_out",
                            config.get_table_lookup("xor_out")?,
                            2usize,
                            || Value::known(F::from(1u64)),
                        )?;
                    table
                        .assign_cell(
                            || "xor_out",
                            config.get_table_lookup("xor_out")?,
                            3usize,
                            || Value::known(F::from(0u64)),
                        )?;
                    Ok(())
                },
            )?;
        Ok(())
    }
}
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
            .ok_or(
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(
                        "cannot find column of {} in columns ({})", name, columns.iter()
                        .map(| x | x.0.clone()).collect:: < Vec < String >> ().join(",")
                    ),
                ),
            )
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
            ColumnType::Selector => {
                self.get_selector(&col_name).map(|x| meta.query_selector(x))
            }
            ColumnType::Advice => {
                self.get_advice(&col_name)
                    .map(|x| {
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
                    })
            }
            ColumnType::Fixed => self.get_fixed(&col_name).map(|x| meta.query_fixed(x)),
            ColumnType::Instance => todo!(),
            ColumnType::ComplexSelector => {
                self.get_selector(&col_name).map(|x| meta.query_selector(x))
            }
            ColumnType::TableLookup => todo!(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use halo2_proofs::{dev::MockProver, pasta::Fp as F};
    #[test]
    fn test_simple() {
        let circuit = MyCircuit {
            _marker: std::marker::PhantomData,
        };
        let prover = MockProver::run(
                4u32,
                &circuit,
                vec![vec![F::from(7u64), F::from(1u64), F::from(6u64)]],
            )
            .unwrap();
        if cfg!(debug_assertions) {
            let d = format!("{:#?}", prover);
            let mut file = std::fs::File::create("visualization.rust").unwrap();
            std::io::Write::write_all(&mut file, d.as_bytes()).unwrap();
        }
        prover.assert_satisfied();
    }
}
