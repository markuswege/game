// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]
mod mouse_controls_camera;

use bevy::asset::AssetMetaCheck;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::math::{uvec2, vec2};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_fast_tilemap::bundle::MapBundleManaged;
use bevy_fast_tilemap::map::{Map, MapIndexerMut};
use bevy_fast_tilemap::plugin::FastTileMapPlugin;
use bevy_fast_tilemap::prelude::AXONOMETRIC;
use mouse_controls_camera::MouseControlsCameraPlugin;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics in web builds on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("IsoTest Bevy"),
                        resolution: (1820., 920.).into(),
                        present_mode: PresentMode::Immediate,
                        ..default()
                    }),
                    ..default()
                }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            MouseControlsCameraPlugin::default(),
            FastTileMapPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<Map>>,
) {
    commands.spawn(Camera2dBundle::default());

    let map = Map::builder(
        uvec2(100, 100),
        asset_server.load("iso_256x128_dominance.png"),
        vec2(256.0, 128.0),
    )
    .with_padding(vec2(256.0, 128.0), vec2(256.0, 128.0), vec2(256.0, 128.0))
    .with_projection(AXONOMETRIC)
    .with_dominance_overhang()
    .build_and_initialize(init_map);

    commands.spawn(MapBundleManaged::new(map, materials.as_mut()));
}
/// Fill the map with a random pattern
fn init_map(m: &mut MapIndexerMut) {
    let mut rng = rand::thread_rng();
    for y in 0..m.size().y {
        for x in 0..m.size().x {
            m.set(x, y, rng.gen_range(1..4));
        }
    }
}
