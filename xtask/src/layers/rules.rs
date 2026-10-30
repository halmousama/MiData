//! The registered crate DAG data (REBUILD_PLAN.md §4.2, D24/D26).
//!
//! Kept separate from the checker so the registry table is easy to review and neither file
//! outgrows the 500-line budget. Adding a crate or an edge is a deliberate edit here.

pub(crate) struct CrateRule {
    pub name: &'static str,
    pub layer: &'static str,
    /// Phase that births the crate (D26: no crate before the phase that first uses it).
    ///
    /// This is **metadata**: `check-layers` prints it and never compares it against a phase
    /// constant (the gate does not know which phase is running). Birth order is enforced by owner
    /// review against D26, not by this gate.
    pub birth: &'static str,
    /// Repository-relative manifest location, used to catch a crate that drifted to another folder.
    pub manifest: &'static str,
    /// Allowed internal dependencies, by package name.
    pub internal: &'static [&'static str],
    /// May depend on any registered internal crate (application roots only).
    pub any_internal: bool,
    /// Allowed external dependencies. `["*"]` means "any" (application roots only).
    pub external: &'static [&'static str],
    /// External dependencies allowed only as dev-dependencies.
    pub dev_external: &'static [&'static str],
    /// May only ever be consumed as a dev-dependency.
    pub dev_only: bool,
}

pub(crate) const REGISTRY: &[CrateRule] = &[
    CrateRule {
        name: "midata-domain",
        layer: "L0",
        birth: "P0-2",
        manifest: "crates/midata-domain/Cargo.toml",
        internal: &[],
        any_internal: false,
        external: &["serde", "chrono", "thiserror"],
        // Dev-only edge to an already-approved Phase-0 crate: the value contract is a serde contract,
        // so its representation is asserted in tests rather than assumed (P0-2).
        dev_external: &["serde_json"],
        dev_only: false,
    },
    CrateRule {
        name: "midata-i18n",
        layer: "L0",
        birth: "P0-2",
        manifest: "crates/midata-i18n/Cargo.toml",
        internal: &[],
        any_internal: false,
        external: &["serde", "thiserror"],
        dev_external: &[],
        dev_only: false,
    },
    CrateRule {
        name: "midata-store",
        layer: "L1",
        birth: "P0-3",
        manifest: "crates/midata-store/Cargo.toml",
        internal: &["midata-domain", "midata-i18n"],
        any_internal: false,
        external: &[
            "rusqlite",
            "serde",
            "serde_json",
            "thiserror",
            "directories",
        ],
        dev_external: &[],
        dev_only: false,
    },
    CrateRule {
        name: "midata-analytics",
        layer: "L2",
        birth: "P3",
        manifest: "crates/midata-analytics/Cargo.toml",
        internal: &["midata-domain", "midata-i18n", "midata-store"],
        any_internal: false,
        external: &["serde", "serde_json", "chrono", "thiserror", "rayon"],
        dev_external: &[],
        dev_only: false,
    },
    CrateRule {
        name: "midata-queryguard",
        layer: "L2",
        birth: "P5",
        manifest: "crates/midata-queryguard/Cargo.toml",
        internal: &["midata-domain", "midata-i18n"],
        any_internal: false,
        external: &["sqlparser", "serde", "serde_json", "thiserror"],
        dev_external: &[],
        dev_only: false,
    },
    CrateRule {
        name: "midata-exchange",
        layer: "L2",
        birth: "P3",
        manifest: "crates/midata-exchange/Cargo.toml",
        internal: &["midata-domain", "midata-i18n"],
        any_internal: false,
        external: &[
            "calamine",
            "csv",
            "rust_xlsxwriter",
            "printpdf",
            "genpdf",
            "image",
            "serde",
            "serde_json",
            "thiserror",
        ],
        dev_external: &[],
        dev_only: false,
    },
    CrateRule {
        name: "midata-ops",
        layer: "L2",
        birth: "P0-3",
        manifest: "crates/midata-ops/Cargo.toml",
        internal: &[
            "midata-domain",
            "midata-i18n",
            "midata-store",
            "midata-analytics",
            "midata-exchange",
        ],
        any_internal: false,
        external: &["evalexpr", "serde", "serde_json", "chrono", "thiserror"],
        dev_external: &[],
        dev_only: false,
    },
    CrateRule {
        name: "midata-engine",
        layer: "L3",
        birth: "P0-4",
        manifest: "crates/midata-engine/Cargo.toml",
        internal: &[
            "midata-domain",
            "midata-i18n",
            "midata-store",
            "midata-ops",
            "midata-analytics",
        ],
        any_internal: false,
        external: &[
            "serde",
            "serde_json",
            "chrono",
            "thiserror",
            "crossbeam-channel",
        ],
        dev_external: &[],
        dev_only: false,
    },
    CrateRule {
        name: "midata-ai",
        layer: "L4",
        birth: "P5",
        manifest: "crates/midata-ai/Cargo.toml",
        internal: &[
            "midata-domain",
            "midata-i18n",
            "midata-store",
            "midata-ops",
            "midata-engine",
            "midata-queryguard",
        ],
        any_internal: false,
        external: &[
            "reqwest",
            "reqwest-eventsource",
            "tokio",
            "serde",
            "serde_json",
            "thiserror",
            "pulldown-cmark",
        ],
        dev_external: &[],
        dev_only: false,
    },
    CrateRule {
        name: "midata-app",
        layer: "L5",
        birth: "P0-5",
        manifest: "crates/midata-app/Cargo.toml",
        internal: &["midata-domain", "midata-engine", "midata-i18n"],
        any_internal: false,
        external: &[
            "egui",
            "eframe",
            "egui_plot",
            "egui_extras",
            "image",
            "rfd",
            "serde",
            "serde_json",
            "chrono",
            "thiserror",
            "crossbeam-channel",
            "tracing",
            "tracing-subscriber",
            "tracing-appender",
            "directories",
            "keyring",
        ],
        dev_external: &["egui_kittest"],
        dev_only: false,
    },
    CrateRule {
        name: "midata-testkit",
        layer: "dev",
        birth: "P0-4",
        manifest: "crates/midata-testkit/Cargo.toml",
        internal: &[
            "midata-domain",
            "midata-i18n",
            "midata-store",
            "midata-ops",
            "midata-engine",
            "midata-analytics",
        ],
        any_internal: false,
        external: &[
            "rusqlite",
            "serde",
            "serde_json",
            "tempfile",
            "egui_kittest",
        ],
        dev_external: &[],
        dev_only: true,
    },
    CrateRule {
        name: "midata",
        layer: "app",
        birth: "P0-5",
        manifest: "apps/midata/Cargo.toml",
        internal: &[],
        any_internal: true,
        external: &["*"],
        dev_external: &["egui_kittest", "midata-testkit"],
        dev_only: false,
    },
    CrateRule {
        name: "midata-cli",
        layer: "app",
        birth: "P0-7",
        manifest: "apps/midata-cli/Cargo.toml",
        // The headless binary links the data/engine spine only. It must never link the egui shell
        // (`midata-app`) or any other surface crate: `midata` is an app root, not a library.
        internal: &[
            "midata-domain",
            "midata-i18n",
            "midata-store",
            "midata-engine",
        ],
        any_internal: false,
        external: &["*"],
        dev_external: &["midata-testkit"],
        dev_only: false,
    },
    CrateRule {
        name: "xtask",
        layer: "tool",
        birth: "P0-1",
        manifest: "xtask/Cargo.toml",
        internal: &[],
        any_internal: false,
        external: &["anyhow", "serde_json"],
        dev_external: &[],
        dev_only: false,
    },
];

