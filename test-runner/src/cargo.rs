use std::env;
use std::process::{Command, Output, Stdio};

use crate::error::{Error, Result};
use crate::run::Project;

fn cargo() -> Result<Command> {
    let mut cmd = Command::new("cargo");
    cmd.current_dir("../target/tests");
    cmd.env("CARGO_TARGET_DIR", "..");
    Ok(cmd)
}

pub fn build_dependencies(project: &Project) -> Result<()> {
    let status = cargo()?
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

pub fn build_test(name: &str) -> Result<Output> {
    let crate_name = env::var("CARGO_PKG_NAME").map_err(Error::PkgName)?;
    let project = format!("{}-tests", crate_name);
    let _ = cargo()?
        .arg("clean")
        .arg("--package")
        .arg(project)
        .arg("--color=never")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    cargo()?
        .arg("build")
        .arg("--bin")
        .arg(name)
        .arg("--quiet")
        .arg("--color=never")
        .output()
        .map_err(Error::Cargo)
}

pub fn run_test(name: &str) -> Result<Output> {
    cargo()?
        .arg("run")
        .arg("--bin")
        .arg(name)
        .arg("--quiet")
        .arg("--color=never")
        .output()
        .map_err(Error::Cargo)
}
