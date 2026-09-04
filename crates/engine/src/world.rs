//! World storage. Stub only for Phase 0; tile grid, chunks and saving arrive
//! in Phase 3.

/// Placeholder for the world state.
pub struct World {
    /// Placeholder dimension of the world in blocks.
    pub size: u32,
}

impl World {
    /// Create a placeholder world.
    pub fn new(size: u32) -> Self {
        Self { size }
    }
}