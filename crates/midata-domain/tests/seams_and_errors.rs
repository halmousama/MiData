//! Acceptance tests for the injection seams (`docs/architecture.md` §9) and the error contract.
//!
//! Three properties are asserted here, because each one is a legacy defect the rebuild must not
//! reproduce:
//! 1. time is injected — a caller can supply a fixed clock, and only `SystemClock` reads the wall;
//! 2. no global state exists in the crate (no `static mut`, no `OnceLock`, no `lazy_static`);
//! 3. a `CommandError` carries a machine reason, an i18n key and params — never a raw message.

// `clippy.toml` allows `expect`/`panic` in tests; the lint can only see `#[test]` functions, so the
// `date` fixture and the source-scanning helper below opt in for the whole test binary.
#![allow(clippy::expect_used)]

use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use midata_domain::{
    latest_by_stamp, Clock, CommandError, ErrorReason, ObservationStamp, SystemClock,
};

fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("valid test date")
}

/// A clock the test owns — the whole point of the `Clock` seam: no test ever depends on wall time.
struct FixedClock {
    now: DateTime<Utc>,
}

impl Clock for FixedClock {
    fn now_utc(&self) -> DateTime<Utc> {
        self.now
    }
}

fn year_of<C: Clock>(clock: &C) -> i32 {
    use chrono::Datelike;
    clock.today().year()
}

#[test]
fn a_caller_can_inject_a_fixed_clock() {
    let fixed = FixedClock {
        now: Utc
            .with_ymd_and_hms(2024, 3, 9, 23, 30, 0)
            .single()
            .expect("valid instant"),
    };
    assert_eq!(year_of(&fixed), 2024);
    assert_eq!(
        fixed.today(),
        date(2024, 3, 9),
        "the default today() derives from now_utc()"
    );
}

#[test]
fn the_system_clock_reads_the_wall_time() {
    let before = Utc::now();
    let now = SystemClock.now_utc();
    let after = Utc::now();
    assert!(now >= before, "the system clock never travels backwards");
    assert!(now <= after, "the system clock is not in the future");
    assert_eq!(SystemClock.today(), Utc::now().date_naive());
}

#[test]
fn the_crate_has_no_global_state_and_reads_wall_time_in_one_place() {
    let sources = crate_sources();
    assert!(
        !sources.is_empty(),
        "the source scan must find the crate's files"
    );

    for (name, text) in &sources {
        for banned in ["static mut", "OnceLock", "lazy_static", "thread_local!"] {
            assert!(
                !text.contains(banned),
                "{name} uses `{banned}` — dependencies are injected, never reached through a global (architecture §9)"
            );
        }
        if text.contains("Utc::now") {
            assert_eq!(
                name, "clock.rs",
                "only the SystemClock may read wall time; {name} must take a `Clock` instead"
            );
        }
    }
}

fn crate_sources() -> Vec<(String, String)> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    let mut directories = vec![root];
    while let Some(directory) = directories.pop() {
        let entries = std::fs::read_dir(&directory).expect("the crate's src directory is readable");
        for entry in entries {
            let path = entry.expect("a readable directory entry").path();
            if path.is_dir() {
                directories.push(path);
            } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
                let text = std::fs::read_to_string(&path).expect("a readable source file");
                let name = path
                    .file_name()
                    .and_then(|value| value.to_str())
                    .expect("a file name")
                    .to_string();
                files.push((name, text));
            }
        }
    }
    files
}

#[test]
fn latest_value_uses_recorded_on_then_id() {
    let observations = [
        ObservationStamp {
            recorded_on: date(2024, 1, 1),
            id: 1,
        },
        ObservationStamp {
            recorded_on: date(2024, 3, 1),
            id: 2,
        },
        // Same day as the previous row, higher id: the later write wins.
        ObservationStamp {
            recorded_on: date(2024, 3, 1),
            id: 7,
        },
        ObservationStamp {
            recorded_on: date(2023, 12, 31),
            id: 99,
        },
    ];
    let latest = latest_by_stamp(&observations, |stamp| *stamp).expect("a latest value exists");
    assert_eq!(latest.id, 7, "max recorded_on, tie-break max id");

    assert!(latest_by_stamp(&[] as &[ObservationStamp], |stamp| *stamp).is_none());
}

#[test]
fn a_command_error_carries_a_reason_key_and_params() {
    let error = CommandError::invalid_param("err.value.type_mismatch")
        .with_param("expected", "number")
        .with_param("actual", "text");
    assert_eq!(error.reason(), ErrorReason::InvalidParam);
    assert_eq!(error.i18n_key(), "err.value.type_mismatch");
    assert_eq!(error.param("expected"), Some("number"));
    assert_eq!(error.param("missing"), None);
    assert_eq!(error.params().len(), 2);

    let display = error.to_string();
    assert!(display.contains("err.value.type_mismatch"));
    assert!(
        display.contains("expected"),
        "params stay visible to developers"
    );

    for error in [
        CommandError::not_found("err.animal.not_found"),
        CommandError::conflict("err.animal.duplicate_custom_id"),
        CommandError::unsupported("err.feature.not_built"),
        CommandError::internal("err.common.internal"),
    ] {
        assert!(
            error.i18n_key().starts_with("err."),
            "every key is an i18n key"
        );
        assert!(!error.to_string().is_empty());
    }
    assert_eq!(ErrorReason::NotFound.as_str(), "not_found");
    assert_eq!(ErrorReason::Conflict.as_str(), "conflict");
    assert_eq!(ErrorReason::Unsupported.as_str(), "unsupported");
    assert_eq!(ErrorReason::Internal.as_str(), "internal");
}

#[test]
fn a_command_error_round_trips_through_json_for_agents() {
    let error = CommandError::conflict("err.money.currency_mismatch").with_param("left", "TND");
    let json = serde_json::to_string(&error).expect("an error is agent-readable JSON");
    let back: CommandError = serde_json::from_str(&json).expect("an error deserializes");
    assert_eq!(back, error);
    assert!(
        json.contains("\"reason\":\"conflict\""),
        "the reason class is machine-readable: {json}"
    );
}
