use std::io;

use crate::system::CellExpression;
use proc_macro2::TokenStream;
use quote::quote;

pub fn convert_to_gate_expression(exp: &CellExpression) -> Result<TokenStream, io::Error> {
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

pub fn convert_to_value(exp: &CellExpression) -> TokenStream {
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
