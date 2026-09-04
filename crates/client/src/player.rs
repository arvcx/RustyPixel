//! Player controller: input, gravity, hand-rolled AABB-vs-grid collision,
//! ground detection and animation state.

use bevy::prelude::*;

use crate::world::tile_center;
use crate::{TILE, WorldResource};

// ---------------------------------------------------------------------------
// Physics constants
// ---------------------------------------------------------------------------

/// AABB extents used for collision. Slightly smaller than the visible sprite
/// so the player can squeeze past tile corners fairly.
pub const PLAYER_SIZE: Vec2 = Vec2::new(12.0, 30.0);
/// Horizontal speed while walking/air-strafing.
pub const MOVE_SPEED: f32 = 120.0;
/// Upward velocity applied on jump.
pub const JUMP_VELOCITY: f32 = 260.0;
/// Gravitational acceleration (world units / s²).
pub const GRAVITY: f32 = -520.0;
/// Terminal fall speed.
pub const MAX_FALL: f32 = -340.0;

// ---------------------------------------------------------------------------
// Animation rects — verified against `textures/player.png` (128x288, six
// bands). Bevy rects are top-left origin, in texture pixels.
// ---------------------------------------------------------------------------

pub const IDLE_FRAMES: [Rect; 2] = [
    Rect::new(9.0, 1.0, 22.0, 42.0),
    Rect::new(41.0, 1.0, 54.0, 42.0),
];
pub const WALK_FRAMES: [Rect; 4] = [
    Rect::new(9.0, 49.0, 22.0, 92.0),
    Rect::new(41.0, 49.0, 54.0, 92.0),
    Rect::new(73.0, 49.0, 86.0, 92.0),
    Rect::new(105.0, 49.0, 118.0, 92.0),
];
pub const JUMP_FRAME: Rect = Rect::new(8.0, 96.0, 23.0, 136.0);
pub const FALL_FRAME: Rect = Rect::new(8.0, 145.0, 23.0, 186.0);

// ---------------------------------------------------------------------------
// Components & state
// ---------------------------------------------------------------------------

/// Marker for the local player entity, plus ground-contact flag.
#[derive(Component)]
pub struct Player {
    pub grounded: bool,
}

/// Velocity of the player, applied during fixed-step integration.
#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

/// Animation state machine.
#[derive(Component, Default, Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
    #[default]
    Idle,
    Walk,
    Jump,
    Fall,
}

/// Which animation is playing and how far along it is.
#[derive(Component)]
pub struct AnimationConfig {
    pub timer: Timer,
    pub frame: usize,
}

impl AnimationConfig {
    fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Repeating),
            frame: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// Spawning
// ---------------------------------------------------------------------------

/// Spawn the player standing on the ground near the middle of the world.
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    world: Res<WorldResource>,
) {
    // Find the surface row at column 24 by scanning downward for the first
    // solid tile, then place the player's center just above the surface so it
    // drops in and settles onto the ground.
    let surface_ty = (0..world.0.height as i32)
        .find(|&ty| world.0.is_solid_at(24, ty))
        .unwrap_or(world.0.height as i32);
    let surface_y = tile_center(24, surface_ty, &world).y;
    let spawn_y = surface_y + 2.0 * TILE - PLAYER_SIZE.y / 2.0;

    commands.spawn((
        Sprite {
            image: asset_server.load("textures/player.png"),
            rect: Some(IDLE_FRAMES[0]),
            // Fixed rendered size so per-frame rect height differences don't
            // change how big the character looks.
            custom_size: Some(Vec2::new(16.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, spawn_y, 2.0),
        Player { grounded: false },
        Velocity(Vec2::ZERO),
        PlayerState::Idle,
        AnimationConfig::new(0.35),
    ));
}

// ---------------------------------------------------------------------------
// Fixed-step physics systems (run in `FixedUpdate`)
// ---------------------------------------------------------------------------

/// Apply gravity and clamp terminal velocity.
pub fn apply_gravity(mut query: Query<&mut Velocity>, time: Res<Time>) {
    let dt = time.delta_secs();
    for mut velocity in &mut query {
        velocity.0.y = (velocity.0.y + GRAVITY * dt).max(MAX_FALL);
    }
}

/// Read keyboard input and set horizontal velocity; jump on Space when
/// grounded.
pub fn player_control(
    mut query: Query<(&mut Velocity, &Player)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let right = keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight);
    let left = keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft);
    let horizontal = (right as i8 - left as i8) as f32 * MOVE_SPEED;

    for (mut velocity, player) in &mut query {
        velocity.0.x = horizontal;
        if keys.just_pressed(KeyCode::Space) && player.grounded {
            velocity.0.y = JUMP_VELOCITY;
        }
    }
}

