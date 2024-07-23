use crate::system::*;

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
