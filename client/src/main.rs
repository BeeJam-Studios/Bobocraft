#![allow(clippy::too_many_arguments)]

use anyhow::Context;
use avian3d::prelude::*;
use bevy::color::palettes::css;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitSettings;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_mod_raycast::prelude::*;
use bevy_scene_hook::{HookPlugin, HookedSceneBundle, SceneHook};
use bevy_simple_text_input::{
    TextInput, TextInputInactive, TextInputPlugin, TextInputSettings, TextInputSubmitEvent,
    TextInputTextColor, TextInputTextFont, TextInputValue,
};
use bobocraft_graph::damage::{Graph as DamageGraph, GraphBuilder};
use config::Config;
use crossbeam_channel::{bounded, Receiver};
use std::env;
use std::f32::consts::PI;
use std::ops::Not;
use std::path::PathBuf;
pub type Fallible<T = ()> = Result<T, anyhow::Error>;

mod camera;
mod config;
#[cfg(test)]
mod tests;

use camera::{CameraPlugin, Locked};

pub fn not_any_with_component<T: Component>(query: Query<(), With<T>>) -> bool {
    query.is_empty()
}

#[derive(Debug, Component)]
pub struct Menu;

#[derive(Debug, Component)]
pub struct PenetrationText;

#[derive(Debug, Component)]
pub struct DamageText;

#[derive(Debug, Component)]
pub struct FocusedBobo;

#[derive(Debug, Component)]
pub struct FocusedCube;

#[derive(Debug, Component)]
pub struct ActiveBobo;

#[derive(Debug, Component)]
pub struct ModifiedBobo;

#[derive(Debug, Component)]
pub struct Hitmarker;

#[derive(Debug, Component)]
pub struct TextTopLeft;

#[derive(Debug, Component)]
pub struct TextTopRight;

#[derive(Debug, Component)]
pub struct TextBottomLeft;

#[derive(Debug, Component)]
pub struct TextBottomRight;

#[derive(Debug, Component, Deref)]
pub struct DamageTestChannel(Receiver<DamageTestProgress>);

#[derive(Debug, Component)]
pub struct Crosshair;

#[derive(Debug, Component)]
pub struct BoboName(String);

#[derive(Debug, Component)]
pub struct BoboDamageTestResult(String);

#[derive(Debug, Component, Deref)]
pub struct Placement(bobocraft_format::Placement);

#[derive(Debug, Component, Deref, DerefMut, Clone, Copy)]
pub struct CorrespondingCube(Entity);

#[derive(Debug, Component, Deref, DerefMut, Clone, Copy)]
pub struct CorrespondingBobo(Entity);

#[derive(Component, Deref, DerefMut)]
pub struct BoboGraph(DamageGraph);

#[derive(Debug, Clone)]
pub enum Item {
    Single(Entity),
    Multiple(Vec<Entity>),
}

use Item::*;

#[derive(Debug, Component, Default)]
pub struct Revert {
    queue: Vec<Item>,
    position: usize,
}

impl Revert {
    pub fn push_single(&mut self, value: Entity) {
        self.queue.truncate(self.position);
        self.queue.push(Single(value));
        self.position += 1;
    }

    pub fn push_multiple(&mut self, value: Vec<Entity>) {
        self.queue.truncate(self.position);
        self.queue.push(Multiple(value));
        self.position += 1;
    }

    pub fn prev(&mut self) -> Option<&Item> {
        self.position = self.position.saturating_sub(1);
        self.queue.get(self.position)
    }

    pub fn next_item(&mut self) -> Option<&Item> {
        let value = self.queue.get(self.position);
        self.position = self.queue.len().min(self.position + 1);
        value
    }
}
fn run_for_key_presses(mut reader: EventReader<KeyboardInput>, func: impl FnMut(&KeyCode)) {
    reader
        .read()
        .filter_map(
            |KeyboardInput {
                 key_code, state, ..
             }| {
                match state.is_pressed() {
                    true => Some(key_code),
                    false => None,
                }
            },
        )
        .for_each(func);
}
fn run_for_keybind(mut reader: EventReader<KeyboardInput>, bind: &KeyCode, mut func: impl FnMut()) {
    for ev in reader.read() {
        if ev.state.is_pressed() && &ev.key_code == bind {
            func();
        }
    }
}
#[derive(Reflect, Clone)]
pub struct MyRaycastSet;

