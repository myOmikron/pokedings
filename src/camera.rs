use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup).add_systems(
            Update,
            attach_camera_to_player.after(crate::player::move_and_animate),
        );
    }
}

pub fn startup(mut commands: Commands) {
    // Setup camera
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 1024.0,
        min_height: 576.0,
    };
    commands.spawn((camera, MainCamera));
}

#[derive(Component)]
struct MainCamera;

#[allow(clippy::type_complexity)]
fn attach_camera_to_player(
    mut query: ParamSet<(
        Query<&Transform, With<Player>>,
        Query<&mut Transform, With<MainCamera>>,
    )>,
) {
    let player_pos = {
        match query.p0().get_single() {
            Ok(p) => p.translation,
            Err(e) => {
                info!("Querying player errored with {:?}", e);
                return;
            }
        }
    };

    match query.p1().get_single_mut() {
        Ok(mut c) => {
            c.translation.x = player_pos.x.round();
            c.translation.y = player_pos.y.round();
        }
        Err(e) => {
            info!("Querying camera errored with {:?}", e);
        }
    }
}
