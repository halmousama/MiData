//! Bounds for pedigree traversal, before the traversal exists.
//!
//! Ancestor graphs contain cycles in real herd data (a data-entry mistake, not an impossibility), so
//! the depth is a validated type and the node count a documented constant. `midata-analytics` (P3)
//! takes a [`PedigreeBounds`] and cannot invent a depth of its own; the visited set and the 500-node
//! cap are part of the same contract (`REBUILD_PLAN.md` §7, §11.3).

use crate::command_error::CommandError;

/// i18n key for a pedigree depth outside the supported range.
pub const ERR_GENERATIONS_OUT_OF_RANGE: &str = "err.pedigree.generations_out_of_range";

/// Shallowest useful pedigree: parents only.
pub const MIN_GENERATIONS: u8 = 1;
/// Deepest supported pedigree.
pub const MAX_GENERATIONS: u8 = 10;
/// Default depth when a caller does not choose one.
pub const DEFAULT_GENERATIONS: u8 = 3;
/// Hard cap on the number of nodes any pedigree traversal may materialise.
pub const MAX_NODES: usize = 500;

/// A validated pedigree depth (`1..=10`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct PedigreeBounds {
    generations: u8,
}

impl PedigreeBounds {
    /// Validate a requested depth — out of range is an invalid-param error, never a clamp.
    pub fn new(generations: u8) -> Result<Self, CommandError> {
        if !(MIN_GENERATIONS..=MAX_GENERATIONS).contains(&generations) {
            return Err(CommandError::invalid_param(ERR_GENERATIONS_OUT_OF_RANGE)
                .with_param("min", MIN_GENERATIONS.to_string())
                .with_param("max", MAX_GENERATIONS.to_string()));
        }
        Ok(Self { generations })
    }

    /// The depth this bounds object carries.
    pub const fn generations(self) -> u8 {
        self.generations
    }

    /// The node cap that always applies, whatever the depth.
    pub const fn max_nodes(self) -> usize {
        MAX_NODES
    }
}

impl Default for PedigreeBounds {
    fn default() -> Self {
        Self {
            generations: DEFAULT_GENERATIONS,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_range_is_inclusive_and_never_clamped() {
        assert!(PedigreeBounds::new(MIN_GENERATIONS).is_ok());
        assert!(PedigreeBounds::new(MAX_GENERATIONS).is_ok());
        for out_of_range in [MIN_GENERATIONS - 1, MAX_GENERATIONS + 1] {
            let error = PedigreeBounds::new(out_of_range).expect_err("out of range");
            assert_eq!(error.i18n_key(), ERR_GENERATIONS_OUT_OF_RANGE);
        }
        assert_eq!(PedigreeBounds::default().max_nodes(), MAX_NODES);
    }
}