fn update_focused_bobo_text(
    mut text_left: Query<&mut Text, (With<TextTopLeft>, Without<TextTopRight>)>,
    mut text_right: Query<&mut Text, (With<TextTopRight>, Without<TextTopLeft>)>,
    focused_bobo: Query<(&BoboName, Option<&BoboDamageTestResult>), With<FocusedBobo>>,
) {
    let mut text_left = text_left.get_single_mut().unwrap();
    let mut text_right = text_right.get_single_mut().unwrap();
    if let Ok((bobo, damage_test)) = focused_bobo.get_single() {
        *text_left = Text(bobo.0.clone());
        if let Some(status) = damage_test {
            *text_right = Text(status.0.clone());
        }
    } else {
        *text_left = Text(String::new());
        *text_right = Text(String::new());
    }
}

fn update_focused_cube_text(
    focused_cube: Query<(&Parent, &Placement), With<FocusedCube>>,
    graphs: Query<&BoboGraph>,
    mut cube_text: Query<&mut Text, (With<TextBottomLeft>, Without<TextBottomRight>)>,
    mut position_text: Query<&mut Text, (With<TextBottomRight>, Without<TextBottomLeft>)>,
) {
    let mut cube_text = cube_text.get_single_mut().unwrap();
    let mut position_text = position_text.get_single_mut().unwrap();
    if let Ok((bobo, placement)) = focused_cube.get_single() {
        *cube_text = Text(format!("{}", placement.cube));
        *position_text = if let Ok(graph) = graphs.get(bobo.get()) {
            let position = placement.translation;
            let cube = graph.cube(position.x, position.y, position.z).unwrap();
            Text(format!("Health: {:?}", cube))
        } else {
            let position = placement.translation;
            Text(format!("{:?}", (position.x, position.y, position.z)))
        };
    } else {
        *cube_text = Text(String::new());
        *position_text = Text(String::new());
    }
}

fn update_raycast_with_locked_cursor(
    mut commands: Commands,
    mut primary_window: Query<&mut Window, (With<PrimaryWindow>, With<Locked>)>,
    mut query: Query<&mut RaycastSource<MyRaycastSet>>,
    corresponding_bobo: Query<&CorrespondingBobo>,
    corresponding_cube: Query<&CorrespondingCube>,
    focused_bobo: Query<Entity, With<FocusedBobo>>,
    focused_cube: Query<Entity, With<FocusedCube>>,
) {
    if let Ok(window) = primary_window.get_single_mut() {
        let middle = Vec2::new(window.width() / 2., window.height() / 2.);
        if let Ok(bobo) = focused_bobo.get_single() {
            commands.entity(bobo).remove::<FocusedBobo>();
        }
        if let Ok(cube) = focused_cube.get_single() {
            commands.entity(cube).remove::<FocusedCube>();
        }
        let mut pick_source = query.get_single_mut().unwrap();
        pick_source.cast_method = RaycastMethod::Screenspace(middle);
        if let Some((entity, _)) = pick_source.get_nearest_intersection() {
            if let Ok(bobo) = corresponding_bobo.get(entity) {
                commands.entity(bobo.0).insert(FocusedBobo);
            }
            if let Ok(cube) = corresponding_cube.get(entity) {
                commands.entity(cube.0).insert(FocusedCube);
            }
        }
    }
}

fn on_mode_changed(
    mut commands: Commands,
    mut events: EventReader<Mode>,
    mut texts: Query<&mut TextColor>,
    focused_bobo: Query<(Entity, &Children, Option<&BoboGraph>), With<FocusedBobo>>,
    placements: Query<&Placement>,
) {
    for event in events.read() {
        if let Ok((bobo, children, graph)) = focused_bobo.get_single() {
            let mut bobo = commands.entity(bobo);
            if graph.is_none() {
                let mut graph_builder = GraphBuilder::with_capacity(children.len());
                for &bobo_child in children.into_iter() {
                    if let Ok(placement) = placements.get(bobo_child) {
                        graph_builder
                            .push(
                                placement.cube,
                                placement.translation.x,
                                placement.translation.y,
                                placement.translation.z,
                                placement.rotation,
                            )
                            .unwrap();
                    }
                }
                bobo.insert((BoboGraph(graph_builder.build()), ModifiedBobo));
            }
        }
        let color = match event {
            Mode::Edit => Color::Srgba(css::BLUE),
            Mode::Damage => Color::Srgba(css::RED),
        };
        for mut text in texts.iter_mut() {
            *text = TextColor(color);
        }
    }
}

