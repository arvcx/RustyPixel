//! RustyPixel client.
//!
//! Phase 1: renders the scene background and the player's idle pose with
//! crisp, integer-pixel scaling.

mod background;
mod camera;
mod player;

use bevy::image::ImagePlugin;
use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin, WindowResolution};

use crate::camera::{LOGICAL_HEIGHT, LOGICAL_WIDTH, WINDOW_SCALE};

fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("RustyPixel client starting...");

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "RustyPixel".into(),
                        resolution: WindowResolution::new(
                            LOGICAL_WIDTH as u32 * WINDOW_SCALE,
                            LOGICAL_HEIGHT as u32 * WINDOW_SCALE,
                        ),
                        ..default()
                    }),
                    ..default()
                }),
        )
        // Night-sky backdrop shown wherever the scene doesn't cover the view.
        .insert_resource(ClearColor(Color::srgb(0.07, 0.09, 0.13)))
        .add_systems(
            Startup,
            (
                background::spawn_background,
                camera::setup_camera,
                player::spawn_player,
            ),
        )
        .run();
}