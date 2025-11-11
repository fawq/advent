use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    let stub = algo::stub_info()?;
    stub.generate()?;
    Ok(())
}
