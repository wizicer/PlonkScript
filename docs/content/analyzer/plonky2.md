You can try the [analyzer](/play/index.html#/analyzer/plonky2){target="_blank"} now.

To start analysis, inject the code like this to get the trace for analysis:

```rust{5-13}
let data = builder.build::<C>();
let proof = data.prove(pw)?; // [!code --]
let proof = data.prove(pw.clone())?; // [!code ++]
 // [!code ++]
let partition_witness = // [!code ++]
    plonky2::iop::generator::generate_partial_witness(pw, &data.prover_only, &data.common); // [!code ++]
 // [!code ++]
let _witness = format!("{:#?}", partition_witness); // [!code ++]
let _data = format!("{:#?}", data); // [!code ++]
let _proof = format!("{:#?}", proof); // [!code ++]
let _output = format!("Plonky2Data {{\nwitness: {},\ndata: {},\nproof: {}\n}}", _witness, _data, _proof); // [!code ++]
let mut file = std::fs::File::create("output.rust").unwrap(); // [!code ++]
std::io::Write::write_all(&mut file, _output.as_bytes()).unwrap(); // [!code ++]
 // [!code ++]
data.verify(proof) // [!code ++]
```

::: tip
ONLY if you work with older version plonky2, you need following changes, latest plonky2 already includes these changes.
:::

Modify `PartitionWitness` in `plonky2/src/iop/witness.rs` to include Debug, like this:

```rust
#[derive(Clone)] // [!code --]
#[derive(Clone, Debug)] // [!code ++]
pub struct PartitionWitness<'a, F: Field> {
    pub values: Vec<Option<F>>,
    ...
} 
```
