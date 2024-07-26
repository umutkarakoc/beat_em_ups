use crate::assets::KnightAssets;
use crate::sprite_sheet::Animation;
use crate::GameState;
use bevy::prelude::*;

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

fn spawn_player(mut commands: Commands, knight: Res<KnightAssets>, samurai: Res<KnightAssets>) {
    commands.spawn((
        Player,
        Samurai,
        Animation::new(1000, 0, 3),
        TextureAtlas::from(samurai.idle_layout.clone()),
        SpriteBundle {
            texture: samurai.idle.clone(),
            transform: Transform::from_translation(Vec3::new(200., 0., 1.)),
            ..Default::default()
        },
    ));

    commands.spawn((
        Enemy,
        Knight,
        Animation::new(1000, 0, 3),
        TextureAtlas::from(knight.idle_layout.clone()),
        SpriteBundle {
            texture: knight.idle.clone(),
            transform: Transform::from_translation(Vec3::new(-200., 0., 1.)),
            ..Default::default()
        },
    ));
}
