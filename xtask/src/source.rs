//! Source-file discovery shared by every file-based gate.
//!
//! The walker is deliberately dumb and deterministic: sorted relative paths, forward slashes on
//! every OS, and a short skip list so `target/`, `.git/` and local `plan/` never affect a gate.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct SourceFile {
    /// Path relative to the repository root, always with `/` separators.
    pub relative: String,
    pub absolute: PathBuf,
}

/// Every file with `extension` under `root`, excluding build and local-only directories.
pub fn files_with_extension(root: &Path, extension: &str) -> Result<Vec<SourceFile>> {
    let mut files = Vec::new();
    walk(root, root, extension, &mut files)?;
    files.sort_by(|a, b| a.relative.cmp(&b.relative));
    Ok(files)
}

/// Every Rust source file in the repository.
pub fn rust_files(root: &Path) -> Result<Vec<SourceFile>> {
    files_with_extension(root, "rs")
}

fn walk(root: &Path, directory: &Path, extension: &str, out: &mut Vec<SourceFile>) -> Result<()> {
    let entries =
        std::fs::read_dir(directory).with_context(|| format!("reading {}", directory.display()))?;

    for entry in entries {
        let entry =
            entry.with_context(|| format!("reading an entry in {}", directory.display()))?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .with_context(|| format!("reading the file type of {}", path.display()))?;

        if file_type.is_dir() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if should_skip_dir(&name) {
                continue;
            }
            walk(root, &path, extension, out)?;
        } else if file_type.is_file()
            && path.extension().and_then(|value| value.to_str()) == Some(extension)
        {
            out.push(SourceFile {
                relative: relative_path(root, &path),
                absolute: path,
            });
        }
    }

    Ok(())
}

fn should_skip_dir(name: &str) -> bool {
    matches!(name, "target" | "plan" | "node_modules" | ".git")
}

/// Normalize a path for reports: relative to the repository root, `/` separators.
pub fn relative_path(root: &Path, path: &Path) -> String {
    let relative = path.strip_prefix(root).unwrap_or(path);
    relative.to_string_lossy().replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_and_local_dirs_are_skipped() {
        assert!(should_skip_dir("target"));
        assert!(should_skip_dir("plan"));
        assert!(should_skip_dir(".git"));
        assert!(!should_skip_dir("crates"));
        // Hidden but shipped: the CI workflow and cargo config must stay in the scan.
        assert!(!should_skip_dir(".github"));
        assert!(!should_skip_dir(".cargo"));
    }

    #[test]
    fn relative_paths_use_forward_slashes() {
        let root = Path::new("repo");
        let path = root.join("xtask").join("src").join("main.rs");
        assert_eq!(relative_path(root, &path), "xtask/src/main.rs");
    }
}
