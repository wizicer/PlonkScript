use crate::system::*;
use crate::CONTEXT;

pub fn register_custom_type(engine: &mut rhai::Engine) {
    let _ = &mut engine
        .register_type_with_name::<Column>("Column")
        .register_indexer_get(Column::get_field)
        .register_indexer_set(Column::set_field)
        .register_type_with_name::<Cell>("Cell")
        .register_get("value", Cell::get_value);
}

impl Column {
    pub fn get_field(&mut self, index: i64) -> Cell {
        let name = get_field_name(&self, index);
        unsafe {
            if CONTEXT.cells.contains_key(&name) {
                return CONTEXT.cells[&name].clone();
            }
        }

        Cell {
            name,
            index,
            value: Some("0".to_string()),
            column: self.clone(),
        }
    }
    pub fn set_field(&mut self, index: i64, value: Cell) {
        let name = get_field_name(&self, index);
        let cell = Cell {
            name: name.clone(),
            index,
            value: value.value,
            ..value
        };
        unsafe {
            let entry = CONTEXT.cells.entry(name).or_insert(cell.clone()); //.and_modify(||cell);
            *entry = cell;
        }
    }
}

impl Cell {
    fn get_value(&mut self) -> String {
        self.value.clone().unwrap()
    }
}

pub fn get_field_name(col: &Column, index: i64) -> String {
    let region = unsafe { CONTEXT.regions.last().unwrap().clone() };
    format!("{}[{}]_{}_{}", col.name, index, region.name, region.id)
}