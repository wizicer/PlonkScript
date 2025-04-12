use num_bigint::BigInt;
use num_traits::cast::ToPrimitive;
use std::str::FromStr;

use crate::system::{cell_expression::ToValueString, *};

// operators
// Cell/CellExpression/Column/String/i64
macro_rules! engine_register_ops {
    ($eng: expr, $op: tt, $func: ident, $a:ty, $b:ty) => {
        $eng.register_fn(stringify!($op), $func::<$a, $b>);
    };
}

macro_rules! engine_register_ops_i64 {
    ($eng: expr, $op: tt, $func: ident, $a:ty) => {
        $eng.register_fn($op, $func::<$a>);
    };
}

macro_rules! engine_register_ops_types {
    ($eng: expr, $op: tt, $func: ident) => {
        engine_register_ops!($eng, $op, $func, Cell, Cell);
        engine_register_ops!($eng, $op, $func, Cell, CellExpression);
        engine_register_ops!($eng, $op, $func, Cell, String);
        engine_register_ops!($eng, $op, $func, Cell, Column);
        engine_register_ops!($eng, $op, $func, Cell, i64);

        engine_register_ops!($eng, $op, $func, CellExpression, Cell);
        engine_register_ops!($eng, $op, $func, CellExpression, CellExpression);
        engine_register_ops!($eng, $op, $func, CellExpression, String);
        engine_register_ops!($eng, $op, $func, CellExpression, Column);
        engine_register_ops!($eng, $op, $func, CellExpression, i64);

        engine_register_ops!($eng, $op, $func, String, Cell);
        engine_register_ops!($eng, $op, $func, String, CellExpression);
        engine_register_ops!($eng, $op, $func, String, String);
        engine_register_ops!($eng, $op, $func, String, Column);
        engine_register_ops!($eng, $op, $func, String, i64);

        engine_register_ops!($eng, $op, $func, Column, Cell);
        engine_register_ops!($eng, $op, $func, Column, CellExpression);
        engine_register_ops!($eng, $op, $func, Column, String);
        engine_register_ops!($eng, $op, $func, Column, Column);
        engine_register_ops!($eng, $op, $func, Column, i64);

        engine_register_ops!($eng, $op, $func, i64, Cell);
        engine_register_ops!($eng, $op, $func, i64, CellExpression);
        engine_register_ops!($eng, $op, $func, i64, String);
        engine_register_ops!($eng, $op, $func, i64, Column);
        engine_register_ops!($eng, $op, $func, i64, i64);
    };
}

macro_rules! engine_register_ops_types_i64 {
    ($eng: expr, $op: tt, $func: ident) => {
        engine_register_ops_i64!($eng, $op, $func, Cell);
        engine_register_ops_i64!($eng, $op, $func, CellExpression);
        engine_register_ops_i64!($eng, $op, $func, String);
        engine_register_ops_i64!($eng, $op, $func, Column);
        engine_register_ops_i64!($eng, $op, $func, i64);
    };
}

pub fn register_operator(engine: &mut rhai::Engine) {
    engine_register_ops_types!(engine, +, operator_plus);
    engine_register_ops_types!(engine, -, operator_minus);
    engine_register_ops_types!(engine, *, operator_mul);
    engine_register_ops_types_i64!(engine, "**", operator_pow);
    engine_register_ops_types!(engine, /, operator_divide);

    // to calculated value
    engine.register_fn(">>", |t1: String, t2: i64| {
        perform_operation(&t1, &t2.to_string(), OperationEnum::RightShift)
    });
    engine.register_fn(">>", |t1: String, t2: String| {
        perform_operation(&t1, &t2, OperationEnum::RightShift)
    });
    engine.register_fn("&", |t1: String, t2: i64| {
        perform_operation(&t1, &t2.to_string(), OperationEnum::BinaryAnd)
    });
    engine.register_fn("&", |t1: String, t2: String| {
        perform_operation(&t1, &t2, OperationEnum::BinaryAnd)
    });
    engine.register_fn("^", |t1: String, t2: i64| {
        perform_operation(&t1, &t2.to_string(), OperationEnum::ExclusiveOr)
    });
    engine.register_fn("^", |t1: String, t2: String| {
        perform_operation(&t1, &t2, OperationEnum::ExclusiveOr)
    });
}

fn operator_plus<T1: ToCellExpression, T2: ToCellExpression>(a: T1, b: T2) -> CellExpression {
    CellExpression::Sum(
        Box::new(a.to_cell_expression()),
        Box::new(b.to_cell_expression()),
    )
}

fn operator_minus<T1: ToCellExpression, T2: ToCellExpression>(a: T1, b: T2) -> CellExpression {
    CellExpression::Sum(
        Box::new(a.to_cell_expression()),
        Box::new(CellExpression::Negated(Box::new(b.to_cell_expression()))),
    )
}

fn operator_mul<T1: ToCellExpression, T2: ToCellExpression>(a: T1, b: T2) -> CellExpression {
    match ((a.to_cell_expression()), (b.to_cell_expression())) {
        (CellExpression::Constant(a), b) => CellExpression::Scaled(Box::new(b), a),
        (b, CellExpression::Constant(a)) => CellExpression::Scaled(Box::new(b), a),
        (a, b) => CellExpression::Product(Box::new(a), Box::new(b)),
    }
}

fn operator_pow<T1: ToCellExpression>(a: T1, b: i64) -> CellExpression {
    let origin_exp = a.to_cell_expression();
    let mut exp = origin_exp.clone();
    for _ in 1..b {
        exp = CellExpression::Product(Box::new(exp), Box::new(origin_exp.clone()));
    }
    exp
}

fn operator_divide<T1: ToCellExpression, T2: ToCellExpression>(a: T1, b: T2) -> CellExpression {
    CellExpression::Constant(
        (BigInt::from_str(&a.to_cell_expression().to_value_string().unwrap()).unwrap()
            / BigInt::from_str(&b.to_cell_expression().to_value_string().unwrap()).unwrap()).to_string(),
    )
}

#[derive(Debug, Clone)]
pub enum OperationEnum {
    RightShift,
    LeftShift,
    BinaryAnd,
    BinaryOr,
    ExclusiveOr,
}

fn perform_operation(big_num_str1: &str, big_num_str2: &str, operation: OperationEnum) -> String {
    let big_num1 = BigInt::from_str(big_num_str1).unwrap();
    let big_num2 = BigInt::from_str(big_num_str2).unwrap();

    match operation {
        OperationEnum::RightShift => {
            let shift = big_num2.to_usize().unwrap();
            (big_num1 >> shift).to_string()
        }
        OperationEnum::LeftShift => {
            let shift = big_num2.to_usize().unwrap();
            (big_num1 << shift).to_string()
        }
        OperationEnum::BinaryAnd => (big_num1 & big_num2).to_string(),
        OperationEnum::BinaryOr => (big_num1 | big_num2).to_string(),
        OperationEnum::ExclusiveOr => (big_num1 ^ big_num2).to_string(),
    }
}
