use std::io;

use rhai::EvalAltResult;

use crate::system::*;

pub fn check_gate_ce(exp: &CellExpression) -> Result<(), Box<EvalAltResult>> {
    match exp {
        CellExpression::Calculated(_) => create_error(
            io::ErrorKind::Unsupported,
            "Calculated cell (no operators than +/*/- is allowed in gate) cannot be converted to gate.",
        ),
        CellExpression::Constant(_) => Ok(()),
        CellExpression::CellValue(c) => match c.column.ctype {
            crate::system::ColumnType::Selector => create_error(
                io::ErrorKind::Unsupported,
                "Selector cannot be used in gate",
            ),
            crate::system::ColumnType::Advice => Ok(()),
            crate::system::ColumnType::Fixed => match c.index {
                0 => Ok(()),
                _ => create_error(
                    io::ErrorKind::Unsupported,
                    "Fixed column cannot have rotation in gate, refer to https://github.com/zcash/halo2/issues/585",
                ),
            },
                
            crate::system::ColumnType::Instance => create_error(
                io::ErrorKind::Unsupported,
                "Instance cannot be used in gate",
            ),

            crate::system::ColumnType::ComplexSelector => create_error(
                io::ErrorKind::Unsupported,
                "Complex selector cannot be used in gate",
            ),

            crate::system::ColumnType::TableLookup => create_error(
                io::ErrorKind::Unsupported,
                "Lookup cannot be used in gate",
            ),
        },
        CellExpression::Negated(n) => check_gate_ce(&*n),
        CellExpression::Product(a, b) => check_gate_ce(&*a).and(check_gate_ce(&*b)),
        CellExpression::Sum(a, b) => check_gate_ce(&*a).and(check_gate_ce(&*b)),
        CellExpression::Scaled(a, _) => check_gate_ce(&*a),
    }
}

fn create_error(kind: io::ErrorKind, err: &str) -> Result<(), Box<EvalAltResult>> {
    Err(Box::new(EvalAltResult::ErrorSystem(
        "gate check error".to_string(),
        Box::new(io::Error::new(kind, err)),
    )))
}
