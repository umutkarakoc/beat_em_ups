use crate::assets::SamuraiAssets;
use crate::sprite_sheet::{Animation, Direction, SpriteAnimation};
use crate::GameState;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct CharacterAnimations {
    pub idle: SpriteAnimation,
    pub run: SpriteAnimation,
}

#[derive(Component, Deref)]
pub struct RunSpeed(pub f32);

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
pub struct Defend;

#[derive(Component)]
pub struct Dodge;

#[derive(Component)]
pub struct Jump;

#[derive(Component)]
pub struct Alive;

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct Rest;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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
        app.insert_resource(ButtonInput::<PlayerInput>::default())
            .add_systems(OnEnter(GameState::Playing), init)
            .add_systems(Update, player_input)
            .add_systems(Update, player_input_clear)
            .add_systems(
                Update,
                (init_samurai, idle_anim, run_anim).run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (set_direction, idle_to_run, to_idle, run)
                    .run_if(run_if_player_alive)
                    .after(player_input)
                    .before(player_input_clear),
            );
    }
}

fn init(mut commands: Commands) {
    commands.spawn((Name::new("Player"), Player, Samurai));
}

fn init_samurai(
    mut commands: Commands,
    players: Query<Entity, Added<Samurai>>,
    assets: Res<SamuraiAssets>,
) {
    for id in &players {
        commands.entity(id).insert((
            RunSpeed(200.),
            Idle,
            Alive,
            CharacterAnimations {
                idle: SpriteAnimation {
                    texture: assets.idle.clone(),
                    layout: assets.idle_layout.clone(),
                    animation: Animation::new(1000, 0, 3),
                },
                run: SpriteAnimation {
                    texture: assets.run.clone(),
                    layout: assets.run_layout.clone(),
                    animation: Animation::new(800, 0, 3),
                },
            },
            LocalController {
                left: PlayerInput::Key(KeyCode::KeyA),
                right: PlayerInput::Key(KeyCode::KeyD),
                up: PlayerInput::Key(KeyCode::KeyW),
                down: PlayerInput::Key(KeyCode::KeyS),
                dodge: PlayerInput::Key(KeyCode::Space),
                attack: PlayerInput::Mouse(MouseButton::Left),
                defense: PlayerInput::Mouse(MouseButton::Right),
            },
            SpriteBundle {
                transform: Transform::from_xyz(-200., 0., 1.),
                ..default()
            },
        ));
    }
}

fn run_if_player_alive(q_player: Query<&Player, (With<Alive>, Without<Dead>)>) -> bool {
    if let Ok(_) = q_player.get_single() {
        return true;
    }
    false
}

fn player_input(
    mut input: ResMut<ButtonInput<PlayerInput>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut kbd: EventReader<KeyboardInput>,
) {
    for ev in kbd.read() {
        match ev.state {
            ButtonState::Pressed => input.press(PlayerInput::Key(ev.key_code)),
            ButtonState::Released => input.release(PlayerInput::Key(ev.key_code)),
        }
    }

    if mouse.just_pressed(MouseButton::Left) {
        input.press(PlayerInput::Mouse(MouseButton::Left));
    }
    if mouse.just_pressed(MouseButton::Left) {
        input.release(PlayerInput::Mouse(MouseButton::Left));
    }
    if mouse.just_pressed(MouseButton::Right) {
        input.press(PlayerInput::Mouse(MouseButton::Right));
    }
    if mouse.just_pressed(MouseButton::Right) {
        input.release(PlayerInput::Mouse(MouseButton::Right));
    }
}
fn player_input_clear(mut input: ResMut<ButtonInput<PlayerInput>>) {
    input.clear()
}

type CanActQuery = (
    With<Alive>,
    Without<Dead>,
    Without<Attack>,
    Without<Dodge>,
    Without<Rest>,
    Without<Defend>,
);

fn set_direction(
    mut commands: Commands,
    players: Query<(Entity, &LocalController), CanActQuery>,
    keys: Res<ButtonInput<PlayerInput>>,
) {
    for (entity, ctl) in &players {
        let mut entity = commands.entity(entity);
        if keys.just_pressed(ctl.left) {
            entity.insert(Direction::Left);
        }
        if keys.just_pressed(ctl.right) {
            entity.insert(Direction::Right);
        }
    }
}

fn idle_to_run(
    mut commands: Commands,
    players: Query<(Entity, &LocalController), (CanActQuery, With<Idle>)>,
    keys: Res<ButtonInput<PlayerInput>>,
) {
    for (entity, ctl) in &players {
        let mut entity = commands.entity(entity);
        if keys.any_pressed([ctl.left, ctl.right]) {
            entity.remove::<Idle>().insert(Run);
        }
    }
}

fn to_idle(
    mut commands: Commands,
    players: Query<(Entity, &LocalController), (CanActQuery, With<Run>)>,
    keys: Res<ButtonInput<PlayerInput>>,
) {
    for (entity, ctl) in &players {
        let mut entity = commands.entity(entity);
        if !keys.any_pressed([ctl.left, ctl.right]) {
            entity.remove::<Run>().insert(Idle);
        }
    }
}

fn run(
    time: Res<Time>,
    mut players: Query<(&mut Transform, &RunSpeed, &Direction), (CanActQuery, With<Run>)>,
) {
    for (mut t, speed, dir) in &mut players {
        t.translation.x += **speed * dir.x() * time.delta_seconds();
    }
}

fn idle_anim(mut commands: Commands, players: Query<(Entity, &CharacterAnimations), Added<Idle>>) {
    for (e, anims) in &players {
        commands.entity(e).insert((
            anims.idle.texture.clone(),
            anims.idle.animation.clone(),
            TextureAtlas::from(anims.idle.layout.clone()),
        ));
    }
}

fn run_anim(mut commands: Commands, players: Query<(Entity, &CharacterAnimations), Added<Run>>) {
    for (e, anims) in &players {
        commands.entity(e).insert((
            anims.run.texture.clone(),
            anims.run.animation.clone(),
            TextureAtlas::from(anims.run.layout.clone()),
        ));
    }
}
