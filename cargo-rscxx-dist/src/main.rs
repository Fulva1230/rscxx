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
fn copy_files_with_extension(
    from: impl AsRef<std::path::Path>,
    dest: impl AsRef<std::path::Path>,
    extension: &str,
) -> anyhow::Result<()> {
    for file in walkdir::WalkDir::new(&from).into_iter().flatten() {
        if file.path().extension().is_some_and(|e| e == extension) {
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
    std::fs::create_dir_all(dist.join("bin"))?;
    std::fs::create_dir_all(dist.join("lib"))?;

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
                        for (ext, file) in artifact.filenames.iter().filter_map(|file| {
                            if let Some(ext) = file.extension() {
                                Some((ext, file))
                            } else {
                                None
                            }
                        }) {
                            let dest =
                                if ext == "lib" {
                                    Some(dist.join("lib").join(
                                        file.file_name().context("file name extract failed")?,
                                    ))
                                } else if ext == "dll" {
                                    Some(dist.join("bin").join(
                                        file.file_name().context("file name extract failed")?,
                                    ))
                                } else {
                                    None
                                };
                            if let Some(dest) = dest {
                                std::fs::copy(file, dest)?;
                            }
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
                        copy_files_with_extension(&build.out_dir, dist.join("lib"), "lib")?;
                    } else if build
                        .linked_libs
                        .iter()
                        .any(|lib| lib.as_str().contains("rscxx"))
                    {
                        copy_files_with_extension(&build.out_dir, dist.join("lib"), "lib")?;
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