fn update_raycast_with_unlocked_cursor(
    mut commands: Commands,
    lock: Query<&Window, (With<PrimaryWindow>, Without<Locked>)>,
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RaycastSource<MyRaycastSet>>,
    corresponding_bobo: Query<&CorrespondingBobo>,
    corresponding_cube: Query<&CorrespondingCube>,
    focused_bobo: Query<Entity, With<FocusedBobo>>,
    focused_cube: Query<Entity, With<FocusedCube>>,
) {
    if lock.get_single().is_ok() {
        if let Some(cursor_moved) = cursor.read().last() {
            if let Ok(bobo_entity) = focused_bobo.get_single() {
                commands.entity(bobo_entity).remove::<FocusedBobo>();
            }
            if let Ok(cube) = focused_cube.get_single() {
                commands.entity(cube).remove::<FocusedCube>();
            }
            let mut pick_source = query.get_single_mut().unwrap();
            pick_source.cast_method = RaycastMethod::Screenspace(cursor_moved.position);
            if let Some((entity, _)) = pick_source.get_nearest_intersection() {
                if let Ok(bobo) = corresponding_bobo.get(entity) {
                    commands.entity(bobo.0).insert(FocusedBobo);
                }
                if let Ok(cube) = corresponding_cube.get(entity) {
                    commands.entity(cube.0).insert(FocusedCube);
                }
            }
        }
    }
}

fn update_bobo(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    bobos: Query<(Entity, &Children, &BoboGraph), With<ModifiedBobo>>,
    mut cubes: Query<(&mut Visibility, &Transform, &Placement)>,
    hitmarkers: Query<(), With<Hitmarker>>,
) {
    for (bobo_entity, bobo_children, graph) in bobos.iter() {
        for &bobo_child in bobo_children.into_iter() {
            if let Ok((mut v, &transform, placement)) = cubes.get_mut(bobo_child) {
                let position = placement.translation;
                let cube = graph.cube(position.x, position.y, position.z).unwrap();
                if cube.is_destroyed() {
                    *v = Visibility::Hidden;
                } else {
                    *v = Visibility::Visible;
                }
                if graph.initial_hit(cube, 0..1) {
                    //last hit
                    let transform = transform.with_scale(transform.scale * Vec3::splat(0.3));
                    commands.entity(bobo_entity).with_children(|parent| {
                        parent.spawn((
                            transform,
                            SceneRoot(asset_server.load("hitmarker.glb#Scene0")),
                            Hitmarker,
                        ));
                    });
                }
                if graph.initial_hit(cube, 1..2) {
                    //next hit
                    commands.entity(bobo_entity).with_children(|parent| {
                        parent.spawn((
                            transform,
                            SceneRoot(asset_server.load("hitmarker.glb#Scene0")),
                            Hitmarker,
                        ));
                    });
                }
            } else if hitmarkers.get(bobo_child).is_ok() {
                commands.entity(bobo_child).despawn_recursive();
            }
        }
        commands.entity(bobo_entity).remove::<ModifiedBobo>();
    }
}

fn activate_bobo(
    commands: &mut Commands,
    raycaster: &RaycastSource<MyRaycastSet>,
    active_bobo: &Query<Entity, With<ActiveBobo>>,
    corresponding: &Query<(&CorrespondingBobo, &CorrespondingCube)>,
) -> Option<(CorrespondingBobo, CorrespondingCube)> {
    if let Some(intersection) = raycaster.get_nearest_intersection() {
        if let Ok((bobo, cube)) = corresponding.get(intersection.0) {
            if let Ok(bobo_entity) = active_bobo.get_single() {
                commands.entity(bobo_entity).remove::<ActiveBobo>();
            }
            commands.entity(bobo.0).insert(ActiveBobo);
            return Some((*bobo, *cube));
        }
    }
    None
}

pub fn damage_on_click(
    mut commands: Commands,
    menu_state: Res<UiState>,
    mut scenes: Query<(&Parent, &Placement)>,
    active_bobo: Query<Entity, With<ActiveBobo>>,
    mut graphs: Query<&mut BoboGraph>,
    mut events: EventReader<MouseButtonInput>,
    to: Query<&RaycastSource<MyRaycastSet>>,
    corresponding: Query<(&CorrespondingBobo, &CorrespondingCube)>,
) {
    if menu_state.mode != Mode::Damage {
        return;
    }
    let raycaster = to.single();
    for event in events.read() {
        match event.button {
            MouseButton::Left if event.state == ButtonState::Pressed => {
                if let Some((target_bobo, _)) =
                    activate_bobo(&mut commands, raycaster, &active_bobo, &corresponding)
                {
                    let bullets = menu_state.penetration;
                    let bullet_damage = menu_state.damage.div_ceil(bullets); //round up

                    if let Ok(mut graph) = graphs.get_mut(target_bobo.0) {
                        for _n in 0..bullets {
                            for intersection in raycaster.intersections() {
                                if let Ok((_, cube)) = corresponding.get(intersection.0) {
                                    if let Ok((parent, placement)) = scenes.get_mut(cube.0) {
                                        let x = placement.translation.x;
                                        let y = placement.translation.y;
                                        let z = placement.translation.z;
                                        let cube = graph.cube(x, y, z).unwrap();
                                        if parent.get() == target_bobo.0 && !cube.is_destroyed() {
                                            //set target
                                            graph.target(x, y, z).unwrap();

                                            break;
                                        }
                                    }
                                }
                            }
                            graph.damage(bullet_damage); //apply damage
                        }
                        dbg!(graph.commit());
                        commands.entity(target_bobo.0).insert(ModifiedBobo);
                    }
                }
            }
            _ => (),
        }
    }
}

