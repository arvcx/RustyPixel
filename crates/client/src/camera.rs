//! Camera setup and the logical/physical viewport constants.

use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy::render::view::Msaa;

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