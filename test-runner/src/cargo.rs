use std::process::{Command, Output, Stdio};

use crate::error::{Error, Result};
use crate::run::Project;

fn cargo(project: &Project) -> Command {
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&project.dir);
    cmd.env("CARGO_TARGET_DIR", "..");
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
