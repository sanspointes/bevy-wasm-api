use bevy_wasm_api_macro::bevy_wasm_api;
use wasm_bindgen::prelude::*;
use bevy_ecs::world::World;

#[wasm_bindgen]
#[derive(serde::Serialize, serde::Deserialize)]
struct MyStruct {
    field1: i32,
}

#[wasm_bindgen(skip_typescript)]
struct MyApi;
#[bevy_wasm_api]
impl MyApi {
    pub fn test(_world: &mut World) -> (Option<MyStruct>, f32) {
        todo!();
    }
}

pub fn main() {
}
