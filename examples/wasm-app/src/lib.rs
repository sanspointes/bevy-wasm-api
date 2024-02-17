use bevy::{ecs::system::SystemState, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_wasm_api::bevy_wasm_api;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    // console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn setup_bb_core(canvas_id: String) {
    let mut app = App::new();

    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bobbin Bear :: Embroidery Editor".to_string(),
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
    pub fn test_result(_world: &mut World) -> Result<bool, bool> {
        Ok(true)
    }

    pub fn count_entites(world: &mut World) -> usize {
        world.query::<Entity>().iter(world).len()
    }

    pub fn test_string(_world: &mut World) -> String {
        "Hello".to_string()
    }

    pub fn test_struct(_world: &mut World) -> MyStruct {
        MyStruct::default()
    }

    pub fn my_method(world: &mut World, r: f32, g: f32, b: f32) {
        let mut sys_state = SystemState::<(
            Commands,
            ResMut<Assets<Mesh>>,
            ResMut<Assets<ColorMaterial>>,
        )>::new(world);

        let (mut commands, mut meshes, mut materials) = sys_state.get_mut(world);
        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::rgb(r, g, b))),
            transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
            ..default()
        });

        sys_state.apply(world);
    }
}
