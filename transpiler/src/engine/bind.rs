use rhai::EvalAltResult;

use self::cell_expression::ToString;
use crate::engine::gate::check_gate_ce;
use crate::system::cell_expression::ToValueString;
use crate::system::*;
use crate::CONTEXT;

pub fn register_bind(engine: &mut rhai::Engine) {
    let _ = &mut engine
        .register_fn("assign_constraint", assign_constraint)
        .register_fn("assign_constraint", assign_constraint_cell_ce)
        .register_fn("assign_constraint", assign_constraint_string)
        .register_fn("assign_common", assign_common_string)
        .register_fn("assign_common", assign_common_ce)
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
    let selector = upsert_gate(
        None, // TODO: gate name should come from code
        CellExpression::Sum(
            Box::new(CellExpression::Negated(Box::new(
                a.clone().to_cell_expression(),
            ))),
            Box::new(b.clone()),
        ),
    );

    // enable selector
    let enable = Cell {
        column: selector.unwrap(),
        name: "".to_string(),
        value: Some("1".to_string()),
        index: 0,
    };
    if let Some(region) = unsafe { CONTEXT.regions.last_mut() } {
        region
            .instructions
            .push(Instruction::EnableSelector(enable.clone()));
    }

    a.clone()
}

fn upsert_gate(name: Option<String>, exp: CellExpression) -> Result<Column, Box<EvalAltResult>> {
    // insert gate if not exists
    // if exists, just ignore

    // println!("upsert_gate({:#?})", exp);
    check_gate_ce(&exp)?;
    let exp_str = exp.to_string();
    unsafe {
        let gate = CONTEXT.gates.iter().find(|(_, n, _, _)| n == &exp_str);
        match gate {
            Some((_, _, col, _)) => {
                return Ok(col.clone());
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

    Ok(selector)
}

fn assign_constraint_string(a: &mut Cell, b: String) -> Cell {
    // println!("assign_constraint({:?}, {:?})", a, b);
    let cb = CellExpression::Constant(b);
    a.value = cb.to_value_string();
    push_instruction_to_last_region(vec![Instruction::AssignAdvice(a.clone(), cb)]);
    a.clone()
}

fn assign_common_string(a: &mut Cell, b: String) -> Cell {
    match a.column.ctype {
        ColumnType::Fixed => {
            let cb = CellExpression::Constant(b);
            a.value = cb.to_value_string();
            push_instruction_to_last_region(vec![Instruction::AssignFixed(a.clone(), cb)]);
            a.clone()
        }
        ColumnType::Instance => {
            let cb = CellExpression::Constant(b);
            a.value = cb.to_value_string();
            a.clone()
            //warning
        }
        o => todo!("{:?}", o),
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

fn push_instruction_to_last_region(a: Vec<Instruction>) {
    if let Some(region) = unsafe { CONTEXT.regions.last_mut() } {
        for i in a {
            region.instructions.push(i);
        }
    }
}
