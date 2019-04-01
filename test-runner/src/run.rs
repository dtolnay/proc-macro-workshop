use std::collections::BTreeMap as Map;
use std::env;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::path::Path;

use super::{Expected, Runner, Test};
use crate::banner;
use crate::cargo;
use crate::error::{Error, Result};
use crate::manifest::{Bin, Dependency, Edition, Manifest, Package, Config, Build, Workspace};
use crate::message;
use crate::normalize;

const IGNORED_LINTS: &[&str] = &["dead_code"];

impl Runner {
    pub fn run(&mut self) {
        if let Err(err) = self.prepare() {
            message::prepare_fail(err);
            panic!("tests failed");
        }

        println!();
        banner::colorful();

        let mut failures = 0;

        if self.tests.is_empty() {
            message::no_tests_enabled();
        } else {
            for test in &self.tests {
                if let Err(err) = test.run() {
                    failures += 1;
                    message::test_fail(err);
                }
            }
        }

        banner::colorful();
        println!();

        if failures > 0 {
            panic!("{} of {} tests failed", failures, self.tests.len());
        }
    }

    fn prepare(&self) -> Result<()> {
        let crate_name = env::var("CARGO_PKG_NAME").map_err(Error::PkgName)?;
        let project = format!("{}-tests", crate_name);

        let manifest = self.make_manifest(crate_name, &project)?;
        let manifest_toml = toml::to_string(&manifest)?;

        let config = self.make_config();
        let config_toml = toml::to_string(&config)?;

        let target_dir = cargo::target_dir()?;
        fs::create_dir_all(target_dir.join("tests/.cargo"))?;
        fs::write(target_dir.join("tests/Cargo.toml"), manifest_toml)?;
        fs::write(target_dir.join("tests/.cargo/config"), config_toml)?;
        fs::write(target_dir.join("tests/main.rs"), b"fn main() {}\n")?;

        cargo::build_dependencies(&project)
    }

    fn make_manifest(&self, crate_name: String, project: &str) -> Result<Manifest> {
        let mut manifest = Manifest {
            package: Package {
                name: project.to_owned(),
                version: "0.0.0".to_owned(),
                edition: Edition::E2018,
                publish: false,
            },
            dependencies: Map::new(),
            bins: Vec::new(),
            workspace: Some(Workspace {}),
        };

        let cwd = env::current_dir()?;

        manifest.dependencies.insert(
            crate_name,
            Dependency {
                version: None,
                path: Some(cwd.display().to_string()),
                features: Vec::new(),
            },
        );

        // If your test cases require additional dependencies, I guess add them
        // here.
        //
        // TODO: Maybe expose additional dependencies as an API on TestCases:
        //
        //     t.dependency("serde", "1.0");
        //

        manifest.bins.push(Bin {
            name: project.to_owned(),
            path: Path::new("main.rs").to_owned(),
        });

        for test in &self.tests {
            manifest.bins.push(Bin {
                name: test.name(),
                path: cwd.join(&test.path),
            });
        }

        Ok(manifest)
    }

    fn make_config(&self) -> Config {
        let mut rustflags = Vec::new();

        for &lint in IGNORED_LINTS {
            rustflags.push("-A".to_owned());
            rustflags.push(lint.to_owned());
        }

        Config {
            build: Build { rustflags },
        }
    }
}

impl Test {
    fn name(&self) -> String {
        self.path
            .file_stem()
            .unwrap_or_else(|| self.path.as_os_str())
            .to_owned()
            .to_string_lossy()
            .replace('-', "_")
    }

    fn run(&self) -> Result<()> {
        message::begin_test(self);
        check_exists(&self.path)?;

        let name = self.name();
        let output = cargo::build_test(&name)?;
        let success = output.status.success();
        let stderr = normalize::diagnostics(output.stderr);

        let check = match self.expected {
            Expected::Pass => Test::check_pass,
            Expected::CompileFail => Test::check_compile_fail,
        };

        check(self, success, stderr)
    }

    fn check_pass(&self, success: bool, stderr: String) -> Result<()> {
        if !success {
            message::failed_to_build(stderr);
            return Err(Error::CargoFail);
        }

        let name = self.name();
        let output = cargo::run_test(&name)?;
        message::output(stderr, &output);

        if output.status.success() {
            Ok(())
        } else {
            Err(Error::RunFailed)
        }
    }

    fn check_compile_fail(&self, success: bool, stderr: String) -> Result<()> {
        if success {
            message::should_not_have_compiled();
            message::warnings(stderr);
            return Err(Error::ShouldNotHaveCompiled);
        }

        let stderr_path = self.path.with_extension("stderr");
        if !stderr_path.exists() {
            let wip_dir = Path::new("wip");
            fs::create_dir_all(wip_dir)?;
            let stderr_name = stderr_path
                .file_name()
                .unwrap_or_else(|| OsStr::new("test.stderr"));
            let wip_path = wip_dir.join(stderr_name);
            message::write_stderr(&wip_path, &stderr_path, &stderr);
            fs::write(wip_path, stderr).map_err(Error::WriteStderr)?;
            return Ok(());
        }

        let expected = fs::read_to_string(stderr_path).map_err(Error::ReadStderr)?;
        if expected == stderr {
            message::nice();
            Ok(())
        } else {
            message::mismatch(&expected, &stderr);
            Err(Error::Mismatch)
        }
    }
}

fn check_exists(path: &Path) -> Result<()> {
    match File::open(path) {
        Ok(_) => Ok(()),
        Err(err) => Err(Error::Open(path.to_owned(), err)),
    }
}
