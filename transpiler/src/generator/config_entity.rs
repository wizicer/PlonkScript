use crate::system::SimplifiedConstraitSystem;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn get_config_entity(circuit_name: &str, cs: &SimplifiedConstraitSystem) -> TokenStream {
    let structs = get_config_structs(circuit_name, cs);
    let cimpl = get_config_impl();
    quote! {
        #structs
        #cimpl
    }
}

fn get_config_structs(circuit_name: &str, _cs: &SimplifiedConstraitSystem) -> TokenStream {
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
