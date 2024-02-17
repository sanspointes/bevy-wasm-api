use bevy::{ecs::system::SystemState, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_wasm_api::bevy_wasm_api;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    // console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn setup_bevy_app(canvas_id: String) {
    let mut app = App::new();

    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "wasm-app example".to_string(),
            resolution: (10., 10.).into(),
            canvas: Some(canvas_id),
            fit_canvas_to_parent: true,
            ..Default::default()
        }),
        ..Default::default()
    });

    app.add_plugins(default_plugins);
    app.run();
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
    pub fn get_entities(world: &mut World) -> Vec<Entity> {
        let result: Vec<_> = world.query_filtered::<Entity, With<Name>>().iter(world).collect();
        result
    }
    pub fn set_entity_name(world: &mut World, entity: u32, name: String) -> Result<bool, String> {
        let mut name_component = world.get_mut::<Name>(Entity::from_raw(entity)).ok_or("Could not find entity".to_string())?;
        name_component.set(name);
        Ok(true)
    }
    pub fn get_entity_name(world: &mut World, entity: u32) -> Option<String> {
        world.get::<Name>(Entity::from_raw(entity)).map(|name| name.to_string())
    }

    pub fn spawn_circle(world: &mut World, x: f32, y: f32, z: f32) -> Entity {
        let mut sys_state = SystemState::<(
            Commands,
            ResMut<Assets<Mesh>>,
            ResMut<Assets<ColorMaterial>>,
        )>::new(world);

        let (mut commands, mut meshes, mut materials) = sys_state.get_mut(world);
        let entity = commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(50.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(Vec3::new(x, y, z)),
                ..default()
            },
            Name::from("Circle"),
        )).id();

        sys_state.apply(world);

        entity
    }
}
