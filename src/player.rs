use crate::assets::SamuraiAssets;
use crate::sprite_sheet::{
    Animation, AnimationEnded, AnimationTimer, Direction, NoRepeat, SpriteAnimation,
};
use crate::GameState;
use bevy::ecs::world::Command;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use rand::Rng;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct CharacterAnimations {
    pub idle: SpriteAnimation,
    pub run: SpriteAnimation,
    pub attack1: SpriteAnimation,
    pub attack2: SpriteAnimation,
    pub attack3: SpriteAnimation,
    pub defence: SpriteAnimation,
    pub dash: SpriteAnimation,
}

#[derive(Component, Deref)]
pub struct RunSpeed(pub f32);

#[derive(Component, Deref)]
pub struct AttackMoveSpeed(pub f32);

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Samurai;

#[derive(Component)]
pub struct Knight;

#[derive(Component, Debug)]
pub struct Idle;

#[derive(Component, Debug)]
pub struct Walk;

#[derive(Component, Debug)]
pub struct Run;

#[derive(Component, Debug)]
pub struct Attack(pub u32);

#[derive(Component, Debug)]
pub struct Defence;

#[derive(Component, Debug)]
pub struct Dash;

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
    dash: PlayerInput,
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
                (
                    init_samurai,
                    idle_anim,
                    attack_anim,
                    action_anim_ended,
                    run_anim,
                    dash_anim,
                    defence_anim,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    set_direction,
                    idle_to_run,
                    run_to_idle,
                    run,
                    attack,
                    combo_attack,
                    dash,
                    defence,
                    attack_move,
                )
                    .run_if(run_if_player_alive)
                    .after(player_input)
                    .before(player_input_clear),
            )
            .add_systems(PostUpdate, post_print);
    }
}

fn init(mut commands: Commands) {
    commands.spawn((Name::new("Player"), Player, Samurai));
}

fn post_print() {
    // println!("----------------------");
}

