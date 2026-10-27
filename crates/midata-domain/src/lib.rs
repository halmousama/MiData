#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unimplemented,
    clippy::todo,
    clippy::unreachable,
    clippy::indexing_slicing
)]
//! `midata-domain` — the shared vocabulary of MiData (L0: pure, zero I/O).
//!
//! Identity types (`Sex`, `Status`, `SpeciesKey`), the dynamic-field vocabulary (`DataType`,
//! `Cardinality`), the value-encoding rules of `REBUILD_PLAN.md` §5.3, the latest-value ordering of
//! §5.4, typed [`CommandError`]s and the injected [`Clock`] seam. No database, no filesystem, no
//! network, and no wall-clock read outside [`SystemClock`] — that is what lets the UI, the CLI, the
//! AI tool loop and the tests share one definition of "what a weight is".
//!
//! Dependencies are limited to `serde`, `chrono` and `thiserror` (AGENTS.md §2); `xtask/src/layers.rs`
//! enforces it.

pub mod age;
pub mod cardinality;
pub mod clock;
pub mod command_error;
pub mod data_type;
mod json_array;
pub mod latest_value;
pub mod money;
pub mod pedigree;
pub mod sex;
pub mod species;
pub mod status;
pub mod value;

pub use age::Age;
pub use cardinality::Cardinality;
pub use clock::{Clock, SystemClock};
pub use command_error::{CommandError, ErrorReason};
pub use data_type::{DataType, ValueColumn};
pub use latest_value::{latest_by_stamp, ObservationStamp};
pub use money::{Currency, Money};
pub use pedigree::{
    PedigreeBounds, DEFAULT_GENERATIONS, MAX_GENERATIONS, MAX_NODES, MIN_GENERATIONS,
};
pub use sex::Sex;
pub use species::SpeciesKey;
pub use status::{validate_exit_date, Status};
pub use value::{EncodedValue, Value, MAX_MULTI_SELECT_ITEMS};
