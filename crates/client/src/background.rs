//! The scene backdrop.

use bevy::prelude::*;

/// Spawns the full-screen background sprite. It is 384x216 and rendered
/// unscaled, so it covers the fixed 384x216 logical viewport exactly.
pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("textures/background.png"),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}