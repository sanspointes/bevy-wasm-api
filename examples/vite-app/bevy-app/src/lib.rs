mod utils;

use bevy::{ecs::system::SystemState, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_wasm_api::bevy_wasm_api;
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
            ..Default::default()
        }),
        ..Default::default()
    });

    app.add_plugins(bevy_wasm_api::BevyWasmApiPlugin::default());
    app.add_plugins(default_plugins);
    app.add_systems(Startup, setup);
    app.run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
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

    pub fn set_entity_position(world: &mut World, entity: u32, x: f32, y: f32, z: f32) -> Result<bool, String> {
        let mut transform = world.get_mut::<Transform>(Entity::from_raw(entity)).ok_or("Could not find entity".to_string())?;
        transform.translation.x = x;
        transform.translation.y = y;
        transform.translation.z = z;
        Ok(true)
    }
    pub fn get_entity_position(world: &mut World, entity: u32) -> Option<(f32, f32, f32)> {
        let transform = world.get::<Transform>(Entity::from_raw(entity));
        transform.map(|transform| {
            let pos = transform.translation;
            (pos.x, pos.y, pos.z)
        })
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
                mesh: meshes.add(bevy::math::primitives::Circle::new(10.)).into(),
                material: materials.add(ColorMaterial::from(Color::srgb(1., 0., 0.))),
                transform: Transform::from_translation(Vec3::new(x, y, z)),
                ..default()
            },
            Name::from("Circle"),
        )).id();

        sys_state.apply(world);

        entity
    }
}
