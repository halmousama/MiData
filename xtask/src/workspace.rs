//! Reading the Cargo workspace model once, for every gate that needs it.
//!
//! `cargo metadata --no-deps` is used deliberately: gates inspect manifests, they never build or
//! resolve dependencies. `--no-deps` also keeps the check cheap and network-free.

use anyhow::{anyhow, Context, Result};
use serde_json::Value;
use std::path::{Path, PathBuf};

/// The repository root — the parent of the `xtask` crate.
pub fn root_dir() -> Result<PathBuf> {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    Path::new(manifest_dir)
        .parent()
        .map(Path::to_path_buf)
        .ok_or_else(|| anyhow!("xtask manifest has no parent directory"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyKind {
    Normal,
    Dev,
    Build,
}

#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub kind: DependencyKind,
    pub is_path: bool,
}

#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub manifest_path: PathBuf,
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Clone)]
pub struct Workspace {
    pub packages: Vec<Package>,
}

/// Load the workspace members (not the dependency graph) via `cargo metadata`.
pub fn load(root: &Path) -> Result<Workspace> {
    let stdout = crate::process::capture(
        "cargo",
        &["metadata", "--no-deps", "--format-version", "1"],
        root,
    )
    .context("running `cargo metadata` (is cargo on PATH?)")?;

    let value: Value = serde_json::from_str(&stdout).context("parsing `cargo metadata` JSON")?;

    let raw_packages = value
        .get("packages")
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("`cargo metadata` returned no `packages` array"))?;

    let mut packages = Vec::with_capacity(raw_packages.len());
    for raw in raw_packages {
        let name = raw
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("package without a name in `cargo metadata`"))?;
        let manifest_path = raw
            .get("manifest_path")
            .and_then(Value::as_str)
            .map(PathBuf::from)
            .unwrap_or_default();

        let mut dependencies = Vec::new();
        if let Some(raw_deps) = raw.get("dependencies").and_then(Value::as_array) {
            for dep in raw_deps {
                let dep_name = dep.get("name").and_then(Value::as_str).unwrap_or_default();
                if dep_name.is_empty() {
                    continue;
                }
                let kind = match dep.get("kind").and_then(Value::as_str) {
                    Some("dev") => DependencyKind::Dev,
                    Some("build") => DependencyKind::Build,
                    _ => DependencyKind::Normal,
                };
                let is_path = dep.get("path").and_then(Value::as_str).is_some();
                dependencies.push(Dependency {
                    name: dep_name.to_string(),
                    kind,
                    is_path,
                });
            }
        }

        packages.push(Package {
            name: name.to_string(),
            manifest_path,
            dependencies,
        });
    }

    packages.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(Workspace { packages })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_dir_contains_the_xtask_manifest() {
        let root = root_dir().expect("xtask has a parent");
        assert!(root.join("xtask").join("Cargo.toml").is_file());
        assert!(root.join("Cargo.toml").is_file());
    }
}
