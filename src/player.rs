use crate::assets::SamuraiAssets;
use crate::input::Active;
use crate::sprite_sheet::{
    self, Animation, AnimationEnded, AnimationTimer, NoRepeat, SpriteAnimation,
};
use crate::{input, GameState};
use bevy::ecs::world::Command;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::Rng;

pub struct PlayerPlugin;

trait Controller: Component + Default {}

#[derive(Default, Component)]
pub struct Controller1;

impl Controller for Controller1 {}

#[cfg_attr(Default, Controller, derive(Controller))]
pub struct Controller2;

#[derive(Component)]
pub struct Character;

#[derive(Component)]
pub struct Samurai;

#[derive(Component)]
pub struct Alive;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct ReadInputSet;
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct ActionSet;

#[derive(Component)]
pub struct LevelLimit;

#[derive(Component)]
pub struct MoveSpeed {
    walk: f32,
    run: f32,
}

#[derive(Component)]
pub struct Speed(f32);

#[derive(Component)]
pub struct Direction(f32, f32);

#[derive(Component, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Movement {
    Idle,
    Walk,
    Run,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), init)
            .add_systems(
                Update,
                (set_direction::<Controller1>, set_movement::<Controller1>)
                    .in_set(ReadInputSet)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (limit, movement)
                    .in_set(ActionSet)
                    .after(ReadInputSet)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                PostUpdate,
                (flip_x, init_samurai, init_shadow, movement_animation)
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

fn init(mut commands: Commands) {
    commands.spawn((Name::new("Player"), Character, Samurai, Controller1));
}

fn init_samurai(mut commands: Commands, players: Query<Entity, Added<Samurai>>) {
    for id in &players {
        commands.entity(id).insert((
            MoveSpeed {
                walk: 80.,
                run: 200.,
            },
            Movement::Idle,
            Alive,
            Direction(1., 0.),
            SpriteBundle {
                transform: Transform::from_xyz(-200., 0., 10.),
                ..default()
            },
        ));

        commands.spawn((
            Controller1,
            input::Analog(0., 0.),
            input::Movement,
            input::KeyboardAnalog(KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyD, KeyCode::KeyA),
        ));
        commands.spawn((
            Controller1,
            input::Run,
            input::KeyboardAction(KeyCode::ShiftLeft),
        ));
        commands.spawn((
            Controller1,
            input::Attack,
            input::MouseAction(MouseButton::Left),
        ));
    }
}

fn init_shadow(
    mut commands: Commands,
    players: Query<Entity, Added<Character>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for p in &players {
        commands.entity(p).with_children(|p| {
            let shadow = Mesh2dHandle(meshes.add(Ellipse::new(12., 6.)));
            let color = materials.add(Color::linear_rgba(0., 0., 0., 0.5));
            // let color = Color::hsl(360. * 1 as f32 / 2 as f32, 0.95, 0.7);

            p.spawn((
                Name::new("Shadow"),
                MaterialMesh2dBundle {
                    mesh: shadow,
                    material: color,
                    transform: Transform::from_xyz(0., -64., 0.),
                    ..default()
                },
            ));
        });
    }
}

fn set_direction<C: Controller>(
    mut commands: Commands,
    player: Query<Entity, (With<C>, With<Character>)>,
    movement: Query<&input::Analog, (With<input::Active>, With<C>, With<input::Movement>)>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };

    let Ok(&input::Analog(x, y)) = movement.get_single() else {
        return;
    };

    if x == 0. && y == 0. {
        commands.entity(player).insert(Direction(x, y));
    } else {
        commands.entity(player).insert(Direction(x, y));
    }
}

fn set_movement<C: Controller>(
    mut commands: Commands,
    player: Query<(Entity, &Movement), With<C>>,
    input: Query<Entity, (With<input::Active>, With<C>, With<input::Movement>)>,
    is_run: Query<(&input::Run, &Active)>,
) {
    let Ok((entity, &movement)) = player.get_single() else {
        return;
    };

    if input.is_empty() {
        if movement != Movement::Idle {
            commands.entity(entity).insert(Movement::Idle);
        }
    } else if is_run.is_empty() {
        if movement != Movement::Walk {
            commands.entity(entity).insert(Movement::Walk);
        }
    } else if movement != Movement::Run {
        commands.entity(entity).insert(Movement::Run);
    }
}

fn movement(
    time: Res<Time>,
    mut players: Query<(&mut Transform, &Movement, &MoveSpeed, &Direction)>,
) {
    for (mut t, movement, speed, &Direction(x, y)) in &mut players {
        if movement == &Movement::Idle {
            return;
        }

        let speed = if movement == &Movement::Walk {
            speed.walk
        } else {
            speed.run
        };

        let way = Vec3::new(x, y, 0.).normalize();
        t.translation.x += speed * way.x * time.delta_seconds();
        t.translation.y += speed * way.y * time.delta_seconds() * 0.6;
    }
}

fn flip_x(mut sprites: Query<(&mut Sprite, &Direction), Changed<Direction>>) {
    for (mut sprite, &Direction(x, _y)) in &mut sprites {
        sprite.flip_x = x < 0.;
    }
}

fn limit(mut players: Query<&mut Transform, With<Character>>) {
    for mut t in &mut players {
        if t.translation.y > 35. {
            t.translation.y = 35.;
        }
        if t.translation.y < -40. {
            t.translation.y = -40.;
        }
        if t.translation.x < -200. {
            t.translation.x = -200.;
        }
        if t.translation.x > 200. {
            t.translation.x = 200.;
        }
    }
}

fn movement_animation(
    mut commands: Commands,
    players: Query<(Entity, &Movement), (With<Samurai>, Changed<Movement>)>,
    assets: Res<SamuraiAssets>,
) {
    for (e, &movement) in &players {
        match movement {
            Movement::Idle => {
                commands.entity(e).insert((
                    assets.idle.clone(),
                    TextureAtlas::from(assets.idle_layout.clone()),
                    Animation::new(1000, 0, 3),
                ));
            }
            Movement::Walk => {
                commands.entity(e).insert((
                    assets.walk.clone(),
                    TextureAtlas::from(assets.walk_layout.clone()),
                    Animation::new(1000, 0, 8),
                ));
            }
            Movement::Run => {
                commands.entity(e).insert((
                    assets.run.clone(),
                    TextureAtlas::from(assets.run_layout.clone()),
                    Animation::new(600, 0, 7),
                ));
            }
        }
    }
}
