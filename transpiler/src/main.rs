use std::fs;

use rhai::EvalAltResult;
use transpiler::try_run;

#[allow(unreachable_code)]
pub fn main() -> Result<(), Box<EvalAltResult>> {
    // std::panic::set_hook(Box::new(|panic_info| {
    //     let backtrace = std::backtrace::Backtrace::capture();
    //     eprintln!("My backtrace: {:#?}", backtrace);
    // }));

    let code = fs::read_to_string("fibonacci.plonk").expect("read file failed");
    let output = try_run(code);
    match output {
        Ok(_) => (),
        Err(e) => {
            println!("Script Error: {:#?}", e);
        }
    }
    // let d = halo2_summarizer::trim(&output, Some(0..16));
    // let d = halo2_summarizer::trim(&output);
    // let d = output;

    // let mut file = std::fs::File::create("visualization.rust").unwrap();
    // std::io::Write::write_all(&mut file, d.as_bytes()).unwrap();

    Ok(())
}
