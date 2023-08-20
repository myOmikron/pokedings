use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::AssetServer;
use bevy::input::Input;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::time::Time;
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, CollidingEntities, GravityScale, KinematicCharacterController,
    RigidBody, Sensor,
};
use bevy_rapier2d::rapier::prelude::ColliderType;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Eq, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub last_direction: Direction,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup)
            .add_systems(Update, move_and_animate);
    }
}

pub fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<TextureAtlas>>,
) {
    // Setup player
    commands.spawn((
        Player {
            speed: 200.0,
            last_direction: Direction::Down,
        },
        SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(
                asset_server.load("graphics/characters/trainer.png"),
                Vec2::new(32.0, 48.0),
                4,
                4,
                None,
                None,
            )),
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Name::new("Player"),
    ));
}

pub fn move_and_animate(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player: Query<&mut Player>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut animation: Query<(&mut AnimationTimer, &mut TextureAtlasSprite)>,
    collision: Query<&CollidingEntities>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let direction = if keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up)
        {
            Some(Direction::Up)
        } else if keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down) {
            Some(Direction::Down)
        } else if keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left) {
            Some(Direction::Left)
        } else if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            Some(Direction::Right)
        } else {
            None
        };

        for (mut timer, mut sprite) in &mut animation {
            timer.tick(time.delta());
            if timer.just_finished() {
                if let Ok(player) = player.get_single() {
                    sprite.index = match &direction {
                        None => match player.last_direction {
                            Direction::Up => 12,
                            Direction::Down => 0,
                            Direction::Left => 4,
                            Direction::Right => 8,
                        },
                        Some(x) => {
                            if player.last_direction != *x || sprite.index % 4 == 3 {
                                match x {
                                    Direction::Up => 12,
                                    Direction::Down => 0,
                                    Direction::Left => 4,
                                    Direction::Right => 8,
                                }
                            } else {
                                sprite.index + 1
                            }
                        }
                    }
                }
            }
        }

        if let Ok(mut player) = player.get_single_mut() {
            let mut direction = match &direction {
                None => Vec3::ZERO,
                Some(x) => match x {
                    Direction::Up => {
                        player.last_direction = Direction::Up;
                        Vec3::new(0.0, 1.0, 0.0)
                    }
                    Direction::Down => {
                        player.last_direction = Direction::Down;
                        Vec3::new(0.0, -1.0, 0.0)
                    }
                    Direction::Left => {
                        player.last_direction = Direction::Left;
                        Vec3::new(-1.0, 0.0, 0.0)
                    }
                    Direction::Right => {
                        player.last_direction = Direction::Right;
                        Vec3::new(1.0, 0.0, 0.0)
                    }
                },
            };

            if direction.length() > 0.0 {
                direction = direction.normalize();
            }

            transform.translation += direction * player.speed * time.delta_seconds();
        }
    }
}
