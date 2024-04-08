use bevy_wasm_api_macro::bevy_wasm_api;
use wasm_bindgen::prelude::*;
use bevy_ecs::world::World;

#[wasm_bindgen(skip_typescript)]
struct MyApi;
#[bevy_wasm_api]
impl MyApi {
    pub fn test_str_ref(_world: &mut World, help: &str) {
        println!("Help me please {help}");
    }
    pub fn test_string(_world: &mut World, help: &str) {
        println!("Help me please {help}");
    }
}

pub fn main() {
}
