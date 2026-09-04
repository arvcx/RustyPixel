//! Player overhead UI: the world-space name tag with the Indonesian flag, the
//! chat box (T opens / Enter sends / Escape closes), and the transient message
//! bubble that appears above the player's head.

use bevy::asset::RenderAssetUsages;
use bevy::image::ImageSampler;
use bevy::input_focus::AutoFocus;
use bevy::input_focus::tab_navigation::TabIndex;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite::{Anchor, Text2d, Text2dShadow};
use bevy::text::{EditableText, TextCursorStyle};
use bevy::ui_widgets::SelectAllOnFocus;

use crate::player::Player;

// ---------------------------------------------------------------------------
// Identity + chat copy
// ---------------------------------------------------------------------------

/// The player's name, shown above the sprite head.
pub const PLAYER_NAME: &str = "@Arvcx";

/// Placeholder text pre-filled (and pre-selected) in the chat input.
pub const CHAT_PLACEHOLDER: &str = "Type a message...";

/// How long a message bubble stays on screen before disappearing.
pub const BUBBLE_SECONDS: f32 = 4.5;

// ---------------------------------------------------------------------------
// Overhead tag layout (local space, relative to the player sprite)
// ---------------------------------------------------------------------------

/// Half the sprite height (sprite is 16x32) — the head-top in local space.
const HALF_SPRITE_H: f32 = 16.0;

/// Where the name tag's top edge sits above the head.
const NAME_TOP_Y: f32 = HALF_SPRITE_H + 8.0;
/// Height of the name tag glyph line (font size 8).
const NAME_HEIGHT: f32 = 8.0;

/// Where the message bubble container sits (anchor point), above the name.
const BUBBLE_CONTAINER_Y: f32 = HALF_SPRITE_H + 24.0;
/// Bubble panel size (height fixed; width derived from the message length).
const BUBBLE_PANEL_H: f32 = 13.0;

// ---------------------------------------------------------------------------
// Flag (procedural, no asset file)
// ---------------------------------------------------------------------------

/// Display size of the flag in logical pixels.
const FLAG_SIZE: Vec2 = Vec2::new(24.0, 15.0);
/// Source texture size. Scaled up 1.5x to `FLAG_SIZE` with nearest sampling the
/// red/white boundary still lands exactly on a display row boundary, so the
/// rendered flag keeps clean, pixel-aligned stripes.
const FLAG_TEX: UVec2 = UVec2::new(16, 10);
/// Indonesian red (#CE1126) top half, white bottom half.
const FLAG_RED: [u8; 4] = [0xCE, 0x11, 0x26, 0xFF];
const FLAG_WHITE: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];

// ---------------------------------------------------------------------------
// Components & resources
// ---------------------------------------------------------------------------

/// Marker on the chat input node so submit/close systems can find it.
#[derive(Component)]
pub struct ChatInput;

/// Timer component on the message bubble container; the container (and its
/// panel/text children) is despawned when the timer expires.
#[derive(Component)]
pub struct ChatBubble {
    pub timer: Timer,
}

/// Shared chat state. `open` mirrors a simple "ChatOpen(bool)" gate: while
/// true, movement/jump input is blocked. `bubble` tracks the entity holding
/// the current message so a new message replaces the old one.
#[derive(Resource, Default)]
pub struct ChatState {
    pub open: bool,
    pub bubble: Option<Entity>,
}

// ---------------------------------------------------------------------------
// Spawning the overhead tag (name + flag)
// ---------------------------------------------------------------------------

