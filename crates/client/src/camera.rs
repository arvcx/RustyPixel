//! Camera setup and the logical/physical viewport constants.

use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy::render::view::Msaa;

use crate::player::Player;

/// Logical viewport width in world units (matches `background.png`, 384px).
pub const LOGICAL_WIDTH: f32 = 384.0;
/// Logical viewport height in world units (matches `background.png`, 216px).
pub const LOGICAL_HEIGHT: f32 = 216.0;
/// Integer pixel upscale factor applied to the window (1152x648 physical).
pub const WINDOW_SCALE: u32 = 3;

/// Spawns the 2D camera at a fixed logical resolution so every sprite texel
/// maps to exactly one logical pixel, which is then integer-upscaled 3x to
/// the window — pixel-perfect rendering.
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            // Fixed logical viewport: 384x216, identical to the background.
            scaling_mode: ScalingMode::Fixed {
                width: LOGICAL_WIDTH,
                height: LOGICAL_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        }),
        // Crisp pixel edges: no multi-sample anti-aliasing. This component is
        // auto-required on `Camera` (default `Sample4`); the explicit value
        // below overrides it.
        Msaa::Off,
    ));
}

/// Follow the player with the camera, clamped so the view stays inside the
/// world bounds. The world (48x27 tiles) is larger than one view, so the
/// camera moves within the 768x432 world.
pub fn camera_follow(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    player: Query<&Transform, With<Player>>,
    world: Res<crate::WorldResource>,
) {
    let Ok(mut cam_tf) = camera.single_mut() else {
        return;
    };
    let Ok(player_tf) = player.single() else {
        return;
    };

    // World half-extents minus half a view = max camera travel.
    let hw = world.0.width as f32 * crate::TILE / 2.0;
    let hh = world.0.height as f32 * crate::TILE / 2.0;
    let lim_x = hw - LOGICAL_WIDTH / 2.0;
    let lim_y = hh - LOGICAL_HEIGHT / 2.0;

    cam_tf.translation.x = player_tf.translation.x.clamp(-lim_x, lim_x);
    cam_tf.translation.y = player_tf.translation.y.clamp(-lim_y, lim_y);
}