use crate::assets::{KnightAssets, SamuraiAssets};
use crate::sprite_sheet::{Animation, Direction};
use crate::GameState;
use bevy::prelude::*;
use std::hash::Hash;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Samurai;

#[derive(Component)]
pub struct Knight;

#[derive(Component)]
pub struct Idle;

#[derive(Component)]
pub struct Walk;

#[derive(Component)]
pub struct Run;

#[derive(Component)]
pub struct Attack;

#[derive(Component)]
pub struct Jump;

pub enum PlayerInput {
    Key(KeyCode),
    Mouse(MouseButton),
}
#[derive(Component)]
pub struct LocalController {
    left: PlayerInput,
    right: PlayerInput,
    up: PlayerInput,
    down: PlayerInput,
    attack: PlayerInput,
    defense: PlayerInput,
    dodge: PlayerInput,
}

pub struct SpriteSheet {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player);
    }
}

fn spawn_player(mut commands: Commands, knight: Res<KnightAssets>, samurai: Res<SamuraiAssets>) {
    commands.spawn((
        Player,
        LocalController {
            left: PlayerInput::Key(KeyCode::KeyA),
            right: PlayerInput::Key(KeyCode::KeyD),
            up: PlayerInput::Key(KeyCode::KeyW),
            down: PlayerInput::Key(KeyCode::KeyS),
            attack: PlayerInput::Mouse(MouseButton::Left),
            defense: PlayerInput::Mouse(MouseButton::Right),
            dodge: PlayerInput::Key(KeyCode::Space),
        },
        Samurai,
        Direction::Right,
        Animation::new(1000, 0, 3),
        TextureAtlas::from(samurai.idle_layout.clone()),
        SpriteBundle {
            texture: samurai.idle.clone(),
            transform: Transform::from_translation(Vec3::new(-200., 0., 1.)),
            ..Default::default()
        },
    ));

    commands.spawn((
        Enemy,
        Knight,
        Direction::Left,
        Animation::new(1000, 0, 3),
        TextureAtlas::from(knight.idle_layout.clone()),
        SpriteBundle {
            texture: knight.idle.clone(),
            transform: Transform::from_translation(Vec3::new(200., 0., 1.)),
            ..Default::default()
        },
    ));
}