pub fn delete_on_click(
    mut commands: Commands,
    menu_state: Res<UiState>,
    mut revert: Query<&mut Revert>,
    mut scenes: Query<(&Parent, Entity, &mut Visibility, &Placement)>,
    active_bobo: Query<Entity, With<ActiveBobo>>,
    mut events: EventReader<MouseButtonInput>,
    to: Query<&RaycastSource<MyRaycastSet>>,
    corresponding: Query<(&CorrespondingBobo, &CorrespondingCube)>,
) {
    if menu_state.mode != Mode::Edit {
        return;
    }
    let raycaster = to.single();
    for event in events.read() {
        match event.button {
            MouseButton::Left if event.state == ButtonState::Pressed => {
                if let Some((bobo, cube)) =
                    activate_bobo(&mut commands, raycaster, &active_bobo, &corresponding)
                {
                    commands.entity(*bobo).remove::<BoboGraph>();
                    let id = cube.0;
                    if let Ok((_, _, mut v, placement)) = scenes.get_mut(id) {
                        let mut revert = revert.get_mut(*bobo).unwrap();
                        debug!("hiding {}", placement.cube);
                        *v = Visibility::Hidden;
                        revert.push_single(id);
                    }
                }
            }
            MouseButton::Right if event.state == ButtonState::Pressed => {
                if let Some((bobo, cube)) =
                    activate_bobo(&mut commands, raycaster, &active_bobo, &corresponding)
                {
                    commands.entity(*bobo).remove::<BoboGraph>();
                    if let Ok((p, _, _, placement)) = scenes.get_mut(cube.0) {
                        let mut revert = revert.get_mut(*bobo).unwrap();
                        let bobo = p.get();
                        let cube = placement.cube;
                        let mut cubes = Vec::new();
                        for (p, entity, mut v, placement) in scenes.iter_mut() {
                            let c = placement.cube;
                            if p.get() == bobo && c.id == cube.id && *v != Visibility::Hidden {
                                debug!("hiding {}", c);
                                *v = Visibility::Hidden;
                                cubes.push(entity);
                            }
                        }
                        revert.push_multiple(cubes);
                    }
                }
            }
            _ => (),
        }
    }
}

fn toggle_mode(
    config: Res<Config>,
    keys: Res<ButtonInput<KeyCode>>,
    mut events: EventWriter<Mode>,
    mut menu_state: ResMut<UiState>,
) {
    if keys.just_pressed(config.toggle_mode) {
        events.send(!menu_state.mode);
        menu_state.mode = !menu_state.mode;
    }
}

fn delete_bobo(
    mut commands: Commands,
    config: Res<Config>,
    key_event: EventReader<KeyboardInput>,
    mut bobos: Query<Entity, With<FocusedBobo>>,
) {
    if let Ok(bobo) = bobos.get_single_mut() {
        run_for_keybind(key_event, &config.delete_bobo, || {
            commands.entity(bobo).despawn_recursive();
        });
    }
}

fn undo_redo_damage(
    mut commands: Commands,
    config: Res<Config>,
    key_event: EventReader<KeyboardInput>,
    mut bobos: Query<(Entity, &mut BoboGraph), With<ActiveBobo>>,
    menu_state: Res<UiState>,
) {
    if menu_state.mode != Mode::Damage {
        return;
    }
    let Ok((bobo, mut graph)) = bobos.get_single_mut() else {
        return;
    };
    run_for_key_presses(key_event, |code| match code {
        c if c == &config.undo => {
            if graph.prev() {
                dbg!(graph.damage_dealt());
                commands.entity(bobo).insert(ModifiedBobo);
            }
        }
        c if c == &config.redo => {
            if graph.next_commit() {
                dbg!(graph.damage_dealt());
                commands.entity(bobo).insert(ModifiedBobo);
            }
        }
        _ => {}
    });
}

fn receive_progress(
    mut commands: Commands,
    mut damage_test_channels: Query<(Entity, &DamageTestChannel)>,
) {
    for (bobo, channel) in damage_test_channels.iter_mut() {
        let mut commands = commands.entity(bobo);
        while let Ok(progress) = channel.try_recv() {
            match progress {
                DamageTestProgress::Progress(progress) => {
                    commands.insert(BoboDamageTestResult(format!("{progress}%")));
                }
                DamageTestProgress::Done { graph, total_shots } => {
                    commands
                        .insert((
                            BoboGraph(graph),
                            ModifiedBobo,
                            BoboDamageTestResult(format!("Took {total_shots} shots")),
                        ))
                        .remove::<DamageTestChannel>();
                }
            }
        }
    }
}

