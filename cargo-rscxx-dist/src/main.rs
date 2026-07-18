use anyhow::Context;
use clap::Parser;
use std::env;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    cmd: String,

    #[arg(short, long, default_value = "dist")]
    output_dir: PathBuf,
}
fn recursive_copy_header(
    from: impl AsRef<std::path::Path>,
    dest: impl AsRef<std::path::Path>,
) -> anyhow::Result<()> {
    for file in walkdir::WalkDir::new(&from).into_iter().flatten() {
        if file.path().extension().is_some_and(|e| e == "h") {
            let dest_file_path = dest.as_ref().join(file.path().strip_prefix(&from)?);
            std::fs::create_dir_all(dest_file_path.parent().context("no parent")?)?;
            std::fs::copy(file.path(), dest_file_path)?;
        }
    }
    Ok(())
}
fn copy_lib_or_dll(
    from: impl AsRef<std::path::Path>,
    dest: impl AsRef<std::path::Path>,
) -> anyhow::Result<()> {
    for file in walkdir::WalkDir::new(&from).into_iter().flatten() {
        if file
            .path()
            .extension()
            .is_some_and(|e| e == "lib" || e == "dll")
        {
            let dest_file_path = dest.as_ref().join(file.file_name());
            std::fs::copy(file.path(), dest_file_path)?;
        }
    }
    Ok(())
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let dist = args.output_dir;
    std::fs::create_dir_all(&dist)?;

    let cargo_cmd = env::var("CARGO")?;
    let build_res = Command::new(cargo_cmd)
        .args([
            "build",
            "--release",
            "--package",
            "rscxx",
            "--message-format=json",
        ])
        .output()?;

    for message in cargo_metadata::Message::parse_stream(build_res.stdout.as_slice()) {
        if let Ok(message) = message {
            match message {
                cargo_metadata::Message::CompilerArtifact(artifact) => {
                    if artifact.target.name == "rscxx" {
                        for file in artifact.filenames.iter().filter(|file| {
                            file.extension().is_some_and(|e| e == "lib" || e == "dll")
                        }) {
                            std::fs::copy(
                                file,
                                dist.join(file.file_name().context("file name extract failed")?),
                            )?;
                        }
                    }
                }
                cargo_metadata::Message::CompilerMessage(_) => {}
                cargo_metadata::Message::BuildScriptExecuted(build) => {
                    if build
                        .linked_libs
                        .iter()
                        .any(|lib| lib.as_str().contains("cxxbridge1"))
                    {
                        copy_lib_or_dll(&build.out_dir, &dist)?;
                    } else if build
                        .linked_libs
                        .iter()
                        .any(|lib| lib.as_str().contains("rscxx"))
                    {
                        copy_lib_or_dll(&build.out_dir, &dist)?;
                        recursive_copy_header(
                            build.out_dir.join("cxxbridge/crate"),
                            dist.join("include"),
                        )?;
                        recursive_copy_header(
                            build.out_dir.join("cxxbridge/include"),
                            dist.join("include"),
                        )?;
                    }
                }
                cargo_metadata::Message::BuildFinished(_) => {}
                cargo_metadata::Message::TextLine(_) => {}
                _ => {}
            }
        }
    }

    Ok(())
}
