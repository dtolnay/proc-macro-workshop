use serde::Deserialize;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

use crate::error::{Error, Result};
use crate::run::Project;

fn raw_cargo() -> Command {
    Command::new(option_env!("CARGO").unwrap_or("cargo"))
}

fn cargo(project: &Project) -> Command {
    let mut cmd = raw_cargo();
    cmd.current_dir(&project.dir);
    cmd.env("CARGO_TARGET_DIR", &project.target_dir);
    cmd
}

pub fn build_dependencies(project: &Project) -> Result<()> {
    let status = cargo(project)
        .arg("build")
        .arg("--bin")
        .arg(&project.name)
        .status()
        .map_err(Error::Cargo)?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::CargoFail)
    }
}

pub fn build_test(project: &Project, name: &str) -> Result<Output> {
    let _ = cargo(project)
        .arg("clean")
        .arg("--package")
        .arg(&project.name)
        .arg("--color=never")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    cargo(project)
        .arg("build")
        .arg("--bin")
        .arg(name)
        .arg("--quiet")
        .arg("--color=never")
        .output()
        .map_err(Error::Cargo)
}

pub fn run_test(project: &Project, name: &str) -> Result<Output> {
    cargo(project)
        .arg("run")
        .arg("--bin")
        .arg(name)
        .arg("--quiet")
        .arg("--color=never")
        .output()
        .map_err(Error::Cargo)
}

pub fn target_dir() -> Result<PathBuf> {
    #[derive(Deserialize)]
    struct Metadata {
        target_directory: PathBuf,
    }

    let output = raw_cargo()
        .arg("metadata")
        .arg("--format-version=1")
        .output()
        .map_err(Error::Cargo)?;

    let metadata: Metadata = serde_json::from_slice(&output.stdout).map_err(Error::Metadata)?;

    Ok(metadata.target_directory)
}
