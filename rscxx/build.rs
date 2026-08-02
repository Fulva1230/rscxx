fn main() -> anyhow::Result<()> {
    let _ = cxx_build::bridge("src/lib.rs").compile("rscxx");

    Ok(())
}