/// Crates that must never be a dependency of anything in this project.
pub(crate) const BANNED_ANYWHERE: &[&str] = &[
    "tauri",
    "wry",
    "webview2-com",
    "electron",
    "axum",
    "actix-web",
    "warp",
    "rocket",
    "duckdb",
    "polars",
    "diesel",
    "sqlx",
    "sea-orm",
];

/// External crates that may appear only in the named home crates.
pub(crate) const ONLY_IN: &[(&str, &[&str])] = &[
    ("egui", &["midata-app", "midata"]),
    ("eframe", &["midata-app", "midata"]),
    ("egui_plot", &["midata-app", "midata"]),
    ("egui_extras", &["midata-app", "midata"]),
    ("egui_kittest", &["midata-app", "midata-testkit"]),
    ("winit", &["midata-app", "midata"]),
    ("rfd", &["midata-app", "midata"]),
    ("tokio", &["midata-ai"]),
    ("reqwest", &["midata-ai"]),
    ("reqwest-eventsource", &["midata-ai"]),
    ("sqlparser", &["midata-queryguard"]),
    ("calamine", &["midata-exchange"]),
    ("csv", &["midata-exchange"]),
    ("rust_xlsxwriter", &["midata-exchange"]),
    ("printpdf", &["midata-exchange"]),
    ("genpdf", &["midata-exchange"]),
];

/// Crates that are application roots, never libraries.
pub(crate) const APP_CRATES: &[&str] = &["midata-app", "midata", "midata-cli"];

/// App-root crate to the app crates it may depend on (D24).
///
/// Application crates are roots, not libraries: nothing below the app layer may depend on one. The
/// one legal app-to-app edge is the UI composition root (`apps/midata`) depending on the egui shell
/// (`midata-app`) — it cannot open a window otherwise. Listing an edge here is the **only** way to
/// allow it, so `midata-cli → midata-app` stays forbidden (the headless binary must never link the
/// egui shell) even though `midata-cli` is itself an app root.
pub(crate) const APP_EDGES: &[(&str, &[&str])] = &[("midata", &["midata-app"])];

/// The app crates `package` may depend on; empty for every crate that is not a declared app root.
pub(crate) fn allowed_app_edges(package: &str) -> &'static [&'static str] {
    APP_EDGES
        .iter()
        .find(|(name, _)| *name == package)
        .map(|(_, edges)| *edges)
        .unwrap_or(&[])
}
