use once_cell::sync::Lazy;
use std::{collections::HashMap, fmt};

pub mod cell_expression;
pub use cell_expression::ToCellExpression;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ColumnType {
    Selector,
    Advice,
    Fixed,
    Instance,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum SpecialType {
    Input,
    Output,
    Field,
    None,
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct Column {
    pub name: String,
    pub ctype: ColumnType,
    pub stype: SpecialType,
}

// Debug
impl fmt::Debug for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Column")
            .field("name", &self.name)
            .field("ctype", &self.ctype)
            .field("stype", &self.stype)
            .finish()
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Cell {
    pub column: Column,
    pub name: String,
    pub value: Option<String>,
    pub index: i64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum CellExpression {
    Constant(String),
    CellValue(Cell),
    Negated(Box<CellExpression>),
    Product(Box<CellExpression>, Box<CellExpression>),
    Sum(Box<CellExpression>, Box<CellExpression>),
    Scaled(Box<CellExpression>, String),
}

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct SimplifiedConstraitSystem {
    pub signals: Vec<Cell>,
    pub columns: Vec<Column>,
    pub regions: Vec<Region>,
    pub instance_count: i64,
    pub gates: Vec<(String, String, Column, CellExpression)>,// name, expression string(key), column, expression
    pub inputs: Lazy<HashMap<String, String>>,
    pub cells: Lazy<HashMap<String, Cell>>,
}

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct Region {
    pub name: String,
    pub id: i64,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Instruction {
    // CopyAdvice(),
    AssignFixed(Cell, CellExpression), // fixed, fix_row(offset), value expression
    EnableSelector(Cell),              // selector, row(offset)
    AssignAdvice(Cell, CellExpression), // advice, adv_row(offset), value expression
    AssignAdviceFromConstant(Cell, i64), // advice, adv_row(offset), constant
    AssignAdviceFromInstance(Cell, Cell), // advice, adv_row(offset), instance, ins_row
    ConstrainEqual(Cell, Cell),        // advice, adv_row(offset), advice, adv_row(offset)
    ConstrainConstant(),               //
}