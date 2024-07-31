use core::panic;

use rhai::Array;
use rhai::EvalAltResult;

use self::cell_expression::ToBaseIndex;
use self::cell_expression::ToString;
use crate::engine::gate::check_gate_ce;
use crate::system::cell_expression::GetBaseIndex;
use crate::system::cell_expression::ToValueString;
use crate::system::*;
use crate::CONTEXT;

use super::custom_type::get_field_name;

pub fn register_bind(engine: &mut rhai::Engine) {
    let _ = &mut engine
        .register_fn("assign_constraint", assign_constraint)
        .register_fn("assign_constraint", assign_constraint_cell_ce)
        .register_fn("assign_constraint", assign_constraint_string)
        .register_fn("constrain_equal", constrain_equal)
        .register_fn("assign_common", assign_common_string)
        .register_fn("assign_common", assign_common_ce)
        .register_fn("assign_common", assign_common_i64)
        .register_fn("push", push_column_i64)
        .register_fn("push", push_column_ce)
        .register_fn("enable_selector", enable_selector)
        .register_fn("lookup", lookup)
        .register_fn("lookup", lookup_without_name)
        //-
        ;
}

// a <== b
fn assign_constraint(a: &mut Cell, b: Cell) -> Cell {
    // println!("assign_constraint({:#?}, {:#?})", a, b);
    a.value = b.value.clone();
    push_instruction_to_last_region(match (a.column.ctype, b.column.ctype) {
        (ColumnType::Advice, ColumnType::Instance) => {
            vec![Instruction::AssignAdviceFromInstance(a.clone(), b.clone())]
        }
        (ColumnType::Instance, ColumnType::Advice) => {
            vec![Instruction::AssignAdviceFromInstance(b.clone(), a.clone())]
        }
        (ColumnType::Advice, ColumnType::Advice) => {
            vec![
                Instruction::AssignAdvice(a.clone(), CellExpression::CellValue(b.clone())),
                Instruction::ConstrainEqual(a.clone(), b.clone()),
            ]
        }
        (_, _) => todo!(),
    });
    a.clone()
}

// a <== b (b is expresion, e.g. b1 + b2)
fn assign_constraint_cell_ce(a: &mut Cell, b: CellExpression) -> Cell {
    // println!("assign_constraint({:?}, {:?})", a, b);
    a.value = b.to_value_string();
    push_instruction_to_last_region(vec![Instruction::AssignAdvice(a.clone(), b.clone())]);

    // set gate
    let (selector, index) = upsert_gate(
        None, // TODO: gate name should come from code
        CellExpression::Sum(
            Box::new(CellExpression::Negated(Box::new(
                a.clone().to_cell_expression(),
            ))),
            Box::new(b.clone()),
        ),
    )
    .unwrap();

    // enable selector
    let enable = Cell {
        column: selector.clone(),
        name: get_field_name(&selector, index),
        value: Some("1".to_string()),
        index: index,
    };
    if let Some(region) = unsafe { CONTEXT.regions.last_mut() } {
        region
            .instructions
            .push(Instruction::EnableSelector(enable.clone()));
    }

    a.clone()
}

fn upsert_gate(
    name: Option<String>,
    exp: CellExpression,
) -> Result<(Column, i64), Box<EvalAltResult>> {
    // insert gate if not exists
    // if exists, just ignore

    // println!("upsert_gate({:#?})", exp);
    let base_index = exp.get_base_index();
    let exp = if base_index > 0 {
        exp.to_base_index(base_index)
    } else {
        exp
    };
    check_gate_ce(&exp)?;
    let exp_str = exp.to_string();
    unsafe {
        let gate = CONTEXT.gates.iter().find(|(_, n, _, _)| n == &exp_str);
        match gate {
            Some((_, _, col, _)) => {
                return Ok((col.clone(), base_index));
            }
            None => {}
        }
    }

    let name = match name {
        Some(n) => n,
        None => format!("gate_{}", unsafe { CONTEXT.gates.len() }),
    };

    let selector = Column {
        name: name.to_string(),
        ctype: ColumnType::Selector,
        stype: SpecialType::None,
    };
    let result = CellExpression::Product(
        Box::new(CellExpression::CellValue(selector.clone().get_field(0))),
        Box::new(exp),
    );

    unsafe {
        CONTEXT
            .gates
            .push((name, exp_str, selector.clone(), result));
        CONTEXT.columns.push(selector.clone());
    }

    Ok((selector, base_index))
}

// a === b
fn constrain_equal(a: &mut Cell, b: Cell) {
    // println!("constrain_equal({:#?}, {:#?})", a, b);
    push_instruction_to_last_region(vec![Instruction::ConstrainEqual(a.clone(), b.clone())]);
}