/// Integrate velocity into position, resolving collisions axis-by-axis
/// against the solid tile grid.
pub fn integrate_and_collide(
    mut query: Query<(&mut Transform, &mut Velocity, &mut Player)>,
    world: Res<WorldResource>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    let half = PLAYER_SIZE / 2.0;

    for (mut tf, mut velocity, mut player) in &mut query {
        let mut pos = tf.translation.truncate();

        // --- X axis -------------------------------------------------
        pos.x += velocity.0.x * dt;
        let min_x = pos.x - half.x;
        let max_x = pos.x + half.x;
        let top_y = pos.y - half.y;
        let bot_y = pos.y + half.y;
        let x0 = tile_x_of(&world, min_x);
        let x1 = tile_x_of(&world, max_x);
        let y0 = tile_y_of(&world, top_y);
        let y1 = tile_y_of(&world, bot_y);
        let mut blocked = false;
        if velocity.0.x > 0.0 {
            for ty in y0..=y1 {
                if world.0.is_solid_at(x1, ty) {
                    blocked = true;
                    break;
                }
            }
            if blocked {
                let tile_left = tile_center(x1, y0, &world).x - TILE / 2.0;
                pos.x = tile_left - half.x;
                velocity.0.x = 0.0;
            }
        } else if velocity.0.x < 0.0 {
            for ty in y0..=y1 {
                if world.0.is_solid_at(x0, ty) {
                    blocked = true;
                    break;
                }
            }
            if blocked {
                let tile_right = tile_center(x0, y0, &world).x + TILE / 2.0;
                pos.x = tile_right + half.x;
                velocity.0.x = 0.0;
            }
        }

        // --- Y axis -------------------------------------------------
        pos.y += velocity.0.y * dt;
        let min_y = pos.y - half.y;
        let max_y = pos.y + half.y;
        let min_x = pos.x - half.x;
        let max_x = pos.x + half.x;
        let x0 = tile_x_of(&world, min_x);
        let x1 = tile_x_of(&world, max_x);
        let y0 = tile_y_of(&world, min_y);
        let y1 = tile_y_of(&world, max_y);
        player.grounded = false;
        if velocity.0.y > 0.0 {
            // Moving up: check the top edge.
            let mut blocked = false;
            for tx in x0..=x1 {
                if world.0.is_solid_at(tx, y0) {
                    blocked = true;
                    break;
                }
            }
            if blocked {
                let tile_bottom = tile_center(x0, y0, &world).y + TILE / 2.0;
                pos.y = tile_bottom + half.y;
                velocity.0.y = 0.0;
            }
        } else if velocity.0.y < 0.0 {
            // Moving down: check the bottom edge — landing.
            let mut blocked = false;
            for tx in x0..=x1 {
                if world.0.is_solid_at(tx, y1) {
                    blocked = true;
                    break;
                }
            }
            if blocked {
                let tile_top = tile_center(x0, y1, &world).y - TILE / 2.0;
                pos.y = tile_top - half.y;
                velocity.0.y = 0.0;
                player.grounded = true;
            }
        }

        tf.translation.x = pos.x;
        tf.translation.y = pos.y;
    }
}

// ---------------------------------------------------------------------------
// Animation systems (run in `Update`)
// ---------------------------------------------------------------------------

/// Derive the current animation state (Idle/Walk/Jump/Fall) from velocity and
/// ground contact, and flip the sprite to face the movement direction.
pub fn update_anim_state(
    mut query: Query<(&Velocity, &Player, &mut PlayerState, &mut Sprite)>,
) {
    for (velocity, player, mut state, mut sprite) in &mut query {
        let vx = velocity.0.x;
        let vy = velocity.0.y;
        *state = if player.grounded {
            if vx == 0.0 {
                PlayerState::Idle
            } else {
                sprite.flip_x = vx < 0.0;
                PlayerState::Walk
            }
        } else if vy > 0.0 {
            PlayerState::Jump
        } else {
            PlayerState::Fall
        };
        // Keep the last facing direction while airborne or stopped.
        if vx < 0.0 {
            sprite.flip_x = true;
        }
    }
}

/// Advance the sprite-sheet animation based on the current state.
pub fn animate_rect(
    mut query: Query<(&mut Sprite, &mut AnimationConfig, &PlayerState)>,
    time: Res<Time>,
) {
    let dt = time.delta();
    for (mut sprite, mut anim, state) in &mut query {
        anim.timer.tick(dt);
        let frames: &[Rect] = match state {
            PlayerState::Idle => &IDLE_FRAMES,
            PlayerState::Walk => &WALK_FRAMES,
            PlayerState::Jump => &[JUMP_FRAME],
            PlayerState::Fall => &[FALL_FRAME],
        };
        if anim.timer.just_finished() {
            anim.frame = (anim.frame + 1) % frames.len();
        }
        // Keep the index in range when switching from a longer animation to a
        // shorter one (e.g. walk frame 3 -> idle).
        anim.frame %= frames.len();
        sprite.rect = Some(frames[anim.frame]);
    }
}

// ---------------------------------------------------------------------------
// Coordinate helpers (shared tile-math, inverse mapping not duplicated)
// ---------------------------------------------------------------------------

/// Column of the tile containing world-space x.
fn tile_x_of(world: &WorldResource, x: f32) -> i32 {
    (x + (world.0.width as f32 * TILE) / 2.0).div_euclid(TILE) as i32
}

/// Row of the tile containing world-space y (row 0 at the top).
fn tile_y_of(world: &WorldResource, y: f32) -> i32 {
    ((world.0.height as f32 * TILE) / 2.0 - y).div_euclid(TILE) as i32
}