fn damage_test(
    mut commands: Commands,
    config: Res<Config>,
    key_event: EventReader<KeyboardInput>,
    damage_test_channels: Query<(), With<DamageTestChannel>>,
    mut graphs: Query<&mut BoboGraph>,
    active_bobo: Query<Entity, With<ActiveBobo>>,
    menu_state: Res<UiState>,
    to: Query<&RaycastSource<MyRaycastSet>>,
    corresponding: Query<(&CorrespondingBobo, &CorrespondingCube)>,
) {
    if menu_state.mode != Mode::Damage {
        return;
    }
    let raycaster = to.single();
    run_for_keybind(key_event, &config.test, || {
        let Some((bobo, Ok(mut graph))) =
            activate_bobo(&mut commands, raycaster, &active_bobo, &corresponding).and_then(
                |(bobo, _)| match damage_test_channels.get(*bobo).is_ok() {
                    true => None,
                    false => Some((bobo, graphs.get_mut(*bobo).map(|g| g.clone()))),
                },
            )
        else {
            return;
        };
        let (tx, rx) = bounded::<DamageTestProgress>(10);
        commands.entity(*bobo).insert(DamageTestChannel(rx));
        let damage = (menu_state.damage - 1) / menu_state.penetration + 1;
        std::thread::spawn(move || {
            let mut damage_test = graph.damage_test(damage, 1);
            for progress in &mut damage_test {
                tx.send(DamageTestProgress::Progress((progress * 100.0) as u8))
                    .unwrap();
            }
            let total_shots = damage_test.get_shots();
            graph.reset();
            tx.send(DamageTestProgress::Done { graph, total_shots })
                .unwrap();
        });
    });
}

fn undo_redo_edit(
    config: Res<Config>,
    key_event: EventReader<KeyboardInput>,
    active_bobo: Query<Entity, With<ActiveBobo>>,
    mut revert: Query<&mut Revert>,
    mut scenes: Query<(&mut Visibility, &Placement)>,
    menu_state: Res<UiState>,
) {
    if menu_state.mode != Mode::Edit {
        return;
    }
    if let Ok(bobo) = active_bobo.get_single() {
        let mut revert = revert.get_mut(bobo).unwrap();
        run_for_key_presses(key_event, |code| match code {
            c if c == &config.undo => match revert.prev() {
                Some(Single(entity)) => {
                    let (mut v, placement) = scenes.get_mut(*entity).unwrap();
                    debug!("showing {}", placement.cube);
                    *v = Visibility::Visible;
                }
                Some(Multiple(cubes)) => {
                    for entity in cubes {
                        let (mut v, placement) = scenes.get_mut(*entity).unwrap();
                        debug!("showing {}", placement.cube);
                        *v = Visibility::Visible;
                    }
                }
                _ => (),
            },
            c if c == &config.redo => match revert.next_item() {
                Some(Single(entity)) => {
                    let (mut v, placement) = scenes.get_mut(*entity).unwrap();
                    debug!("hiding {}", placement.cube);
                    *v = Visibility::Hidden;
                }
                Some(Multiple(cubes)) => {
                    for entity in cubes {
                        let (mut v, placement) = scenes.get_mut(*entity).unwrap();
                        debug!("hiding {}", placement.cube);
                        *v = Visibility::Hidden;
                    }
                }
                _ => (),
            },
            _ => {}
        });
    }
}

fn reset(
    mut commands: Commands,
    mut active_bobo: Query<(Entity, &mut BoboGraph), With<ActiveBobo>>,
    config: Res<Config>,
    key_event: EventReader<KeyboardInput>,
) {
    if let Ok((bobo, mut graph)) = active_bobo.get_single_mut() {
        run_for_keybind(key_event, &config.reset, || {
            graph.reset();
            commands.entity(bobo).insert(ModifiedBobo);
        });
    }
}

fn toggle_background(
    config: Res<Config>,
    key_event: EventReader<KeyboardInput>,
    mut clear_color: ResMut<ClearColor>,
) {
    run_for_keybind(key_event, &config.toggle_background, || {
        *clear_color.as_mut() = ClearColor(
            if clear_color
                .to_linear()
                .to_f32_array_no_alpha()
                .into_iter()
                .reduce(|acc, e| acc + e)
                .unwrap()
                < 3.0
            {
                info!("Changed background color from black to white");
                Color::WHITE
            } else {
                info!("Changed background color from white to black");
                Color::BLACK
            },
        );
    });
}

const ASSET_SCALE: f32 = 5.0;

