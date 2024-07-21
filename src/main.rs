// disable console on windows for release builds
#![allow(clippy::type_complexity)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::render::texture::{DefaultImageSampler, ImageLoaderSettings, ImageSampler};
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use std::io::Cursor;
use winit::window::Icon;
mod assets;
mod menu;
mod player;
mod sprite_sheet;
use crate::assets::AssetsPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
use crate::sprite_sheet::SpriteSheetPlugin;
use assets::TextureAssets;
use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::text::TextSettings;
use bevy::{app::App, render::camera::ScalingMode};

fn main() {
    let mut app = App::new();
    app.insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(TextSettings {
            allow_dynamic_font_size: true,
            ..default()
        })
        .init_state::<GameState>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy game".to_string(), // ToDo
                        resolution: WindowResolution::new(480., 270.),
                        // Bind to canvas included in `index.html`
                        canvas: Some("#bevy".to_owned()),
                        fit_canvas_to_parent: true,
                        // Tells wasm not to override default event handling, like F5 and Ctrl+R
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, (add_camera, set_window_icon))
        .add_plugins((SpriteSheetPlugin, AssetsPlugin, MenuPlugin, PlayerPlugin))
        .add_systems(OnEnter(GameState::Playing), add_bg);

    #[cfg(debug_assertions)]
    {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
            WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Escape)),
        ));
    }

    app.run();
}

// Sets the icon on windows and X11
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let Some(primary) = windows.get_window(primary_entity) else {
        return;
    };
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

fn add_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 100.),
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(480.),
            ..default()
        },
        ..default()
    });
}

fn add_bg(mut commands: Commands, assets: Res<TextureAssets>) {
    commands.spawn(
        (SpriteBundle {
            texture: assets.bg.clone(),
            ..default()
        }),
    );
}
