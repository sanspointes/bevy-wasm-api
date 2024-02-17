use bevy_wasm_api_macro::bevy_wasm_api;
use wasm_bindgen::prelude::*;
use bevy_ecs::world::World;

#[wasm_bindgen(skip_typescript)]
struct MyApi;
#[bevy_wasm_api]
impl MyApi {
    pub fn test_bool(_world: &mut World, _help: bool) {
        todo!();
    }
    pub fn test_optional_bool(_world: &mut World, _help: Option<bool>) {
        todo!();
    }
}

pub fn main() {
}