pub fn spawn_bobo(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    name: String,
    placements: Vec<bobocraft_format::Placement>,
    mut bobos: Query<&mut Transform, With<BoboName>>,
) {
    debug!("spawning bobo named {name}");
    let scale = Transform::from_scale(Vec3::splat(ASSET_SCALE));
    let mut entity_commands = commands.spawn_empty();
    let bobo_id = entity_commands.id();

    entity_commands.insert((
        Revert::default(),
        Transform::default(),
        Visibility::default(),
        BoboName(name.clone()),
    ));

    let mut max = Vec3::splat(f32::MAX);

    entity_commands.with_children(|parent| {
        for p in placements.iter() {
            max.x = f32::min(max.x, p.translation.x as f32 * -1.0);
            max.y = f32::min(max.y, p.translation.z as f32);
            max.z = f32::min(max.z, p.translation.y as f32);
        }

        for p in placements {
            debug!("spawning {} at {:?}", p.cube, p.translation);
            let translation = Vec3::new(
                p.translation.x as f32 * -1.0,
                p.translation.z as f32,
                p.translation.y as f32,
            );

            let rotation = Quat::from_euler(
                EulerRot::YZX,
                (p.rotation.z as f32).to_radians(),
                (p.rotation.y as f32).to_radians(),
                (p.rotation.x as f32).to_radians() * -1.0,
            );
            let transform = scale.with_translation(translation).with_rotation(rotation);
            let mut entity_commands = parent.spawn_empty();
            let cube_id = entity_commands.id();
            entity_commands.insert((
                HookedSceneBundle {
                    scene: SceneRoot(
                        asset_server.load(format!("embedded://gltf/{:?}.glb#Scene0", p.cube)),
                    ),
                    hook: SceneHook::new(move |entity, commands| {
                        if entity.get::<Mesh3d>().is_some() {
                            commands.insert(RaycastMesh::<MyRaycastSet>::default());
                            commands.insert(CorrespondingCube(cube_id));
                            commands.insert(CorrespondingBobo(bobo_id));
                            //_commands.insert(p.color);
                        }
                    }),
                },
                transform,
                Placement(p),
            ));

            let scale = Transform::from_scale(Vec3::new(1.0, 1.0, 0.1));
            for c in p.cube.connections {
                info!("{:?}", c);
                let translation = Vec3::new(
                    (c.x as f32) / (ASSET_SCALE * 2.0),
                    (c.z as f32) / (ASSET_SCALE * 2.0),
                    (c.y as f32) / (ASSET_SCALE * 2.0),
                );
                let rotation = Quat::from_euler(
                    EulerRot::YZX,
                    ((if c.x % 2 == 0 { 0 } else { 90 }) as f32).to_radians(),
                    ((if c.y % 2 == 0 { 0 } else { 0 }) as f32).to_radians(),
                    ((if c.z % 2 == 0 { 0 } else { 90 }) as f32).to_radians(),
                );

                let transform = scale.with_translation(translation).with_rotation(rotation);

                entity_commands.with_children(|cube| {
                    let mut entity_commands = cube.spawn_empty();
                    entity_commands.insert((
                        HookedSceneBundle {
                            scene: SceneRoot(
                                asset_server.load(format!("gltf/medium_cube.glb#Scene0")),
                            ),
                            hook: SceneHook::new(move |entity, commands| {
                                if entity.get::<Mesh3d>().is_some() {
                                    commands.insert(RaycastMesh::<MyRaycastSet>::default());
                                    //_commands.insert(p.color);
                                }
                            }),
                        },
                        transform,
                    ));
                });
            }
        }
        info!("bobo named {name} spawned");
    });
}

#[derive(Event, Debug, Clone)]
pub enum DamageTestProgress {
    Progress(u8),
    Done {
        graph: DamageGraph,
        total_shots: u32,
    },
}

#[derive(Event, Debug, PartialEq, Clone, Copy)]
enum Mode {
    Edit,
    Damage,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Edit => write!(f, "Edit"),
            Mode::Damage => write!(f, "Damage"),
        }
    }
}

impl Not for Mode {
    type Output = Mode;

    fn not(self) -> Self::Output {
        match self {
            Mode::Edit => Mode::Damage,
            Mode::Damage => Mode::Edit,
        }
    }
}

#[derive(Debug, Resource)]
pub struct UiState {
    mode: Mode,
    damage: u32,
    penetration: u32,
}

impl Default for UiState {
    fn default() -> UiState {
        let default_damage = 230000;
        let default_penetration = 3;
        UiState {
            mode: Mode::Edit,
            damage: default_damage,
            penetration: default_penetration,
        }
    }
}

