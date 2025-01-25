use crate::config::Config;
use bevy::ecs::event::EventCursor;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_mod_raycast::prelude::*;

use crate::MyRaycastSet;

/// Keeps track of mouse motion events, pitch, and yaw
#[derive(Resource, Default)]
struct InputState {
    reader_motion: EventCursor<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

#[derive(Component)]
pub struct FlyCam;

#[derive(Component)]
pub struct Locked;

pub fn toggle_grab_cursor(mut commands: Commands, entity: Entity, window: &mut Window) {
    let mut entity = commands.entity(entity);
    (
        window.cursor_options.grab_mode,
        window.cursor_options.visible,
    ) = match window.cursor_options.grab_mode {
        CursorGrabMode::None => {
            entity.insert(Locked);
            (CursorGrabMode::Confined, false)
        }
        _ => {
            entity.remove::<Locked>();
            (CursorGrabMode::None, true)
        }
    }
}

fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 20.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
        FlyCam,
        RaycastSource::<MyRaycastSet>::new().with_early_exit(false),
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 1.0,
            rotation: Quat::IDENTITY,
        },
    ));
}

fn player_move(
    config: Res<Config>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    if let Ok(window) = primary_window.get_single() {
        for mut transform in query.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0., local_z.z);
            let right = Vec3::new(local_z.z, 0., -local_z.x);

            for &key in keys.get_pressed() {
                if window.cursor_options.grab_mode != CursorGrabMode::None {
                    if key == config.movement.forward {
                        velocity += forward;
                    } else if key == config.movement.backward {
                        velocity -= forward;
                    } else if key == config.movement.left {
                        velocity -= right;
                    } else if key == config.movement.right {
                        velocity += right;
                    } else if key == config.movement.up {
                        velocity += Vec3::Y;
                    } else if key == config.movement.down {
                        velocity -= Vec3::Y;
                    }
                }
            }

            velocity = velocity.normalize_or_zero();

            transform.translation += velocity * time.delta_secs() * config.movement.speed
        }
    } else {
        warn!("Primary window not found for `player_move`!");
    }
}

fn player_look(
    config: Res<Config>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<FlyCam>>,
) {
    if let Ok(window) = primary_window.get_single() {
        let delta_state = state.as_mut();
        for mut transform in query.iter_mut() {
            for ev in delta_state.reader_motion.read(&motion) {
                if window.cursor_options.grab_mode != CursorGrabMode::None {
                    // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                    let window_scale = window.height().min(window.width());
                    delta_state.pitch -=
                        (config.movement.sensitivity * ev.delta.y * window_scale).to_radians();
                    delta_state.yaw -=
                        (config.movement.sensitivity * ev.delta.x * window_scale).to_radians();
                }

                delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);

                // Order is important to prevent unintended roll
                transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw)
                    * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}

fn cursor_grab(
    config: Res<Config>,
    commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut primary_window: Query<(Entity, &mut Window), With<PrimaryWindow>>,
) {
    if let Ok((entity, mut window)) = primary_window.get_single_mut() {
        if keys.just_pressed(config.toggle_cursor_grab) {
            toggle_grab_cursor(commands, entity, &mut window);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
}

fn initial_cursor_grab(
    commands: Commands,
    mut primary_window: Query<(Entity, &mut Window), With<PrimaryWindow>>,
) {
    if let Ok((entity, mut window)) = primary_window.get_single_mut() {
        toggle_grab_cursor(commands, entity, &mut window);
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
}

fn spawn_lights(mut commands: Commands) {
    commands.spawn(DirectionalLight::default());
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1000.0,
    });
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .add_systems(Startup, (spawn_lights, setup_camera, initial_cursor_grab))
            .add_systems(Update, (player_move, player_look, cursor_grab));
    }
}
