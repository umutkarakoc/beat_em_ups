use std::time::Duration;

use bevy::{core_pipeline::core_2d::graph::input, input::*, prelude::*};
use keyboard::KeyboardInput;

use crate::GameState;

pub struct PlayerInput;
impl Plugin for PlayerInput {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            // .add_systems(Startup, init )
            .add_systems(
                PreUpdate,
                (keyboard_action, keyboard_analog, mouse_action)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(PostUpdate, clear);
    }
}

#[derive(Component)]
pub struct Input;

#[derive(Component, PartialEq, Eq, Clone)]
pub struct KeyboardAction(pub KeyCode);

#[derive(Component, PartialEq, Eq, Clone)]
pub struct MouseAction(pub MouseButton);

#[derive(Component, Debug)]
pub struct Analog(pub f32, pub f32);

#[derive(Component)]
pub struct KeyboardAnalog(pub KeyCode, pub KeyCode, pub KeyCode, pub KeyCode);

#[derive(Component)]
pub struct Active;
#[derive(Component)]
pub struct Just;
#[derive(Component)]
pub struct Released;

#[derive(Component)]
pub struct Movement;

#[derive(Component)]
pub struct Run;

#[derive(Component)]
pub struct Attack;

#[derive(Component)]
pub struct Dodge;

fn keyboard_action(
    mut commands: Commands,
    inputs: Query<(Entity, &KeyboardAction)>,
    mut kbd: EventReader<KeyboardInput>,
) {
    for ev in kbd.read() {
        for (entity, &KeyboardAction(key)) in &inputs {
            if ev.key_code != key {
                continue;
            }
            if ButtonState::Pressed == ev.state {
                commands.entity(entity).insert(Active).insert(Just);
            } else {
                commands.entity(entity).insert(Released).remove::<Active>();
            }
        }
    }
}
fn keyboard_analog(
    mut commands: Commands,
    inputs: Query<(Entity, &Analog, &KeyboardAnalog)>,
    mut kbd: EventReader<KeyboardInput>,
) {
    for (e, &Analog(current_x, current_y), &KeyboardAnalog(up, down, right, left)) in &inputs {
        for ev in kbd.read() {
            let y = if up == ev.key_code {
                if ev.state == ButtonState::Pressed {
                    1.0
                } else if ev.state == ButtonState::Released {
                    0.0
                } else {
                    current_y
                }
            } else if down == ev.key_code {
                if ev.state == ButtonState::Pressed {
                    -1.0
                } else if ev.state == ButtonState::Released {
                    0.0
                } else {
                    current_y
                }
            } else {
                current_y
            };

            let x = if right == ev.key_code {
                if ev.state == ButtonState::Pressed {
                    1.0
                } else if ev.state == ButtonState::Released {
                    0.0
                } else {
                    current_x
                }
            } else if left == ev.key_code {
                if ev.state == ButtonState::Pressed {
                    -1.0
                } else if ev.state == ButtonState::Released {
                    0.0
                } else {
                    current_x
                }
            } else {
                current_x
            };

            if x == 0. && y == 0. {
                commands.entity(e).remove::<Active>().insert(Released);
            } else {
                commands.entity(e).insert(Active).insert(Just);
            }

            commands.entity(e).insert(Analog(x, y));
        }
    }
}

fn mouse_action(
    mut commands: Commands,
    inputs: Query<(Entity, &MouseAction)>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    for (e, &MouseAction(mb)) in &inputs {
        if mouse.just_pressed(mb) {
            commands.entity(e).insert(Active).insert(Just);
        } else if mouse.just_released(mb) {
            commands.entity(e).insert(Released).remove::<Active>();
        }
    }
}

fn clear(mut commands: Commands, mut inputs: Query<Entity, With<Input>>) {
    for e in &mut inputs {
        commands.entity(e).remove::<Just>().remove::<Released>();
        // a.tick(time.delta());

        // if a.0.finished() {
        //     println!("remove");
        //     commands.entity(e).remove::<Active>();
        // }
    }
}
