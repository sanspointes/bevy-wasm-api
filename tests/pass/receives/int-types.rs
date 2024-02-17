use bevy_wasm_api_macro::bevy_wasm_api;
use wasm_bindgen::prelude::*;
use bevy_ecs::world::World;

#[wasm_bindgen(skip_typescript)]
struct MyApi;
#[bevy_wasm_api]
impl MyApi {
    pub fn test_i32(_world: &mut World, _help: i32) {
        todo!();
    }
    pub fn test_u32(_world: &mut World, _help: u32) {
        todo!();
    }
    pub fn test_i16(_world: &mut World, _help: i16) {
        todo!();
    }
    pub fn test_u16(_world: &mut World, _help: u16) {
        todo!();
    }
    pub fn test_i8(_world: &mut World, _help: i8) {
        todo!();
    }
    pub fn test_u8(_world: &mut World, _help: u8) {
        todo!();
    }
}

pub fn main() {
}
