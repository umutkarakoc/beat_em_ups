use std::{ops::Deref, time::Duration};

use bevy::{prelude::*, utils::tracing::Instrument};

#[derive(Component, Deref, DerefMut, Clone)]
pub struct AnimationTimer(pub Timer);

impl AnimationTimer {
    pub fn new(ms: u64) -> AnimationTimer {
        AnimationTimer(Timer::new(Duration::from_millis(ms), TimerMode::Repeating))
    }
}

#[derive(Component, Clone)]
pub struct AnimationIndex {
    pub start: usize,
    pub end: usize,
}

#[derive(Component)]
pub struct NoRepeat;

#[derive(Component, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

impl Direction {
    pub fn x(&self) -> f32 {
        match self {
            Direction::Left => -1.0,
            Direction::Right => 1.0,
        }
    }
    pub fn is_flip_x(&self) -> bool {
        match self {
            Direction::Left => true,
            Direction::Right => false,
        }
    }
}

#[derive(Event)]
pub struct AnimationEnded(pub Entity);

impl AnimationIndex {
    pub fn new(start: usize, end: usize) -> AnimationIndex {
        AnimationIndex { start, end }
    }
}

#[derive(Bundle, Clone)]
pub struct Animation {
    pub timer: AnimationTimer,
    pub index: AnimationIndex,
}
impl Animation {
    pub fn new(duration: u64, start: usize, end: usize) -> Animation {
        Animation {
            timer: AnimationTimer::new(duration / (end - start) as u64),
            index: AnimationIndex::new(start, end),
        }
    }
}

#[derive(Component)]
pub struct SpriteAnimation {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub animation: Animation,
}

impl SpriteAnimation {
    pub fn new(
        texture: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
        animation: Animation,
    ) -> Self {
        SpriteAnimation {
            texture,
            layout,
            animation,
        }
    }
}

pub struct SpriteSheetPlugin;

impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (animate, flip_x))
            .add_event::<AnimationEnded>();
    }
}

fn animate(
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &AnimationIndex,
        &mut AnimationTimer,
        &mut TextureAtlas,
        Option<&NoRepeat>,
    )>,
    mut ended: EventWriter<AnimationEnded>,
) {
    for (entity, indices, mut timer, mut atlas, norepeat) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let mut next = atlas.index + 1;

            if next > indices.end && norepeat.is_some() {
                ended.send(AnimationEnded(entity));
                return;
            }
            if next > indices.end {
                next = indices.start;
            }
            atlas.index = next;
        }
    }
}

fn flip_x(mut sprites: Query<(&mut Sprite, &Direction), Changed<Direction>>) {
    for (mut sprite, dir) in &mut sprites {
        sprite.flip_x = dir.is_flip_x()
    }
}
