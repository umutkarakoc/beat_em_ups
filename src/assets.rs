use crate::{
    sprite_sheet::{Animation, SpriteAnimation},
    GameState,
};
use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct AssetsPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>()
                .load_collection::<SamuraiAssets>()
                .load_collection::<KnightAssets>(),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    // #[asset(path = "audio.ogg")]
    // pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "bevy.png")]
    pub bevy: Handle<Image>,

    #[asset(path = "bg.png")]
    pub bg: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct KnightAssets {
    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 128, columns = 4, rows = 1))]
    pub idle_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "knight/idle.png")]
    pub idle: Handle<Image>,

    #[asset(path = "knight/walk.png")]
    pub walk: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 128, columns = 8, rows = 1))]
    pub run_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "knight/run.png")]
    pub run: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct SamuraiAssets {
    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 128, columns = 4, rows = 1))]
    pub idle_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "samurai/idle.png")]
    pub idle: Handle<Image>,

    #[asset(path = "samurai/walk.png")]
    pub walk: Handle<Image>,

    #[asset(texture_atlas_layout(tile_size_x = 128, tile_size_y = 128, columns = 8, rows = 1))]
    pub run_layout: Handle<TextureAtlasLayout>,

    #[asset(path = "samurai/run.png")]
    pub run: Handle<Image>,
}