/// Attach the name tag and flag to the freshly spawned player. Runs right
/// after `spawn_player` in the Startup chain, so the player exists by then.
pub fn spawn_overhead_ui(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    players: Query<Entity, Added<Player>>,
) {
    let Ok(player) = players.single() else {
        return;
    };

    let flag = flag_texture(&mut images);
    commands.entity(player).with_children(|parent| {
        // Name tag: anchored at its top-center so it hangs down from a point
        // just above the sprite head and stays centered over the player.
        parent.spawn((
            Text2d::new(PLAYER_NAME),
            TextFont {
                font_size: FontSize::Px(8.0),
                ..default()
            },
            TextColor(Color::srgb(0.95, 0.95, 0.94)),
            Text2dShadow {
                offset: Vec2::new(0.0, -1.0),
                color: Color::srgba(0.0, 0.0, 0.0, 0.85),
            },
            TextLayout::justify(Justify::Center),
            Transform::from_xyz(0.0, NAME_TOP_Y, 1.0),
            Anchor::TOP_CENTER,
            Name::new("NameTag"),
        ));

        // "🇮🇩 @Arvcx": a compact identity chip. The name is centered over the
        // head and the flag tucks in on the left, sharing the name's vertical
        // span so it reads as one unit, flag first.
        // Approximate the monospace name width (6 glyphs x 4.8px at size 8) —
        // Fira Mono advances 0.6em per glyph — and park the 24px-wide flag
        // just left of it with a 2px gap.
        let name_half_w = PLAYER_NAME.chars().count() as f32 * 4.8 / 2.0;
        let flag_x = -(name_half_w + 2.0 + FLAG_SIZE.x / 2.0);
        parent.spawn((
            Sprite {
                image: flag,
                custom_size: Some(FLAG_SIZE),
                ..default()
            },
            Transform::from_xyz(flag_x, NAME_TOP_Y - NAME_HEIGHT / 2.0, 1.0),
            Name::new("PlayerFlag"),
        ));
    });
}

/// Build the Indonesian flag as a nearest-sampled 16x10 texture: red over
/// white. Scaled to `FLAG_SIZE` (24x15) it still maps to clean pixel rows.
fn flag_texture(images: &mut Assets<Image>) -> Handle<Image> {
    let w = FLAG_TEX.x as usize;
    let h = FLAG_TEX.y as usize;
    let mut data = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        let stripe = if y < h / 2 { FLAG_RED } else { FLAG_WHITE };
        for _ in 0..w {
            data.extend_from_slice(&stripe);
        }
    }
    let mut image = Image::new(
        Extent3d {
            width: FLAG_TEX.x,
            height: FLAG_TEX.y,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.sampler = ImageSampler::nearest();
    images.add(image)
}

// ---------------------------------------------------------------------------
// Chat box (open / submit / close)
// ---------------------------------------------------------------------------

/// Open the chat box on `T`. Only one box at a time; a second `T` press while
/// it is open is ignored.
pub fn chat_toggle(
    keys: Res<ButtonInput<KeyCode>>,
    mut chat: ResMut<ChatState>,
    mut commands: Commands,
) {
    if keys.just_pressed(KeyCode::KeyT) && !chat.open {
        chat.open = true;
        commands.spawn((
            ChatInput,
            Name::new("ChatBox"),
            Node {
                position_type: PositionType::Absolute,
                left: px(4.0),
                bottom: px(4.0),
                width: px(200.0),
                padding: px(6.0).all(),
                border: px(1.0).all(),
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.06, 0.09, 0.82)),
            BorderColor::all(Color::srgba(0.85, 0.85, 0.8, 0.35)),
            // Pre-fill the placeholder and select it all on focus, so typing
            // immediately replaces it with the player's message.
            EditableText::new(CHAT_PLACEHOLDER),
            TextLayout::no_wrap(),
            TextFont {
                font_size: FontSize::Px(9.0),
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.86)),
            TextCursorStyle {
                color: Color::srgb(0.98, 0.94, 0.78),
                ..default()
            },
            TabIndex(0),
            AutoFocus,
            SelectAllOnFocus,
        ));
    }
}

