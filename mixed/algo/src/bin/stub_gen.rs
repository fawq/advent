use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    let stub = main_mod::stub_info()?;
    stub.generate()?;
    Ok(())
}
