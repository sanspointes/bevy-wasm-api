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

#[wasm_bindgen(skip_typescript)]
struct MyApi;

#[bevy_wasm_api]
impl MyApi {
    pub fn test(_world: &mut World) -> Option<(i32, f32)> {
        todo!();
    }
}
