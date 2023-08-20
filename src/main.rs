use std::time::Duration;

use bevy::asset::ChangeWatcher;
use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::plugin::RapierPhysicsPlugin;
use bevy_rapier2d::prelude::NoUserData;
use bevy_rapier2d::render::RapierDebugRenderPlugin;

use crate::camera::CameraPlugin;
use crate::debug::DebugPlugin;
use crate::helper::tiled::{TiledMap, TiledMapBundle};
use crate::player::PlayerPlugin;

mod camera;
mod debug;
mod helper;
mod player;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    MenuLoading,
    InMenu,
    GameLoading,
    InGame,
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Setup map
    let map_handle: Handle<TiledMap> = asset_server.load("maps/test.tmx");
    commands.spawn(TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Pokédings"),
                    resizable: false,
                    present_mode: PresentMode::AutoVsync,
                    resolution: WindowResolution::new(1280.0, 720.0),
                    ..Default::default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                ..default()
            }),))
        .add_state::<GameState>()
        .add_plugins(DebugPlugin)
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(24.0),
            RapierDebugRenderPlugin::default(),
        ))
        .add_plugins((TilemapPlugin, helper::tiled::TiledMapPlugin))
        .add_plugins((PlayerPlugin, CameraPlugin))
        .add_systems(Startup, startup)
        .run();
}
