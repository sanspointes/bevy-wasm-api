mod utils;

use bevy::{ecs::system::SystemState, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_wasm_api::bevy_wasm_api;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub fn setup_bevy_app(canvas_id: String) {
    let mut app = App::new();

    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Vite Example".to_string(),
            resolution: (10., 10.).into(),
            canvas: Some(canvas_id),
            fit_canvas_to_parent: true,
            ..Default::default()
        }),
        ..Default::default()
    });

    app.add_plugins(bevy_wasm_api::BevyWasmApiPlugin);
    app.add_plugins(default_plugins);
    app.add_systems(Startup, setup);
    app.run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Default, Tsify, serde::Deserialize, serde::Serialize)]
struct MyStruct {
    fielda: i32,
    fieldb: String,
}

#[allow(dead_code)]
struct MyApi;

#[allow(dead_code)]
#[bevy_wasm_api]
impl MyApi {
    pub fn count_entites(world: &mut World) -> usize {
        world.query::<Entity>().iter(world).len()
    }

    pub fn spawn_box(world: &mut World, x: f32, y: f32, z: f32) -> Entity {
        let mut sys_state = SystemState::<(
            Commands,
            ResMut<Assets<Mesh>>,
            ResMut<Assets<ColorMaterial>>,
        )>::new(world);

        let (mut commands, mut meshes, mut materials) = sys_state.get_mut(world);
        let entity = commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(Vec3::new(x, y, z)),
            ..default()
        }).id();

        sys_state.apply(world);

        entity
    }
}