const FONT_SIZE: f32 = 40.0;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("embedded://fonts/FiraMono-Medium.ttf");
    let color = Color::Srgba(css::BLUE);
    commands
        .spawn((
            Text(String::new()),
            TextColor(color),
            TextFont {
                font: font.clone(),
                font_size: FONT_SIZE,
                ..Default::default()
            },
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
        ))
        .insert(TextTopLeft);

    commands
        .spawn((
            Text(String::new()),
            TextColor(color),
            TextFont {
                font: font.clone(),
                font_size: FONT_SIZE,
                ..Default::default()
            },
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
        ))
        .insert(TextTopRight);

    commands
        .spawn((
            Text(String::new()),
            TextColor(color),
            TextFont {
                font: font.clone(),
                font_size: FONT_SIZE,
                ..Default::default()
            },
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(0.0),
                bottom: Val::Px(0.0),
                ..default()
            },
        ))
        .insert(TextBottomRight);

    commands
        .spawn((
            Text(String::new()),
            TextColor(color),
            TextFont {
                font: font.clone(),
                font_size: FONT_SIZE,
                ..Default::default()
            },
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                bottom: Val::Px(0.0),
                ..default()
            },
        ))
        .insert(TextBottomLeft);
}

fn setup_crosshair(mut commands: Commands) {
    commands
        .spawn(Node {
            // Take the size of the parent node.
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(Node {
                    // Take the size of the parent node.
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                // Take the size of the parent node.
                                width: Val::Px(5.0),
                                height: Val::Px(5.0),
                                ..default()
                            },
                            BackgroundColor(Color::Srgba(css::LIMEGREEN)),
                        ))
                        .insert(Crosshair);
                });
        });
}

fn setup_lights(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 2000.0,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 10.0, 5.0),
            rotation: Quat::from_euler(EulerRot::XYZ, -PI / 8.0, -PI / 8.0, -PI / 8.0),
            ..default()
        },
    ));

    commands.spawn((
        DirectionalLight {
            color: Color::WHITE,
            illuminance: 8000.0,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 10.0, 5.0),
            rotation: Quat::from_euler(EulerRot::XYZ, -PI / 2.0, 0.0, 0.0),
            ..default()
        },
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.0,
    });
}

fn crosshair_visibility(
    mut crosshairs: Query<&mut Visibility, With<Crosshair>>,
    config: Res<Config>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(config.toggle_cursor_grab) {
        let mut crosshair_vis = crosshairs.single_mut();
        *crosshair_vis = if *crosshair_vis == Visibility::Hidden {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}
#[allow(clippy::type_complexity)]
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut events: EventWriter<Mode>,
    mut menu_state: ResMut<UiState>,
) {
    for (interaction, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                events.send(!menu_state.mode);
                menu_state.mode = !menu_state.mode;
                *text = Text(menu_state.mode.to_string())
            }
            Interaction::Hovered => border_color.0 = Color::WHITE,
            Interaction::None => border_color.0 = BORDER_COLOR_INACTIVE,
        }
    }
}

fn menu_visibility(
    menu: Query<Entity, With<Menu>>,
    config: Res<Config>,
    menu_state: Res<UiState>,
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if keys.just_pressed(config.toggle_cursor_grab) {
        if let Ok(entity) = menu.get_single() {
            commands.entity(entity).despawn_recursive();
        } else {
            let font = asset_server.load("embedded://fonts/FiraMono-Medium.ttf");
            let color = match menu_state.mode {
                Mode::Edit => Color::Srgba(css::BLUE),
                Mode::Damage => Color::Srgba(css::RED),
            };

            commands
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    // Make this container node to be Interactive so that clicking on it removes
                    // focus from the text input.
                    Interaction::None,
                    Menu,
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                width: Val::Px(150.0),
                                border: UiRect::all(Val::Px(5.0)),
                                padding: UiRect::all(Val::Px(5.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::BLACK),
                            BorderColor(BORDER_COLOR_INACTIVE),
                            Button,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text(menu_state.mode.to_string()),
                                TextFont {
                                    font: font.clone(),
                                    font_size: FONT_SIZE,
                                    ..Default::default()
                                },
                                TextColor(color),
                            ));
                        });
                    parent.spawn((
                        PenetrationText,
                        Node {
                            width: Val::Px(100.0),
                            border: UiRect::all(Val::Px(5.0)),
                            padding: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        BorderColor(BORDER_COLOR_INACTIVE),
                        BackgroundColor(Color::BLACK),
                        TextInput,
                        TextInputTextColor(TextColor(color)),
                        TextInputTextFont(TextFont {
                            font: font.clone(),
                            font_size: FONT_SIZE,
                            ..Default::default()
                        }),
                        TextInputSettings {
                            retain_on_submit: true,
                            mask_character: None,
                        },
                        TextInputValue(menu_state.penetration.to_string()),
                        TextInputInactive(true),
                    ));
                    parent.spawn((
                        DamageText,
                        Node {
                            width: Val::Px(200.0),
                            border: UiRect::all(Val::Px(5.0)),
                            padding: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        BorderColor(BORDER_COLOR_INACTIVE),
                        BackgroundColor(Color::BLACK),
                        TextInput,
                        TextInputTextColor(TextColor(color)),
                        TextInputTextFont(TextFont {
                            font,
                            font_size: FONT_SIZE,
                            ..Default::default()
                        }),
                        TextInputSettings {
                            retain_on_submit: true,
                            mask_character: None,
                        },
                        TextInputValue(menu_state.damage.to_string()),
                        TextInputInactive(true),
                    ));
                });
        }
    }
}

