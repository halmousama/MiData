//! Registered scaffolding commands that are deliberately not implemented in P0-1.
//!
//! They exist so the command surface in `docs/development.md` is honest: invoking one fails with an
//! actionable "unsupported" error. It never pretends to generate a file and never panics
//! (AGENTS.md: unfinished features return an "unsupported" error).

use anyhow::{bail, Result};

pub fn unsupported(command: &str) -> Result<()> {
    let landing = match command {
        "new-command" | "new-screen" => "P0-4 (engine registry) / P0-5 (app shell)",
        "seed" => "P0-6 (deterministic demo seed)",
        "bench" => "P0-9 (perf harness and budgets)",
        _ => "a later task",
    };
    bail!(
        "`cargo xtask {command}` is a registered skeleton and is not implemented yet — it lands with {landing}. \
         It fails loudly rather than generating a misleading placeholder."
    )
}

#[cfg(test)]
mod tests {
    use super::unsupported;

    #[test]
    fn every_skeleton_fails_with_an_unsupported_message() {
        for command in ["new-command", "new-screen", "seed", "bench"] {
            let error = unsupported(command).expect_err("skeletons must return Err");
            assert!(
                error.to_string().contains("not implemented yet"),
                "{command} must explain that it is unsupported: {error}"
            );
        }
    }
}
