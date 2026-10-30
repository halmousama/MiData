#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable,
    clippy::indexing_slicing
)]
//! Workspace gate runner for MiData.
//!
//! Every gate lives here (not in CI YAML) so that `cargo xtask ci` and GitHub Actions execute the
//! same code. A gate that cannot run says so loudly; it never passes silently (P0-1 acceptance,
//! `test-integrity.no-vacuous-gates`).
//!
//! Commands: `check-layers`, `file-budget`, `i18n-coverage`, `stats`, `ci`, plus the registered
//! skeletons `new-command`, `new-screen`, `seed`, `bench` that fail with an "unsupported" error.

mod attribution;
mod ci;
mod file_budget;
mod gates;
mod i18n;
mod layers;
mod process;
mod scaffold;
mod source;
mod stats;
mod terms;
mod workspace;

use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args_os()
        .skip(1)
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect();

    match dispatch(&args) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("xtask: error: {err:#}");
            ExitCode::FAILURE
        }
    }
}

fn dispatch(args: &[String]) -> anyhow::Result<ExitCode> {
    let Some((command, rest)) = args.split_first() else {
        print_usage();
        return Ok(ExitCode::from(2));
    };

    let root = workspace::root_dir()?;

    match command.as_str() {
        "check-layers" => {
            layers::run(&root)?;
            Ok(ExitCode::SUCCESS)
        }
        "banned-names" => {
            if has_flag(rest, "--write-baseline") {
                attribution::write_baseline(&root)?;
            } else {
                gates::banned_names(&root)?;
            }
            Ok(ExitCode::SUCCESS)
        }
        "commit-msg" => {
            let message = rest
                .first()
                .ok_or_else(|| anyhow::anyhow!("commit-msg needs the path of the message file"))?;
            attribution::check_message_file(std::path::Path::new(message))?;
            Ok(ExitCode::SUCCESS)
        }
        "file-budget" => {
            if has_flag(rest, "--write-baseline") {
                file_budget::write_baseline(&root)?;
            } else {
                file_budget::check(&root)?;
            }
            Ok(ExitCode::SUCCESS)
        }
        "i18n-coverage" => {
            if has_flag(rest, "--write-baseline") {
                i18n::write_baseline(&root)?;
            } else {
                i18n::check(&root)?;
            }
            Ok(ExitCode::SUCCESS)
        }
        "stats" => {
            stats::run(&root, has_flag(rest, "--check"))?;
            Ok(ExitCode::SUCCESS)
        }
        "ci" => {
            ci::run(&root)?;
            Ok(ExitCode::SUCCESS)
        }
        "new-command" | "new-screen" | "seed" | "bench" => {
            scaffold::unsupported(command)?;
            Ok(ExitCode::SUCCESS)
        }
        "help" | "--help" | "-h" => {
            print_usage();
            Ok(ExitCode::SUCCESS)
        }
        other => {
            eprintln!("xtask: unknown command `{other}`\n");
            print_usage();
            Ok(ExitCode::from(2))
        }
    }
}

fn has_flag(args: &[String], flag: &str) -> bool {
    args.iter().any(|arg| arg == flag)
}

fn print_usage() {
    println!(
        "MiData workspace gates\n\
         \n\
         Usage: cargo xtask <command> [flags]\n\
         \n\
         Commands:\n\
           check-layers                 verify the registered crate DAG (xtask/src/layers.rs)\n\
           banned-names                 fail on a banned name or D30 attribution term outside its allowlist\n\
           commit-msg <file>            reject a commit message carrying a D30 attribution term (git hook)\n\
           file-budget                  fail on Rust files over the 500-line soft cap\n\
           i18n-coverage                report EN/FR catalogue coverage and enforce the ratchet\n\
           stats                        regenerate docs/scorecard.md (--check: verify only)\n\
           ci                           run the full pre-completion gate\n\
           new-command                  registered skeleton (not implemented yet)\n\
           new-screen                   registered skeleton (not implemented yet)\n\
           seed                         registered skeleton (not implemented yet)\n\
           bench                        registered skeleton (not implemented yet)\n\
         \n\
         Baseline flags:\n\
           file-budget --write-baseline     rewrite docs/file-budget.baseline\n\
           i18n-coverage --write-baseline   rewrite docs/i18n-coverage.baseline\n\
           banned-names --write-baseline    rewrite docs/banned-names.baseline\n"
    );
}

#[cfg(test)]
mod tests {
    use super::has_flag;

    #[test]
    fn flags_are_matched_exactly() {
        let args = vec!["--check".to_string(), "--quick".to_string()];
        assert!(has_flag(&args, "--check"));
        assert!(!has_flag(&args, "--check-only"));
    }
}
