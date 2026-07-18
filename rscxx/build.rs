use std::env;
use std::path::PathBuf;
use anyhow::Context;
use std::process::Command;

fn recursive_copy_header<P: AsRef<std::path::Path>>(from: P, dest: P) -> anyhow::Result<()> {
    for file in walkdir::WalkDir::new(&from).into_iter().flatten() {
        if file.file_name().to_str().unwrap().ends_with(".h") {
            let dest_file_path  = dest.as_ref().join(file.path().strip_prefix(&from)?);
            std::fs::create_dir_all(dest_file_path.parent().context("no parent")?)?;
            std::fs::copy(file.path(), dest_file_path)?;
        }
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let _ = cxx_build::bridge("src/lib.rs").compile("rscxx");

    let out_dir = env::var("OUT_DIR").map(PathBuf::from)?;

    let dist_dir = env::var("DIST_DIR");
    if let Ok(dist_dir) = dist_dir {
        let dist_dir = std::path::Path::new(&dist_dir);
        recursive_copy_header(out_dir.join("cxxbridge/include"), dist_dir.join("include"))?;
        recursive_copy_header(out_dir.join("cxxbridge/crate"), dist_dir.join("include"))?;
    }

    Ok(())
}
