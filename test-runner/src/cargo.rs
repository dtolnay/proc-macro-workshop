use std::env;
use std::process::{Command, Output};

use crate::error::{Error, Result};

pub fn build_dependencies(project: &str) -> Result<()> {
    let status = Command::new("cargo")
        .current_dir("target/tests")
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
        let _ = Command::new("cargo")
            .current_dir("target/tests")
            .arg("clean")
            .arg("--package")
            .arg(project)
            .arg("--color=never")
            .status();
    }

    Command::new("cargo")
        .current_dir("target/tests")
        .arg("build")
        .arg("--bin")
        .arg(name)
        .arg("--quiet")
        .arg("--color=never")
        .output()
        .map_err(Error::Cargo)
}

pub fn run_test(name: &str) -> Result<Output> {
    Command::new("cargo")
        .current_dir("target/tests")
        .arg("run")
        .arg("--bin")
        .arg(name)
        .arg("--quiet")
        .arg("--color=never")
        .output()
        .map_err(Error::Cargo)
}
