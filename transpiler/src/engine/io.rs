use crate::system::*;
use crate::CONTEXT;
use crate::system::cell_expression::ToValueString;

pub const DEFAULT_INSTANCE_COLUMN_NAME: &str = "defins";

pub fn register_io(engine: &mut rhai::Engine) {
    let _ = &mut engine
        .register_fn("init_input", init_input)
        .register_fn("init_output", init_output)
        .register_fn("set_output", set_output)
        .register_fn("init_advice_column", init_advice_column)
        .register_fn("init_fixed_column", init_fixed_column)
        .register_fn("set_parameter", set_parameter)
        .register_fn("set_parameter", set_parameter_i64)
        .register_fn("inspect", inspect)
        .register_fn("inspect", inspect_ce)
        .register_fn("inspect", inspect_str);
}

fn get_lastest_instance_index() -> i64 {
    let a = unsafe { CONTEXT.instance_count };
    unsafe { CONTEXT.instance_count += 1 };
    a
}

fn init_input(v: &str) -> Cell {
    // println!("init_input({})", v);
    let cell = Cell {
        name: v.to_string(),
        index: get_lastest_instance_index(),
        value: unsafe { Some(CONTEXT.inputs[v].clone()) },
        column: Column {
            name: DEFAULT_INSTANCE_COLUMN_NAME.to_string(),
            ctype: ColumnType::Instance,
            stype: SpecialType::Input,
        },
    };
    unsafe {
        CONTEXT.signals.push(cell.clone());
    }
    cell
}
fn init_output(v: String) -> Cell {
    // println!("init_output({})", v);
    let cell = Cell {
        name: v.to_string(),
        index: get_lastest_instance_index(),
        value: None,
        column: Column {
            name: DEFAULT_INSTANCE_COLUMN_NAME.to_string(),
            ctype: ColumnType::Instance,
            stype: SpecialType::Output,
        },
    };
    unsafe {
        CONTEXT.signals.push(cell.clone());
    }
    cell
}
fn set_output(name: String, cell: Cell) {
    unsafe {
        if let Some(pos) = CONTEXT.signals.iter().position(|x| x.name == name) {
            CONTEXT.signals.splice(pos..(pos + 1), vec![cell]);
        }
    }
}
fn init_advice_column(v: String) -> Column {
    // println!("init_advice_column({})", v);
    let col = Column {
        name: v.to_string(),
        ctype: ColumnType::Advice,
        stype: SpecialType::None,
    };
    unsafe {
        CONTEXT.columns.push(col.clone());
    }
    col
}

fn init_fixed_column(v: String) -> Column {
    // println!("init_fixed_column({})", v);
    let col = Column {
        name: v.to_string(),
        ctype: ColumnType::Fixed,
        stype: SpecialType::None,
    };
    unsafe {
        CONTEXT.columns.push(col.clone());
    }
    col
}

fn set_parameter_i64(name: String, v: i64) {
    set_parameter(name, v.to_string())
}

fn set_parameter(name: String, v: String) {
    unsafe {
        CONTEXT.inputs.insert(name, v);
    }
}

fn inspect(obj: Cell) {
    println!("{:#?}", obj);
}

fn inspect_ce(obj: CellExpression) {
    println!("{:#?}", obj.to_value_string());
}

fn inspect_str(obj: String) {
    println!("{:#?}", obj);
}
