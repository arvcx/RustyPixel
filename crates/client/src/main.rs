//! RustyPixel client.
//!
//! Phase 2+3: player controller (movement, jump, gravity, collision, ground
//! detection, animation states) and the tile world (rendering + physics).

mod background;
mod camera;
mod player;
mod player_ui;
mod world;

use bevy::app::AppExit;
use bevy::image::ImagePlugin;
use bevy::input_focus::tab_navigation::TabNavigationPlugin;
use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin, WindowResolution};
use rustypixel_engine::world::WorldGrid;

use crate::camera::{LOGICAL_HEIGHT, LOGICAL_WIDTH, WINDOW_SCALE};
use crate::player_ui::ChatState;

/// Size of one tile in world units.
pub const TILE: f32 = 16.0;

/// Path of the world save file (TOML), relative to the working directory.
const WORLD_SAVE_PATH: &str = "world.toml";

/// The world grid shared by rendering, physics and (de)serialization.
#[derive(Resource)]
pub struct WorldResource(pub WorldGrid);

/// Whether a world save is still pending (once per session).
#[derive(Resource)]
struct PendingSave(bool);

/// Load the world grid: from `world.toml` if present, otherwise generate the
/// default deterministic world and immediately persist it.
fn load_world(mut commands: Commands) {
    let grid = match WorldGrid::load(WORLD_SAVE_PATH) {
        Ok(grid) => {
            tracing::info!("world loaded from {WORLD_SAVE_PATH}");
            grid
        }
        Err(err) => {
            tracing::warn!("no world file ({}); generating default world", err);
            let grid = WorldGrid::generate_default();
            if let Err(save_err) = grid.save(WORLD_SAVE_PATH) {
                tracing::warn!("could not persist default world: {save_err}");
            }
            grid
        }
    };
    commands.insert_resource(WorldResource(grid));
    commands.insert_resource(PendingSave(false));
}

/// Listen for app exit and write the world grid once (auto-save on close).
fn save_world_on_exit(
    exit: MessageReader<AppExit>,
    world: Res<WorldResource>,
    mut pending: ResMut<PendingSave>,
) {
    if exit.is_empty() {
        return;
    }
    if !pending.0 {
        match world.0.save(WORLD_SAVE_PATH) {
            Ok(()) => tracing::info!("world saved to {WORLD_SAVE_PATH}"),
            Err(err) => tracing::error!("failed to save world: {err}"),
        }
        pending.0 = true;
    }
}

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
        // Tab-indexed focus navigation for the chat input widget.
        .add_plugins(TabNavigationPlugin)
        // Fixed 60 Hz physics timestep; `Res<Time>` inside FixedUpdate systems
        // resolves to `Time<Fixed>` automatically.
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        // Night-sky backdrop shown wherever the scene doesn't cover the view.
        .insert_resource(ClearColor(Color::srgb(0.07, 0.09, 0.13)))
        // Chat overlay state (open flag + current message bubble).
        .insert_resource(ChatState::default())
        .add_systems(
            Startup,
            (
                load_world,
                (
                    background::spawn_background,
                    camera::setup_camera,
                    world::spawn_world,
                    player::spawn_player,
                    player_ui::spawn_overhead_ui,
                )
                    .chain(),
            )
                .chain(),
        )
        // Physics: deterministic 60 Hz fixed-step.
        .add_systems(
            FixedUpdate,
            (
                player::apply_gravity,
                player::player_control,
                player::integrate_and_collide,
            )
                .chain(),
        )
        // Presentation / state: per-frame.
        .add_systems(
            Update,
            (
                player::update_anim_state,
                player::animate_rect,
                camera::camera_follow,
                player_ui::chat_toggle,
                player_ui::chat_submit,
                player_ui::bubble_expiry,
                save_world_on_exit,
            ),
        )
        .run();
}