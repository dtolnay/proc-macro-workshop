use std::env;
use std::fmt::{self, Display};
use std::io;
use std::path::PathBuf;

#[remain::sorted]
pub enum Error {
    Cargo(io::Error),
    CargoFail,
    Io(io::Error),
    Mismatch,
    Open(PathBuf, io::Error),
    PkgName(env::VarError),
    ProjectDir,
    ReadStderr(io::Error),
    RunFailed,
    ShouldNotHaveCompiled,
    Toml(toml::ser::Error),
    WriteStderr(io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Display for Error {
    #[remain::check]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;

        #[sorted]
        match self {
            Cargo(e) => write!(f, "failed to execute cargo: {}", e),
            CargoFail => write!(f, "cargo reported an error"),
            Io(e) => write!(f, "{}", e),
            Mismatch => write!(f, "compiler error does not match expected error"),
            Open(path, e) => write!(f, "{}: {}", path.display(), e),
            PkgName(e) => write!(f, "failed to detect CARGO_PKG_NAME: {}", e),
            ProjectDir => write!(f, "failed to determine name of project dir"),
            ReadStderr(e) => write!(f, "failed to read stderr file: {}", e),
            RunFailed => write!(f, "execution of the test case was unsuccessful"),
            ShouldNotHaveCompiled => {
                write!(f, "expected test case to fail to compile, but it succeeded")
            }
            Toml(e) => write!(f, "{}", e),
            WriteStderr(e) => write!(f, "failed to write stderr file: {}", e),
        }
    }
}

impl Error {
    pub fn already_printed(&self) -> bool {
        use self::Error::*;

        match self {
            CargoFail | Mismatch | RunFailed | ShouldNotHaveCompiled => true,
            _ => false,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Error::Toml(err)
    }
}
