use bevy_wasm_api_macro::bevy_wasm_api;
use wasm_bindgen::prelude::*;
use bevy_ecs::world::World;

#[wasm_bindgen(skip_typescript)]
struct MyApi;
#[bevy_wasm_api]
impl MyApi {
    pub fn test_f64(_world: &mut World, _help: f64) {
        todo!();
    }
    pub fn test_f32(_world: &mut World, _help: f32) {
        todo!();
    }
}

pub fn main() {
}
