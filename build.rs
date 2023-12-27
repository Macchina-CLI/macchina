use std::env;
use std::error::Error;
use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    let target = env::var("TARGET").unwrap();

    if !target.contains("netbsd") {
        EmitBuilder::builder().emit()?;
    }

    Ok(())
}
