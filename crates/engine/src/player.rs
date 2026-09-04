//! Player management. Stub only for Phase 0; the full player system is
//! described in PRD §9.

/// Placeholder for a player.
pub struct Player {
    /// Placeholder display name.
    pub name: String,
}

impl Player {
    /// Create a placeholder player.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}