use bevy::{prelude::*, render::camera::ScalingMode};

use crate::{assets::TextureAssets, GameState};

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_camera)
            .add_systems(OnEnter(GameState::Playing), add_bg);
    }
}

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 100.),
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(270.),

            ..default()
        },
        ..default()
    });
}

fn add_bg(mut commands: Commands, assets: Res<TextureAssets>) {
    println!("test");
    commands.spawn(SpriteBundle {
        texture: assets.bg.clone(),
        ..default()
    });
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(-480., 0., 0.),
        texture: assets.bg.clone(),
        ..default()
    });
    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(480., 0., 0.),
        texture: assets.bg.clone(),
        ..default()
    });
}