fn init_samurai(
    mut commands: Commands,
    players: Query<Entity, Added<Samurai>>,
    assets: Res<SamuraiAssets>,
) {
    for id in &players {
        commands.entity(id).insert((
            RunSpeed(200.),
            AttackMoveSpeed(200.),
            Idle,
            Alive,
            Direction::Right,
            CharacterAnimations {
                idle: SpriteAnimation {
                    texture: assets.idle.clone(),
                    layout: assets.idle_layout.clone(),
                    animation: Animation::new(1000, 0, 3),
                },
                run: SpriteAnimation {
                    texture: assets.run.clone(),
                    layout: assets.run_layout.clone(),
                    animation: Animation::new(600, 0, 7),
                },
                attack1: SpriteAnimation {
                    texture: assets.attack1.clone(),
                    layout: assets.attack1_layout.clone(),
                    animation: Animation::new(200, 0, 3),
                },
                attack2: SpriteAnimation {
                    texture: assets.attack2.clone(),
                    layout: assets.attack2_layout.clone(),
                    animation: Animation::new(200, 0, 4),
                },
                attack3: SpriteAnimation {
                    texture: assets.attack3.clone(),
                    layout: assets.attack3_layout.clone(),
                    animation: Animation::new(200, 0, 3),
                },
                defence: SpriteAnimation {
                    texture: assets.defence.clone(),
                    layout: assets.defence_layout.clone(),
                    animation: Animation::new(100, 0, 1),
                },
                dash: SpriteAnimation {
                    texture: assets.dash.clone(),
                    layout: assets.dash_layout.clone(),
                    animation: Animation::new(400, 0, 5),
                },
            },
            LocalController {
                left: PlayerInput::Key(KeyCode::KeyA),
                right: PlayerInput::Key(KeyCode::KeyD),
                up: PlayerInput::Key(KeyCode::KeyW),
                down: PlayerInput::Key(KeyCode::KeyS),
                dash: PlayerInput::Key(KeyCode::Space),
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
    Without<Dash>,
    Without<Rest>,
    Without<Defence>,
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
            println!("run");
            entity.remove::<Idle>().remove::<Dead>().insert(Run);
        }
    }
}

fn run_to_idle(
    mut commands: Commands,
    players: Query<(Entity, &LocalController), (CanActQuery, With<Run>)>,
    keys: Res<ButtonInput<PlayerInput>>,
) {
    for (entity, ctl) in &players {
        let mut entity = commands.entity(entity);
        if !keys.any_pressed([ctl.left, ctl.right]) {
            println!("idle");
            entity.remove::<Run>().insert(Idle);
        }
    }
}

fn attack(
    mut commands: Commands,
    players: Query<(Entity, &LocalController), CanActQuery>,
    keys: Res<ButtonInput<PlayerInput>>,
) {
    for (entity, ctl) in &players {
        let mut entity = commands.entity(entity);
        if keys.just_pressed(ctl.attack) {
            println!("attack");
            entity.remove::<Idle>().remove::<Run>().insert(Attack(1));
        }
    }
}

fn combo_attack(
    mut commands: Commands,
    players: Query<(Entity, &LocalController, &Attack, &AnimationTimer), With<Alive>>,
    keys: Res<ButtonInput<PlayerInput>>,
) {
    return;
    for (entity, ctl, attack, timer) in &players {
        if timer.remaining_secs() > 0.1 {
            continue;
        }
        let mut entity = commands.entity(entity);
        println!("combo");
        if keys.just_pressed(ctl.attack) {
            entity
                .remove::<Idle>()
                .remove::<Run>()
                .insert(Attack(attack.0 + 1));
        }
    }
}

fn defence(
    mut commands: Commands,
    players: Query<(Entity, &LocalController), CanActQuery>,
    keys: Res<ButtonInput<PlayerInput>>,
) {
    for (entity, ctl) in &players {
        let mut entity = commands.entity(entity);
        if keys.just_pressed(ctl.defense) {
            entity.remove::<Idle>().remove::<Run>().insert(Defence);
        }
    }
}
fn dash(
    mut commands: Commands,
    players: Query<(Entity, &LocalController), CanActQuery>,
    keys: Res<ButtonInput<PlayerInput>>,
) {
    for (entity, ctl) in &players {
        let mut entity = commands.entity(entity);
        if keys.just_pressed(ctl.dash) {
            entity.remove::<Idle>().remove::<Run>().insert(Dash);
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

fn attack_move(
    time: Res<Time>,
    mut players: Query<(&mut Transform, &AttackMoveSpeed, &Direction), With<Attack>>,
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

fn attack_anim(
    mut commands: Commands,
    players: Query<(Entity, &CharacterAnimations), Added<Attack>>,
) {
    for (e, anims) in &players {
        let i = rand::thread_rng().gen_range(1..=3);
        let anim = match i {
            1 => &anims.attack1,
            2 => &anims.attack2,
            _ => &anims.attack3,
        };
        commands.entity(e).insert((
            anim.texture.clone(),
            anim.animation.clone(),
            NoRepeat,
            TextureAtlas::from(anim.layout.clone()),
        ));
    }
}
fn dash_anim(mut commands: Commands, players: Query<(Entity, &CharacterAnimations), Added<Dash>>) {
    for (e, anims) in &players {
        commands.entity(e).insert((
            anims.dash.texture.clone(),
            anims.dash.animation.clone(),
            NoRepeat,
            TextureAtlas::from(anims.dash.layout.clone()),
        ));
    }
}
fn defence_anim(
    mut commands: Commands,
    players: Query<(Entity, &CharacterAnimations), Added<Defence>>,
) {
    for (e, anims) in &players {
        commands.entity(e).insert((
            anims.defence.texture.clone(),
            anims.defence.animation.clone(),
            NoRepeat,
            TextureAtlas::from(anims.defence.layout.clone()),
        ));
    }
}
fn action_anim_ended(
    mut commands: Commands,
    mut ended: EventReader<AnimationEnded>,
    players: Query<Entity>,
) {
    for &AnimationEnded(entity) in ended.read() {
        if let Ok(player) = players.get(entity) {
            println!("ended");
            commands
                .entity(player)
                .remove::<Attack>()
                .remove::<Defence>()
                .remove::<Dash>()
                .remove::<NoRepeat>()
                .insert(Idle);
        };
    }
}
