use crate::generator::util::ToQuoteField;
use crate::system::SimplifiedConstraitSystem;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub fn get_test(circuit_name: &str, cs: &SimplifiedConstraitSystem) -> TokenStream {
    let circuit_name = format_ident!("{}", circuit_name);
    let k = cs
        .inputs
        .get("k")
        .or(Some(&"8".to_string()))
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let public_input = cs
        .signals
        .clone()
        .into_iter()
        .map(|x| match x.value {
            Some(x) => x
                .to_quote_field()
                .expect(format!("Decoding failed: {x}").as_str()),
            None => panic!("No value for signal [{}]", x.name),
        })
        .collect::<Vec<TokenStream>>();

    quote! {
        #[cfg(test)]
        mod tests {
            use super::*;
            use halo2_proofs::{dev::MockProver, pasta::Fp as F};

            #[test]
            fn test_simple() {
                let circuit = #circuit_name {
                    _marker: std::marker::PhantomData,
                };

                let prover = MockProver::run(#k, &circuit, vec![vec![#(#public_input),*]]).unwrap();
                if cfg!(debug_assertions) {
                    let d = format!("{:#?}", prover);
                    let mut file = std::fs::File::create("visualization.rust").unwrap();
                    std::io::Write::write_all(&mut file, d.as_bytes()).unwrap();
                }

                prover.assert_satisfied();
            }
        }
    }
}