fn focus(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInputInactive, &mut BorderColor)>,
) {
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive, mut border_color) in &mut text_input_query {
                if entity == interaction_entity {
                    inactive.0 = false;
                    *border_color = Color::WHITE.into();
                } else {
                    inactive.0 = true;
                    *border_color = BORDER_COLOR_INACTIVE.into();
                }
            }
        }
    }
}

fn text_listener(
    mut events: EventReader<TextInputSubmitEvent>,
    mut penetration_text: Query<&mut TextInputValue, (With<PenetrationText>, Without<DamageText>)>,
    mut damage_text: Query<&mut TextInputValue, (With<DamageText>, Without<PenetrationText>)>,
    mut menu_state: ResMut<UiState>,
) {
    for event in events.read() {
        if let Ok(mut value) = penetration_text.get_mut(event.entity) {
            if let Ok(parsed) = event.value.parse() {
                menu_state.penetration = parsed;
            } else {
                value.0 = menu_state.penetration.to_string();
            }
        }
        if let Ok(mut value) = damage_text.get_mut(event.entity) {
            if let Ok(parsed) = event.value.parse() {
                menu_state.damage = parsed;
            } else {
                value.0 = menu_state.damage.to_string();
            }
        }
    }
}

const BORDER_COLOR_INACTIVE: Color = Color::srgb(0.25, 0.25, 0.25);

fn main() -> Fallible {
    #[cfg(windows)]
    let _ = nu_ansi_term::enable_ansi_support();
    let model_path = match env::args().nth(1) {
        Some(model_path) => PathBuf::from(model_path),
        None => return Ok(()),
    };

    let config = config::get()?;

    let plugins = DefaultPlugins.build().set(WindowPlugin {
        primary_window: Some(Window {
            title: String::from(env!("CARGO_PKG_NAME")),
            ..default()
        }),
        ..default()
    });

    let mut app = App::new();

    app.insert_resource(config)
        .insert_resource(RaycastPluginState::<MyRaycastSet>::default())
        .insert_resource(WinitSettings::game())
        .insert_resource(ClearColor(Color::BLACK));

    app.init_resource::<UiState>();

    app.add_plugins((
        plugins,
        EmbeddedAssetPlugin::default(),
        PhysicsPlugins::default(),
        DeferredRaycastingPlugin::<MyRaycastSet>::default(),
        HookPlugin,
        CameraPlugin,
        TextInputPlugin,
    ));
    #[cfg(feature = "editor")]
    app.add_plugins(bevy_editor_pls::EditorPlugin::default());

    app.add_systems(
        First,
        (
            update_raycast_with_locked_cursor.before(RaycastSystem::BuildRays::<MyRaycastSet>),
            update_raycast_with_unlocked_cursor.before(RaycastSystem::BuildRays::<MyRaycastSet>),
            receive_progress,
        ),
    )
    .add_systems(
        Update,
        (
            toggle_mode,
            button_system,
            text_listener,
            menu_visibility,
            focus,
            undo_redo_damage,
            undo_redo_edit,
            reset,
            damage_test,
            toggle_background,
            crosshair_visibility,
            delete_on_click.run_if(not_any_with_component::<Menu>),
            damage_on_click.run_if(not_any_with_component::<Menu>),
            on_mode_changed.after(button_system),
            update_focused_bobo_text,
            update_focused_cube_text,
            delete_bobo.run_if(not_any_with_component::<Menu>),
        ),
    )
    .add_systems(PostUpdate, update_bobo);

    app.add_systems(Startup, (setup_ui, setup_crosshair, setup_lights));

    app.add_event::<Mode>().add_event::<DamageTestProgress>();

    let (name, placements) =
        bobocraft_format::from_file(&model_path).context("loading bobo model failed")?;
    app.add_systems(
        Startup,
        move |commands: Commands,
              asset_server: Res<AssetServer>,
              bobos: Query<&mut Transform, With<BoboName>>| {
            spawn_bobo(
                commands,
                asset_server,
                name.clone(),
                placements.clone(),
                bobos,
            );
        },
    );

    app.run();
    Ok(())
}
