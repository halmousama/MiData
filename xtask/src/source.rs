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

/// Extensionless files are scanned only up to this size. A file without an extension is usually text
/// (a script, a licence, a hook) but could be a build artefact; the cap keeps an arbitrarily large
/// blob out of the gate's memory without losing any realistic source file.
const MAX_EXTENSIONLESS_BYTES: u64 = 1_048_576;

/// Every file with `extension` under `root`, excluding build and local-only directories.
pub fn files_with_extension(root: &Path, extension: &str) -> Result<Vec<SourceFile>> {
    let mut files = Vec::new();
    walk(
        root,
        root,
        &mut |path: &Path| path.extension().and_then(|value| value.to_str()) == Some(extension),
        &mut files,
    )?;
    files.sort_by(|a, b| a.relative.cmp(&b.relative));
    Ok(files)
}

/// Every Rust source file in the repository.
pub fn rust_files(root: &Path) -> Result<Vec<SourceFile>> {
    files_with_extension(root, "rs")
}

/// Every **text** file that has no extension — `tools/hooks/commit-msg`, `LICENSE-*`, a future
/// `Makefile`. An extension-driven scan silently ignores these, which would let a shipped surface
/// escape the naming gates entirely (P0-14 review R3).
///
/// "Text" is decided by decoding: a file that is not valid UTF-8 is a binary artefact, not a
/// branding surface, and is skipped. The size cap is `MAX_EXTENSIONLESS_BYTES`.
pub fn extensionless_text_files(root: &Path) -> Result<Vec<SourceFile>> {
    let mut candidates = Vec::new();
    walk(
        root,
        root,
        &mut |path: &Path| path.extension().is_none(),
        &mut candidates,
    )?;
    candidates.sort_by(|a, b| a.relative.cmp(&b.relative));

    let mut files = Vec::new();
    for candidate in candidates {
        let metadata = std::fs::metadata(&candidate.absolute)
            .with_context(|| format!("reading the size of {}", candidate.relative))?;
        if metadata.len() > MAX_EXTENSIONLESS_BYTES {
            continue;
        }
        let bytes = std::fs::read(&candidate.absolute)
            .with_context(|| format!("reading {}", candidate.relative))?;
        if std::str::from_utf8(&bytes).is_ok() {
            files.push(candidate);
        }
    }
    Ok(files)
}

fn walk(
    root: &Path,
    directory: &Path,
    matches: &mut dyn FnMut(&Path) -> bool,
    out: &mut Vec<SourceFile>,
) -> Result<()> {
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
            walk(root, &path, matches, out)?;
        } else if file_type.is_file() && matches(&path) {
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

    /// P0-14 review R3: an extension-driven scan used to ignore every extensionless file, so a hook
    /// script or a licence could carry a banned name and the gate would still exit 0.
    #[test]
    fn extensionless_text_files_are_scanned_and_binaries_are_not() -> Result<()> {
        let root = std::env::temp_dir().join(format!(
            "midata-source-extensionless-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(root.join("nested"))?;
        std::fs::write(root.join("hook-script"), "text without an extension\n")?;
        std::fs::write(root.join("nested").join("LICENSE"), "licence text\n")?;
        std::fs::write(root.join("blob"), [0xff_u8, 0xfe, 0x00, 0x01])?;
        std::fs::write(root.join("plain.md"), "has an extension\n")?;

        let found = extensionless_text_files(&root)?;
        let relative: Vec<&str> = found.iter().map(|file| file.relative.as_str()).collect();
        assert_eq!(relative, vec!["hook-script", "nested/LICENSE"]);

        std::fs::remove_dir_all(&root)?;
        Ok(())
    }
}
