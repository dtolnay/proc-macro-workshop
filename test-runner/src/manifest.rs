use serde::Serialize;

use std::collections::BTreeMap as Map;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct Manifest {
    pub package: Package,
    pub dependencies: Map<String, Dependency>,
    #[serde(rename = "bin")]
    pub bins: Vec<Bin>,
}

#[derive(Serialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub edition: Edition,
    pub publish: bool,
}

#[derive(Serialize)]
pub enum Edition {
    #[serde(rename = "2018")]
    E2018,
}

#[derive(Serialize)]
pub struct Bin {
    pub name: String,
    pub path: PathBuf,
}

#[derive(Serialize)]
pub struct Dependency {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<String>,
}

#[derive(Serialize)]
pub struct Config {
    pub build: Build,
}

#[derive(Serialize)]
pub struct Build {
    pub rustflags: Vec<String>,
}
