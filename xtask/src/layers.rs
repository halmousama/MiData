//! The registered crate DAG (REBUILD_PLAN.md §4.2, D24/D26).
//!
//! Every crate and every allowed edge is declared here. A crate that exists but is not registered
//! fails; an edge (internal or external) that is not declared fails. Crates that are registered but
//! not born yet are simply counted as planned — product crates are birth-gated from P0-2 onward,
//! so P0-1 ships only `xtask` in the workspace.
//!
//! Enforcement details:
//! - `midata-testkit` is a **dev-dependency only**.
//! - Application crates are roots, never libraries: nothing below the app layer may depend on one.
//!   The single app-to-app edge is declared per root in `APP_EDGES` — today only
//!   `midata → midata-app` (the UI composition root needs the egui shell). `midata-cli` declares
//!   none, so the headless binary can never link the egui shell.
//! - Heavy/unstable crates are restricted to their home crate even when an app wildcard is used:
//!   egui stack → `midata-app`/`midata`; tokio/reqwest → `midata-ai`; sqlparser → `midata-queryguard`;
//!   file-format crates → `midata-exchange`; banned crates → nowhere.
//!
//! `birth` is declared and printed, never enforced here; birth order (D26) is owner review.

use crate::workspace::{DependencyKind, Workspace};
use anyhow::{bail, Result};
use std::collections::BTreeSet;
use std::fmt;
use std::path::Path;

mod rules;

use rules::{allowed_app_edges, CrateRule, APP_CRATES, BANNED_ANYWHERE, ONLY_IN, REGISTRY};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Violation {
    pub package: String,
    pub message: String,
}

impl fmt::Display for Violation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.package, self.message)
    }
}

fn rule_for(name: &str) -> Option<&'static CrateRule> {
    REGISTRY.iter().find(|rule| rule.name == name)
}

fn violation(package: &str, message: impl Into<String>) -> Violation {
    Violation {
        package: package.to_string(),
        message: message.into(),
    }
}

/// Pure checker: given a workspace model, return every layering violation.
pub fn check_workspace(workspace: &Workspace) -> Vec<Violation> {
    let registered: BTreeSet<&str> = REGISTRY.iter().map(|rule| rule.name).collect();
    let mut violations = Vec::new();

    for package in &workspace.packages {
        let Some(rule) = rule_for(&package.name) else {
            violations.push(violation(
                &package.name,
                "workspace crate is not registered in xtask/src/layers.rs (birth-gated: register it in the phase that creates it)",
            ));
            continue;
        };

        if !package.manifest_path.ends_with(Path::new(rule.manifest)) {
            violations.push(violation(
                &package.name,
                format!(
                    "manifest lives at {} but the registry expects {} (a crate must stay in its registered home)",
                    package.manifest_path.display(),
                    rule.manifest
                ),
            ));
        }

        for dependency in &package.dependencies {
            let dep = dependency.name.as_str();

            if registered.contains(dep) {
                if APP_CRATES.contains(&dep) {
                    // An app crate is a root, not a library. An app root may depend on one only
                    // when its own rule declares that exact edge (`APP_EDGES`); every other crate
                    // — including `midata-cli`, which declares none — is rejected.
                    if !allowed_app_edges(&package.name).contains(&dep) {
                        violations.push(violation(
                            &package.name,
                            format!(
                                "depends on application crate `{dep}` — nothing may depend on an app unless its rule declares the edge"
                            ),
                        ));
                        continue;
                    }
                    if !dependency.is_path {
                        violations.push(violation(
                            &package.name,
                            format!("internal dependency `{dep}` must be a path dependency"),
                        ));
                    }
                    continue;
                }

                if let Some(dep_rule) = rule_for(dep) {
                    if dep_rule.dev_only {
                        if dependency.kind != DependencyKind::Dev {
                            violations.push(violation(
                                &package.name,
                                format!(
                                    "depends on `{dep}` outside dev-dependencies (it is a dev-only fixture crate)"
                                ),
                            ));
                        } else if !dependency.is_path {
                            violations.push(violation(
                                &package.name,
                                format!("dev dependency `{dep}` must be a path dependency"),
                            ));
                        }
                        continue;
                    }
                }

                if !dependency.is_path {
                    violations.push(violation(
                        &package.name,
                        format!("internal dependency `{dep}` must be a path dependency"),
                    ));
                }

                if rule.any_internal {
                    continue;
                }

                if !rule.internal.contains(&dep) {
                    violations.push(violation(
                        &package.name,
                        format!(
                            "illegal internal edge `{} -> {dep}` (allowed: {})",
                            rule.name,
                            display_list(rule.internal)
                        ),
                    ));
                }
                continue;
            }

            if BANNED_ANYWHERE.contains(&dep) {
                violations.push(violation(
                    &package.name,
                    format!(
                        "banned dependency `{dep}` (architecture decision: never a dependency)"
                    ),
                ));
                continue;
            }

            if let Some((_, homes)) = ONLY_IN.iter().find(|(name, _)| *name == dep) {
                if !homes.contains(&rule.name) {
                    violations.push(violation(
                        &package.name,
                        format!(
                            "`{dep}` may only be used by {} (found in {})",
                            display_list(homes),
                            rule.name
                        ),
                    ));
                }
            }

            if is_wildcard(rule.external) {
                continue;
            }
            if rule.external.contains(&dep) {
                continue;
            }
            if dependency.kind == DependencyKind::Dev && rule.dev_external.contains(&dep) {
                continue;
            }

            violations.push(violation(
                &package.name,
                format!(
                    "unregistered external dependency `{dep}` — add it to the crate's allowlist in xtask/src/layers.rs (dependency changes are owner-approved)"
                ),
            ));
        }
    }

    violations
}

