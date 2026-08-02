use anyhow::Context;
use cargo_metadata::camino::{Utf8Path, Utf8PathBuf};
use clap::Parser;
use std::env;
use std::process::Command;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg()]
    cmd: String,

    #[arg(short, long, default_value = "dist")]
    output_dir: Utf8PathBuf,
}
fn copy_files(
    from: impl AsRef<Utf8Path>,
    into: impl AsRef<Utf8Path>,
    glob_pattern: &str,
    preserve_structure: bool,
) -> anyhow::Result<()> {
    let from = from.as_ref();
    let into = into.as_ref();
    let glob_pattern = glob::Pattern::new(glob_pattern)?;
    for file in WalkDir::new(from).into_iter().flatten() {
        let file_path = Utf8Path::from_path(file.path()).context("path is not utf8")?;
        let file_subpath = file_path.strip_prefix(from)?;
        if glob_pattern.matches_path(file_subpath.as_std_path()) {
            let dest_file_path = if preserve_structure {
                into.join(file_subpath)
            } else {
                into.join(file_path.file_name().context("no filename")?)
            };
            if let Some(parent) = dest_file_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
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
                        copy_files(&build.out_dir, &dist.join("lib"), "**/*.lib", false)?;
                    } else if build
                        .linked_libs
                        .iter()
                        .any(|lib| lib.as_str().contains("rscxx"))
                    {
                        copy_files(&build.out_dir, &dist.join("lib"), "**/*.lib", false)?;
                        copy_files(&build.out_dir.join("cxxbridge/crate"), &dist.join("include"), "**/*.h", true)?;
                        copy_files(&build.out_dir.join("cxxbridge/include"), &dist.join("include"), "**/*.h", true)?;
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
