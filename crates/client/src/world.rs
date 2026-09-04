//! World rendering: spawns one sprite per solid tile from the engine grid.

use bevy::prelude::*;
use rustypixel_engine::world::{GRASS, PLATFORM, STONE};
use rustypixel_engine::world::TileId;

use crate::{TILE, WorldResource};

/// Tile color per type. Grass is green, stone gray, platforms warm light
/// stone; unknown tiles render magenta so they are obvious.
fn tile_color(tile: TileId) -> Color {
    match tile {
        GRASS => Color::srgb(0.36, 0.56, 0.28),
        STONE => Color::srgb(0.5, 0.5, 0.55),
        PLATFORM => Color::srgb(0.72, 0.6, 0.45),
        _ => Color::srgb(1.0, 0.0, 1.0),
    }
}

/// Render marker on each spawned tile entity.
///
/// The `x`/`y` fields are intentionally unused in this phase — they will be
/// needed for block interaction (Phase 4).
#[derive(Component)]
#[allow(dead_code)]
pub struct Tile {
    pub x: u32,
    pub y: u32,
}

impl Tile {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as u32,
            y: y as u32,
        }
    }
}

/// Spawn a sprite for every solid tile in the world grid. Uses plain colored
/// sprites (no texture) — the solid-color world is intentional for Phase 3.
pub fn spawn_world(mut commands: Commands, world: Res<WorldResource>) {
    let w = world.0.width as usize;
    let h = world.0.height as usize;

    let mut sprites: Vec<(Sprite, Transform, Tile)> = Vec::with_capacity(w * h);
    for ty in 0..h {
        for tx in 0..w {
            let tile = world.0.tile_at(tx as i32, ty as i32);
            if tile == rustypixel_engine::world::AIR {
                continue;
            }
            let pos = tile_center(tx as i32, ty as i32, &world);
            sprites.push((
                Sprite::from_color(tile_color(tile), Vec2::splat(TILE)),
                Transform::from_xyz(pos.x, pos.y, 1.0),
                Tile::new(tx, ty),
            ));
        }
    }
    // The world is up to ~1300 solid tiles; batch spawning keeps startup fast.
    commands.spawn_batch(sprites);
}

/// World position of the center of tile column `tx`, row `ty`. Row 0 is the
/// top of the world; y grows upward in world space.
pub fn tile_center(tx: i32, ty: i32, world: &WorldResource) -> Vec2 {
    let x = (tx as f32 + 0.5) * TILE - (world.0.width as f32 * TILE) / 2.0;
    let y = (world.0.height as f32 * TILE) / 2.0 - (ty as f32 + 0.5) * TILE;
    Vec2::new(x, y)
}