fn assign_constraint_string(a: &mut Cell, b: String) -> Cell {
    // println!("assign_constraint({:?}, {:?})", a, b);
    let cb = CellExpression::Constant(b);
    a.value = cb.to_value_string();
    push_instruction_to_last_region(vec![Instruction::AssignAdvice(a.clone(), cb)]);
    a.clone()
}

fn assign_common_string(a: &mut Cell, b: String) -> Result<Cell, Box<EvalAltResult>> {
    match a.column.ctype {
        ColumnType::Fixed => {
            let cb = CellExpression::Constant(b);
            a.value = cb.to_value_string();
            push_instruction_to_last_region(vec![Instruction::AssignFixed(a.clone(), cb)]);
            Ok(a.clone())
        }
        ColumnType::Instance => {
            let cb = CellExpression::Constant(b);
            a.value = cb.to_value_string();
            Ok(a.clone())
            //warning
        }
        ColumnType::Advice => {
            let cb = CellExpression::Constant(b);
            a.value = cb.to_value_string();
            push_instruction_to_last_region(vec![Instruction::AssignAdvice(a.clone(), cb)]);
            Ok(a.clone())
        }
        o => Err(format!("unsupported column type {:?}", o).into()),
    }
}

fn assign_common_ce(a: &mut Cell, b: CellExpression) -> Result<Cell, Box<EvalAltResult>> {
    match a.column.ctype {
        ColumnType::Fixed => {
            a.value = b.to_value_string();
            push_instruction_to_last_region(vec![Instruction::AssignFixed(a.clone(), b)]);
            Ok(a.clone())
        }
        ColumnType::Advice => {
            a.value = b.to_value_string();
            push_instruction_to_last_region(vec![Instruction::AssignAdvice(a.clone(), b)]);
            Ok(a.clone())
        }
        o => todo!("{:?}", o),
    }
}

fn assign_common_i64(a: &mut Cell, b: i64) -> Result<Cell, Box<EvalAltResult>> {
    assign_common_string(a, b.to_string())
}

fn push_column_i64(a: &mut Column, b: i64) -> Result<(), Box<EvalAltResult>> {
    push_column(a, b.to_string())
}

fn push_column_ce(a: &mut Column, b: CellExpression) -> Result<(), Box<EvalAltResult>> {
    push_column(a, b.to_value_string().unwrap())
}

fn push_column(a: &mut Column, b: String) -> Result<(), Box<EvalAltResult>> {
    match a.ctype {
        ColumnType::TableLookup => {
            let ins = vec![Instruction::AssignCell(a.clone(), b)];

            let table = unsafe {
                let table = CONTEXT.tables.iter_mut().find(|n| n.name == a.name);
                match table {
                    Some(t) => t,
                    None => {
                        let ib = InstructionBundle {
                            id: CONTEXT.tables.len() as i64,
                            name: a.name.clone(),
                            instructions: vec![],
                        };
                        CONTEXT.tables.push(ib);
                        let table = CONTEXT.tables.iter_mut().find(|n| n.name == a.name);
                        match table {
                            Some(t) => t,
                            None => panic!("abnormal! table not found after inserted"),
                        }
                    }
                }
            };
            for i in ins {
                table.instructions.push(i);
            }

            Ok(())
        }
        o => todo!("{:?}", o),
    }
}

fn push_instruction_to_last_region(a: Vec<Instruction>) {
    if let Some(region) = unsafe { CONTEXT.regions.last_mut() } {
        for i in a {
            region.instructions.push(i);
        }
    }
}

fn enable_selector(a: &mut Cell) {
    // println!("enable_selector({:?})", a);
    a.value = Some("1".to_string());
    if let Some(region) = unsafe { CONTEXT.regions.last_mut() } {
        region
            .instructions
            .push(Instruction::EnableSelector(a.clone()));
    }
}

// fn lookup(ces: Vec<CellExpression>, cols: Vec<Column>) -> Result<(), Box<EvalAltResult>> {
fn lookup_without_name(ces: Array, cols: Array) -> Result<(), Box<EvalAltResult>> {
    lookup("default".to_string(), ces, cols)
}

fn lookup(name: String, ces: Array, cols: Array) -> Result<(), Box<EvalAltResult>> {
    let ces = ces
        .into_iter()
        .map(|x| x.try_cast::<CellExpression>().unwrap())
        .collect::<Vec<CellExpression>>();
    let cols = cols
        .into_iter()
        .map(|x| x.try_cast::<Column>().unwrap())
        .collect::<Vec<Column>>();
    if ces.len() != cols.len() {
        return Err("ces and cols length not match".into());
    }

    let mut map = Vec::<(CellExpression, Column)>::new();
    for (ce, col) in ces.into_iter().zip(cols.into_iter()) {
        map.push((ce, col));
    }

    unsafe {
        CONTEXT.lookups.push(LookupParameter { name, map });
    }
    Ok(())
}
