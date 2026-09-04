//! The scene backdrop.

use bevy::prelude::*;

/// The world is 768x432 (48x27 tiles of 16px), which is larger than one
/// 384x216 background image. Two copies of the backdrop, scaled 2x and placed
/// side by side, tile seamlessly across the whole world. The texture is
/// vertically repetitive by design, so the 2x vertical stretch blends in.
pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image: Handle<Image> = asset_server.load("textures/background.png");
    let scale = Vec3::new(2.0, 2.0, 1.0);

    commands.spawn((
        Sprite {
            image: image.clone(),
            ..default()
        },
        // Left copy: covers x[-384,0].
        Transform::from_scale(scale).with_translation(Vec3::new(-192.0, 0.0, 0.0)),
    ));

    commands.spawn((
        Sprite {
            image,
            ..default()
        },
        // Right copy: covers x[0,384].
        Transform::from_scale(scale).with_translation(Vec3::new(192.0, 0.0, 0.0)),
    ));
}