fn is_wildcard(external: &[&str]) -> bool {
    external.len() == 1 && external.first() == Some(&"*")
}

fn display_list(items: &[&str]) -> String {
    if items.is_empty() {
        "none".to_string()
    } else {
        items.join(", ")
    }
}

pub fn run(root: &Path) -> Result<()> {
    let workspace = crate::workspace::load(root)?;
    let violations = check_workspace(&workspace);

    if violations.is_empty() {
        let present: BTreeSet<&str> = workspace
            .packages
            .iter()
            .map(|package| package.name.as_str())
            .collect();
        let planned = REGISTRY
            .iter()
            .filter(|rule| !present.contains(rule.name))
            .count();
        println!(
            "check-layers: OK — {} workspace crate(s) registered, {} planned crate(s) not yet born",
            present.len(),
            planned
        );
        println!("  registered DAG (crate(birth phase) by layer):");
        let mut by_layer: std::collections::BTreeMap<&str, Vec<String>> =
            std::collections::BTreeMap::new();
        for rule in REGISTRY {
            by_layer
                .entry(rule.layer)
                .or_default()
                .push(format!("{}({})", rule.name, rule.birth));
        }
        for (layer, crates) in by_layer {
            println!("    {layer}: {}", crates.join(", "));
        }
        return Ok(());
    }

    eprintln!(
        "check-layers: {} violation(s) against the registered DAG:",
        violations.len()
    );
    for item in &violations {
        eprintln!("  - {item}");
    }
    bail!("layer check failed")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::{Dependency, Package};
    use std::path::PathBuf;

    fn dependency(name: &str, kind: DependencyKind) -> Dependency {
        Dependency {
            name: name.to_string(),
            kind,
            is_path: true,
        }
    }

    fn package(name: &str, deps: &[Dependency]) -> Package {
        let manifest = REGISTRY
            .iter()
            .find(|rule| rule.name == name)
            .map(|rule| PathBuf::from(rule.manifest))
            .unwrap_or_else(|| PathBuf::from(format!("crates/{name}/Cargo.toml")));
        Package {
            name: name.to_string(),
            manifest_path: manifest,
            dependencies: deps.to_vec(),
        }
    }

    fn workspace(packages: Vec<Package>) -> Workspace {
        Workspace { packages }
    }

    #[test]
    fn registered_plan_has_unique_names_and_valid_edges() {
        let names: BTreeSet<&str> = REGISTRY.iter().map(|rule| rule.name).collect();
        assert_eq!(names.len(), REGISTRY.len(), "duplicate crate in REGISTRY");

        for rule in REGISTRY {
            assert!(!rule.birth.is_empty(), "{} has no birth phase", rule.name);
            for edge in rule.internal {
                assert!(
                    names.contains(edge),
                    "{} declares unknown internal edge {edge}",
                    rule.name
                );
            }
            if rule.any_internal {
                assert!(
                    is_wildcard(rule.external),
                    "{}: only application roots may use the external wildcard",
                    rule.name
                );
            }
        }

        // Every declared app-to-app edge must name a registered app root, and may only be declared
        // by an app root itself.
        for (package, edges) in rules::APP_EDGES {
            assert!(
                APP_CRATES.contains(package),
                "`{package}` declares an app edge but is not an app root"
            );
            for edge in *edges {
                assert!(
                    APP_CRATES.contains(edge),
                    "`{package}` declares `{edge}` as an app edge but it is not an app crate"
                );
            }
        }
    }

    #[test]
    fn allowed_edge_is_accepted() {
        let model = workspace(vec![
            package(
                "midata-domain",
                &[dependency("serde", DependencyKind::Normal)],
            ),
            package("xtask", &[dependency("serde_json", DependencyKind::Normal)]),
        ]);
        assert_eq!(check_workspace(&model), Vec::new());
    }

    #[test]
    fn illegal_ui_dependency_below_app_is_rejected() {
        let model = workspace(vec![package(
            "midata-domain",
            &[dependency("egui", DependencyKind::Normal)],
        )]);
        let violations = check_workspace(&model);
        assert!(
            violations
                .iter()
                .any(|item| item.message.contains("may only be used by")),
            "a UI dependency in L0 must be a violation: {violations:?}"
        );
    }

    #[test]
    fn app_depending_on_store_is_rejected() {
        let model = workspace(vec![
            package(
                "midata-app",
                &[dependency("midata-store", DependencyKind::Normal)],
            ),
            package("midata-store", &[]),
        ]);
        let violations = check_workspace(&model);
        assert!(
            violations
                .iter()
                .any(|item| item.message.contains("illegal internal edge")),
            "midata-app may depend only on domain, engine and i18n: {violations:?}"
        );
    }

    #[test]
    fn testkit_as_a_normal_dependency_is_rejected() {
        let model = workspace(vec![
            package(
                "midata-engine",
                &[dependency("midata-testkit", DependencyKind::Normal)],
            ),
            package("midata-testkit", &[]),
        ]);
        let violations = check_workspace(&model);
        assert!(
            violations
                .iter()
                .any(|item| item.message.contains("dev-dependencies")),
            "testkit is dev-only: {violations:?}"
        );
    }

    #[test]
    fn testkit_as_a_dev_dependency_is_accepted() {
        let model = workspace(vec![
            package(
                "midata-engine",
                &[dependency("midata-testkit", DependencyKind::Dev)],
            ),
            package("midata-testkit", &[]),
        ]);
        assert_eq!(check_workspace(&model), Vec::new());
    }

    #[test]
    fn unregistered_workspace_crate_is_rejected() {
        let model = workspace(vec![package("midata-mystery", &[])]);
        let violations = check_workspace(&model);
        assert!(
            violations
                .iter()
                .any(|item| item.message.contains("not registered")),
            "an unregistered crate must fail: {violations:?}"
        );
    }

    #[test]
    fn unregistered_external_dependency_is_rejected() {
        let model = workspace(vec![package(
            "xtask",
            &[dependency("clap", DependencyKind::Normal)],
        )]);
        let violations = check_workspace(&model);
        assert!(
            violations
                .iter()
                .any(|item| item.message.contains("unregistered external dependency")),
            "an undeclared dependency must fail: {violations:?}"
        );
    }

    #[test]
    fn application_crate_cannot_be_a_dependency() {
        let model = workspace(vec![
            package(
                "midata-engine",
                &[dependency("midata-cli", DependencyKind::Normal)],
            ),
            package("midata-cli", &[]),
        ]);
        let violations = check_workspace(&model);
        assert!(
            violations
                .iter()
                .any(|item| item.message.contains("nothing may depend on an app")),
            "apps are never dependencies: {violations:?}"
        );
    }

    /// The UI composition root is the one legal app-to-app edge: without it `apps/midata` cannot
    /// open a window (it must link `midata-app`). This test is the positive half of R1.
    #[test]
    fn ui_composition_root_may_depend_on_the_app_shell() {
        let model = workspace(vec![
            package(
                "midata",
                &[dependency("midata-app", DependencyKind::Normal)],
            ),
            package("midata-app", &[]),
        ]);
        assert_eq!(
            check_workspace(&model),
            Vec::new(),
            "`midata` must be allowed to depend on `midata-app`"
        );
    }

    /// The headless binary must never link the egui shell. This is the negative half of R1: the
    /// gate rejected this edge for the right reason only after `midata-cli` got an explicit
    /// internal allowlist instead of the app wildcard.
    #[test]
    fn headless_cli_may_not_depend_on_the_ui_shell() {
        let model = workspace(vec![
            package(
                "midata-cli",
                &[dependency("midata-app", DependencyKind::Normal)],
            ),
            package("midata-app", &[]),
        ]);
        let violations = check_workspace(&model);
        assert!(
            violations
                .iter()
                .any(|item| item.message.contains("nothing may depend on an app")),
            "midata-cli → midata-app must be rejected: {violations:?}"
        );
    }

    /// Proof that `midata-cli` no longer carries the app wildcard: an edge outside its explicit
    /// allowlist (the analytics engine) fails the same way any L5-only edge does.
    #[test]
    fn headless_cli_has_an_explicit_internal_allowlist() {
        let model = workspace(vec![
            package(
                "midata-cli",
                &[dependency("midata-analytics", DependencyKind::Normal)],
            ),
            package("midata-analytics", &[]),
        ]);
        let violations = check_workspace(&model);
        assert!(
            violations
                .iter()
                .any(|item| item.message.contains("illegal internal edge")),
            "midata-cli may depend only on domain, i18n, store and engine: {violations:?}"
        );
    }
}
