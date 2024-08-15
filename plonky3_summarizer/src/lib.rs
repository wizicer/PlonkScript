use p3_field::Field;
use p3_matrix::dense::DenseMatrix;
use p3_uni_stark::{Entry, SymbolicExpression};
use serde_json::{json, Value};
use std::fs::File;
use std::io::Write;

fn symbol_to_json<F: Field>(symbol: &SymbolicExpression<F>) -> Value {
    match symbol {
        SymbolicExpression::Variable(var) => json!({
            "type": "Variable",
            "entry": match &var.entry {
                Entry::Preprocessed { offset } => json!({
                    "type": "Preprocessed",
                    "offset": offset,
                }),
                Entry::Main { offset } => json!({
                    "type": "Main",
                    "offset": offset,
                }),
                Entry::Permutation { offset } => json!({
                    "type": "Permutation",
                    "offset": offset,
                }),
                Entry::Public => json!({
                    "type": "Public",
                }),
                Entry::Challenge => json!({
                    "type": "Challenge",
                }),
            },
            "index": var.index,
        }),
        SymbolicExpression::IsFirstRow => json!({ "type": "IsFirstRow" }),
        SymbolicExpression::IsLastRow => json!({ "type": "IsLastRow" }),
        SymbolicExpression::IsTransition => json!({ "type": "IsTransition" }),
        SymbolicExpression::Constant(val) => json!({
            "type": "Constant",
            "value": val,
        }),
        SymbolicExpression::Add {
            x,
            y,
            degree_multiple,
        } => json!({
            "type": "Add",
            "x": symbol_to_json(&x),
            "y": symbol_to_json(&y),
            "degree_multiple": degree_multiple,
        }),
        SymbolicExpression::Sub {
            x,
            y,
            degree_multiple,
        } => json!({
            "type": "Sub",
            "x": symbol_to_json(&x),
            "y": symbol_to_json(&y),
            "degree_multiple": degree_multiple,
        }),
        SymbolicExpression::Neg { x, degree_multiple } => json!({
            "type": "Neg",
            "x": symbol_to_json(&x),
            "degree_multiple": degree_multiple,
        }),
        SymbolicExpression::Mul {
            x,
            y,
            degree_multiple,
        } => json!({
            "type": "Mul",
            "x": symbol_to_json(&x),
            "y": symbol_to_json(&y),
            "degree_multiple": degree_multiple,
        }),
    }
}

fn all_to_json<F: Field>(
    symbols: &Vec<SymbolicExpression<F>>,
    trace: &DenseMatrix<F>,
    public_values: &Vec<F>,
) -> Value {
    json!({
        "symbols": symbols.iter().map(|symbol| symbol_to_json(symbol)).collect::<Vec<_>>(),
        "trace": {
            "values": trace.values,
            "width": trace.width,
        },
        "public": public_values,
    })
}

pub fn save_as_json<F: Field, A>(
    chip: &A,
    trace: &DenseMatrix<F>,
    public_values: &Vec<F>,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>>
where
    A: p3_air::Air<p3_uni_stark::SymbolicAirBuilder<F>>,
{
    let syms = p3_uni_stark::get_symbolic_constraints::<F, A>(&chip, 0, public_values.len());

    let mut file = File::create(file_name)?;
    let json_data = all_to_json(&syms, trace, public_values);
    write!(&mut file, "{}", json_data)?;
    Ok(())
}
