//! The player character. Phase 1 shows only the idle pose: the first band of
//! the sprite sheet, cropped via `Sprite.rect`. Animation arrives in Phase 2.

use bevy::prelude::*;

use crate::camera::LOGICAL_HEIGHT;

/// Crop window of the first band of `textures/player.png` (top 42 rows,
/// full 128px width). In Bevy, sprite rects use texture pixel coordinates
/// with the origin at the image's top-left corner.
const PLAYER_RECT: Rect = Rect::new(0.0, 0.0, 128.0, 42.0);

/// Target rendered height of the player in logical pixels. The 42px band is
/// scaled down to this so the character reads as a small figure.
const PLAYER_TARGET_HEIGHT: f32 = 32.0;

/// Vertical position of the player's anchor (its center). The sprite is
/// bottom-touching: its feet rest on the bottom edge of the logical view.
const PLAYER_Y: f32 = -LOGICAL_HEIGHT / 2.0 + PLAYER_TARGET_HEIGHT / 2.0;

/// Marker component identifying the local player entity.
#[derive(Component)]
pub struct Player;

/// Spawns the player sprite, cropped to the idle band of the sprite sheet.
pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("textures/player.png"),
            rect: Some(PLAYER_RECT),
            ..default()
        },
        Transform::from_xyz(0.0, PLAYER_Y, 1.0)
            .with_scale(Vec3::splat(PLAYER_TARGET_HEIGHT / PLAYER_RECT.height())),
        Player,
    ));
}