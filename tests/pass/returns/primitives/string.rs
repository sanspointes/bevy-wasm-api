use bevy_wasm_api_macro::bevy_wasm_api;
use wasm_bindgen::prelude::*;
use bevy_ecs::world::World;

#[wasm_bindgen(skip_typescript)]
struct MyApi;
#[bevy_wasm_api]
impl MyApi {
    pub fn test_string(_world: &mut World) -> String {
        todo!();
    }
}

pub fn main() {
}
