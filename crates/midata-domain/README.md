# midata-domain

L0 · pure · zero I/O. The shared vocabulary every other crate speaks: identity enums, the dynamic-field
vocabulary, the value-encoding rules, latest-value ordering, typed errors and the injected clock.

`REBUILD_PLAN.md` §5.3–§5.6; `AGENTS.md` §2 (workspace map) and §3 (rules 2, 3, 10, 11).

**Dependencies:** `serde`, `chrono`, `thiserror` only. No database, no filesystem, no network, no
`egui`. `xtask/src/layers/rules.rs` rejects anything else.

**Never panics.** The crate-level header denies `unwrap_used`, `expect_used`, `panic`,
`unimplemented`, `todo`, `unreachable` and `indexing_slicing` outside tests. Every fallible path
returns a `CommandError` whose message is an i18n key plus machine-readable params.

## Modules and public API

| Module | Public items | What it decides |
|---|---|---|
| `age` | `Age` | Age from `birth_date` **and nothing else** — `Age::between(birth, on) -> Option<Age>`. Age is never stored (rule 2). Leap-day births clamp by walking back to the last valid day, so a 29 Feb birth reaches 1 year on 28 Feb. |
| `cardinality` | `Cardinality` | `Single` / `Series` — the upsert vs. append rule of §5.3. `single` keeps one row per (entity, field); `series` allows one row per day. |
| `clock` | `Clock`, `SystemClock` | The clock seam (`docs/architecture.md` §9). **The only wall-clock read in the codebase** is `SystemClock::now_utc`; everything else takes `&impl Clock`. Tests inject a fixed clock, so no behaviour depends on the current time. |
| `command_error` | `CommandError`, `ErrorReason` | The one error type crossing the engine boundary. `reason` (`invalid_param` / `not_found` / `conflict` / `unsupported` / `internal`) lets a caller branch without parsing text; `i18n_key` plus `params` render the user message. Serialises as JSON for the CLI and the agent surfaces. |
| `data_type` | `DataType`, `ValueColumn` | `Number`, `Text`, `Boolean`, `Date`, `Select`, `MultiSelect` → which observation column holds it (rule 3). |
| `latest_value` | `ObservationStamp`, `latest_by_stamp` | "Current value" = max `recorded_on`, tie-break max `id` (§5.4). One implementation, reused by store and analytics — never re-derived. |
| `money` | `Money`, `Currency` | Integer minor units (`i64`) + currency code, default TND. `checked_add` / `checked_sub` return `err.money.currency_mismatch` or `err.money.overflow`; **no floats, ever** (rule 11). |
| `pedigree` | `PedigreeBounds`, `MIN_GENERATIONS`, `DEFAULT_GENERATIONS`, `MAX_GENERATIONS`, `MAX_NODES` | The bounds every pedigree traversal must carry: depth 1–10, 500-node global cap (rule: bound recursion). |
| `sex` | `Sex` | `Male` / `Female`. Sire must be male, dam female — enforced by the commands, expressed here. |
| `species` | `SpeciesKey` | An **open registry key**, not an enum: `parse` accepts lowercase letters, digits, `_`, `-`, ≤ 32 bytes. Adding a species is data, never a code change (rule 4). |
| `status` | `Status`, `validate_exit_date` | `Alive` / `Sold` / `Dead` / … plus the exit-date rule: an animal that left needs a date, an animal still in the herd must not have one. |
| `value` | `Value`, `EncodedValue`, `MAX_MULTI_SELECT_ITEMS` | Typed value ↔ stored columns. `Value::encode(data_type)` and `Value::decode(data_type, encoded)` uphold **exactly one non-NULL column**, finite numbers only, no empty or duplicate choices, ≤ 256 choices, ISO dates. |

`json_array` is private: a strict, bounded codec for the `multi_select` JSON array, because persisting
`serde_json` in this layer is not permitted. It rejects unpaired surrogates and raw control characters.

## Example

```rust
use midata_domain::{
    Clock, CommandError, DataType, EncodedValue, SpeciesKey, Status, SystemClock, Value,
    validate_exit_date,
};

fn example() -> Result<(), CommandError> {
    let species = SpeciesKey::parse("camel")?;      // open registry: no code change needed
    let encoded = Value::Text("Baya".into()).encode(DataType::Text)?;
    assert_eq!(encoded, EncodedValue::Text("Baya".into()));

    let today = SystemClock.now_utc().date_naive(); // Clock seam, never `Utc::now()` here
    let _ = (species, today);

    validate_exit_date(Status::Alive, None)?;       // an animal in the herd has no exit date
    Ok(())
}
```

## Adding to this crate

Add a type only when it is pure vocabulary shared by at least two layers. Persistence belongs in
`midata-store`, computation in `midata-analytics`, workflows in `midata-ops`. A new enum variant that
nothing branches on is dead schema by another name (anti-pattern register, `REBUILD_PLAN.md` §3).