/// Submit on Enter, close on Escape. Submitting turns the typed message into a
/// bubble above the player; Escape discards it. Either way the box despawns.
pub fn chat_submit(
    keys: Res<ButtonInput<KeyCode>>,
    mut chat: ResMut<ChatState>,
    player: Query<Entity, With<Player>>,
    chat_input: Query<(Entity, &EditableText), With<ChatInput>>,
    mut commands: Commands,
) {
    if !chat.open {
        return;
    }
    let Ok((input_entity, editable)) = chat_input.single() else {
        return;
    };

    if keys.just_pressed(KeyCode::Escape) {
        commands.entity(input_entity).despawn();
        chat.open = false;
        return;
    }

    if !keys.just_pressed(KeyCode::Enter) || editable.is_composing() {
        return;
    }

    let message = editable.value().to_string();
    commands.entity(input_entity).despawn();
    chat.open = false;

    // The placeholder is just a hint: submit it and it behaves as empty.
    let message = message.trim();
    if message.is_empty() || message == CHAT_PLACEHOLDER {
        return;
    }

    let Ok(player_entity) = player.single() else {
        return;
    };
    spawn_bubble(&mut commands, &mut chat, player_entity, message);
}

// ---------------------------------------------------------------------------
// Message bubble
// ---------------------------------------------------------------------------

/// Replace any existing bubble above the player with `message`. The bubble is
/// a container child of the player holding a faint dark panel and the text, so
/// both fade together when the timer runs out.
fn spawn_bubble(
    commands: &mut Commands,
    chat: &mut ChatState,
    player: Entity,
    message: &str,
) {
    // A new message replaces the previous one.
    if let Some(prev) = chat.bubble.take() {
        if let Ok(mut prev_entity) = commands.get_entity(prev) {
            prev_entity.despawn();
        }
    }

    // Fira Mono advances ~4.8px per glyph at size 8; add breathing room.
    let glyphs = message.chars().count() as f32;
    let panel_w = (glyphs * 4.8 + 10.0).max(24.0);

    let mut container: Option<Entity> = None;
    commands.entity(player).with_children(|parent| {
        let bubble_entity = parent
            .spawn((
                Transform::from_xyz(0.0, BUBBLE_CONTAINER_Y, 2.0),
                ChatBubble {
                    timer: Timer::from_seconds(BUBBLE_SECONDS, TimerMode::Once),
                },
                Name::new("ChatBubble"),
            ))
            .id();
        container = Some(bubble_entity);

        // Subtle dark panel so the text reads as a chat bubble above the
        // world, even over bright tiles.
        parent.commands().entity(bubble_entity).with_children(|bubble| {
            bubble.spawn((
                Sprite::from_color(
                    Color::srgba(0.04, 0.05, 0.08, 0.72),
                    Vec2::new(panel_w, BUBBLE_PANEL_H),
                ),
                Transform::from_xyz(0.0, 0.0, 0.0),
                Name::new("ChatBubblePanel"),
            ));
            bubble.spawn((
                Text2d::new(message),
                TextFont {
                    font_size: FontSize::Px(8.0),
                    ..default()
                },
                TextColor(Color::srgb(0.95, 0.95, 0.92)),
                Text2dShadow {
                    offset: Vec2::new(0.0, -1.0),
                    color: Color::srgba(0.0, 0.0, 0.0, 0.85),
                },
                TextLayout::justify(Justify::Center),
                Transform::from_xyz(0.0, 0.0, 0.1),
                Name::new("ChatBubbleText"),
            ));
        });
    });

    chat.bubble = container;
}

/// Tick the bubble timers and remove the bubble when its time is up.
pub fn bubble_expiry(
    mut commands: Commands,
    mut chat: ResMut<ChatState>,
    mut bubbles: Query<(Entity, &mut ChatBubble)>,
    time: Res<Time>,
) {
    for (entity, mut bubble) in &mut bubbles {
        bubble.timer.tick(time.delta());
        if bubble.timer.just_finished() {
            if chat.bubble == Some(entity) {
                chat.bubble = None;
            }
            commands.entity(entity).despawn();
        }
    }
}