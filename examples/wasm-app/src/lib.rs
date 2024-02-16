use bevy::{ecs::system::SystemState, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_wasm_api::bevy_wasm_api;
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

#[wasm_bindgen(skip_typescript)]
struct MyApi;

#[bevy_wasm_api]
impl MyApi {
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

// impl MyApi {
//     pub fn my_method(world: &mut World, r: f32, g: f32, b: f32) {
//         let mut sys_state = SystemState::<(
//             Commands,
//             ResMut<Assets<Mesh>>,
//             ResMut<Assets<ColorMaterial>>,
//         )>::new(world);
//         let (mut commands, mut meshes, mut materials) = sys_state.get_mut(world);
//         commands.spawn(MaterialMesh2dBundle {
//             mesh: meshes.add(shape::Circle::new(50.).into()).into(),
//             material: materials.add(ColorMaterial::from(Color::rgb(r, g, b))),
//             transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
//             ..default()
//         });
//         sys_state.apply(world);
//     }
// }
// #[wasm_bindgen(typescript_custom_section)]
// const TS_APPEND_CONTENT: &'static str =
//     "\nexport class MyApi {\nmy_method_wasm() Promise<void>;\n }";
//
// #[wasm_bindgen]
// impl MyApi {
//     #[wasm_bindgen]
//     pub fn my_method_wasm(&self, r: f32, g: f32, b: f32) -> bevy_wasm_api::Promise {
//         bevy_wasm_api::future_to_promise(bevy_wasm_api::execute_in_world(
//             bevy_wasm_api::ExecutionChannel::FrameStart,
//             move |world: &mut World| {
//                 let response = MyApi::my_method(world, r, g, b);
//                 Ok(bevy_wasm_api::JsValue::UNDEFINED)
//             },
//         ))
//     }
// }
