//! Tile world: grid data model plus save/load. Engine-side data only — this
//! crate has no Bevy dependency; the client renders and applies physics over
//! this grid.

use std::path::Path;

use serde::{Deserialize, Serialize};

/// Identifier for a tile type.
pub type TileId = u16;

/// Empty tile — not solid.
pub const AIR: TileId = 0;
/// Grass block (solid), sits on top of the underground.
pub const GRASS: TileId = 1;
/// Stone block (solid), the underground material.
pub const STONE: TileId = 2;
/// Floating platform block (solid), used for platforms.
pub const PLATFORM: TileId = 3;

/// Whether a tile blocks movement. AIR is passable; everything else is solid.
pub fn is_solid(tile: TileId) -> bool {
    match tile {
        AIR => false,
        GRASS | STONE | PLATFORM => true,
        _ => true, // Unknown tiles are treated as solid for safety.
    }
}

/// A rectangular grid of tiles. Row 0 is the top of the world; `tiles` is
/// stored row-major (`index = y * width + x`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldGrid {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<TileId>,
}

impl WorldGrid {
    /// Create an empty grid (all AIR) of the given dimensions.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            tiles: vec![AIR; (width * height) as usize],
        }
    }

    /// Read the tile at grid column `x`, row `y`.
    ///
    /// Out-of-bounds coordinates return `STONE` — the area outside the world
    /// is treated as solid wall so entities can never leave the grid.
    pub fn tile_at(&self, x: i32, y: i32) -> TileId {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return STONE;
        }
        self.tiles[y as usize * self.width as usize + x as usize]
    }

    /// Set the tile at column `x`, row `y` (in-bounds only; out of range is
    /// ignored).
    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileId) {
        if x < self.width as usize && y < self.height as usize {
            let idx = y * self.width as usize + x;
            self.tiles[idx] = tile;
        }
    }

    /// Whether the tile at column `x`, row `y` blocks movement.
    pub fn is_solid_at(&self, x: i32, y: i32) -> bool {
        is_solid(self.tile_at(x, y))
    }

    /// Build the fixed deterministic default world: 48 wide x 27 tall.
    ///
    /// Layout:
    /// - The bottom 7 rows (rows 20..26 inclusive) are solid STONE, with the
    ///   top surface row (row 20) surfaced with GRASS.
    /// - Three floating PLATFORM runs.
    /// - A single STONE pillar as a jump obstacle.
    ///
    /// Everything above is AIR, so the player spawns in clear space.
    pub fn generate_default() -> Self {
        const W: u32 = 48;
        const H: u32 = 27;
        let mut grid = Self::new(W, H);

        // Underground + floor (rows 20..26) with grass on the surface.
        for y in 20..27 {
            for x in 0..W as usize {
                grid.set_tile(x, y, STONE);
            }
        }
        for x in 0..W as usize {
            grid.set_tile(x, 20, GRASS);
        }

        // Floating platforms.
        for x in 8..15 {
            grid.set_tile(x, 16, PLATFORM);
        }
        for x in 26..35 {
            grid.set_tile(x, 13, PLATFORM);
        }
        for x in 40..48 {
            grid.set_tile(x, 10, PLATFORM);
        }

        // A pillar as a jump obstacle (column 46, rows 6..14) — tall enough
        // that you must platform up onto the third run to hop over it.
        for y in 6..15 {
            grid.set_tile(46, y, STONE);
        }

        grid
    }

    /// Serialize this grid to TOML and write it to `path`.
    ///
    /// Missing or unwritable paths surface an [`std::io::Result`] error. There
    /// is no world asset yet, so a manually-placed `world.toml` or the first
    /// run's generated world both load fine.
    pub fn save(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let toml =
            toml::to_string(self).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(path, toml)
    }

    /// Load a grid from a TOML file at `path`.
    ///
    /// Returns a descriptive error if the file is missing or invalid.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, WorldLoadError> {
        let raw = std::fs::read_to_string(path)?;
        let grid: WorldGrid = toml::from_str(&raw)?;
        Ok(grid)
    }
}

/// Error type for [`WorldGrid::load`].
#[derive(Debug)]
pub enum WorldLoadError {
    /// The file could not be read.
    Io(std::io::Error),
    /// The file contents could not be parsed as a [`WorldGrid`].
    Toml(toml::de::Error),
}

impl std::fmt::Display for WorldLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorldLoadError::Io(e) => write!(f, "failed to read world file: {e}"),
            WorldLoadError::Toml(e) => write!(f, "failed to parse world file: {e}"),
        }
    }
}

impl std::error::Error for WorldLoadError {}

impl From<std::io::Error> for WorldLoadError {
    fn from(e: std::io::Error) -> Self {
        WorldLoadError::Io(e)
    }
}

impl From<toml::de::Error> for WorldLoadError {
    fn from(e: toml::de::Error) -> Self {
        WorldLoadError::Toml(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_grid_is_all_air() {
        let grid = WorldGrid::new(4, 3);
        assert_eq!(grid.width, 4);
        assert_eq!(grid.height, 3);
        assert!(grid.tiles.iter().all(|&t| t == AIR));
    }

    #[test]
    fn set_and_read_tile() {
        let mut grid = WorldGrid::new(4, 3);
        grid.set_tile(2, 1, GRASS);
        assert_eq!(grid.tile_at(2, 1), GRASS);
        assert_eq!(grid.tile_at(0, 0), AIR);
    }

    #[test]
    fn default_world_has_solid_bottom_and_air_top() {
        let grid = WorldGrid::generate_default();
        // Surface grass at the top of the underground.
        assert_eq!(grid.tile_at(24, 20), GRASS);
        // Below the surface is stone.
        assert_eq!(grid.tile_at(24, 26), STONE);
        // The topmost rows are open air.
        for x in 0..grid.width as i32 {
            assert_eq!(grid.tile_at(x, 0), AIR, "top row solid at x={x}");
        }
    }

    #[test]
    fn out_of_bounds_is_solid() {
        let grid = WorldGrid::new(4, 3);
        assert!(grid.is_solid_at(-1, 0));
        assert!(grid.is_solid_at(0, -1));
        assert!(grid.is_solid_at(4, 0));
        assert!(grid.is_solid_at(0, 3));
    }

    #[test]
    fn save_load_round_trip_preserves_grid() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("rustypixel_test_world_{}.toml", std::process::id()));
        let grid = WorldGrid::generate_default();
        grid.save(&path).expect("save");

        let loaded = WorldGrid::load(&path).expect("load");
        assert_eq!(loaded.width, grid.width);
        assert_eq!(loaded.height, grid.height);
        assert_eq!(loaded.tiles, grid.tiles);

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn load_missing_file_returns_error() {
        let path = std::env::temp_dir().join("rustypixel_does_not_exist.toml");
        let _ = std::fs::remove_file(&path);
        assert!(WorldGrid::load(&path).is_err());
    }
}