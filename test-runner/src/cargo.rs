use std::env;
use std::process::{Command, Output};
use std::path::PathBuf;

use crate::error::{Error, Result};

pub fn target_dir() -> Result<PathBuf> {
    let mut target_dir = env::current_dir()?;
    target_dir.pop(); // chop off our crate name
    target_dir.push("target");
    assert!(target_dir.exists());
    Ok(target_dir)
}

fn cargo() -> Result<Command> {
    let mut cmd = Command::new("cargo");
    let target_dir = target_dir()?;
    cmd.current_dir(target_dir.join("tests"))
        .env("CARGO_TARGET_DIR", &target_dir);
    Ok(cmd)
}

pub fn build_dependencies(project: &str) -> Result<()> {
    let status = cargo()?
        .arg("build")
        .arg("--bin")
        .arg(project)
        .status()
        .map_err(Error::Cargo)?;

    if status.success() {
        Ok(())
    } else {
        Err(Error::CargoFail)
    }
}

pub fn build_test(name: &str) -> Result<Output> {
    if let Ok(crate_name) = env::var("CARGO_PKG_NAME") {
        let project = format!("{}-tests", crate_name);
        let _ = cargo()?
            .arg("clean")
            .arg("--package")
            .arg(project)
            .arg("--color=never")
            .status();
    }

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
