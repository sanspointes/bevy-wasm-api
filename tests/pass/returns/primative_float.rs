use bevy_wasm_api_macro::bevy_wasm_api;
use wasm_bindgen::prelude::*;
use bevy_ecs::world::World;

#[wasm_bindgen(skip_typescript)]
struct MyApi;
#[bevy_wasm_api]
impl MyApi {
    pub fn test_i32(_world: &mut World) -> i32 {
        todo!();
    }
    pub fn test_u32(_world: &mut World) -> u32 {
        todo!();
    }
    pub fn test_i16(_world: &mut World) -> i16 {
        todo!();
    }
    pub fn test_u16(_world: &mut World) -> u16 {
        todo!();
    }
    pub fn test_i8(_world: &mut World) -> i8 {
        todo!();
    }
    pub fn test_u8(_world: &mut World) -> u8 {
        todo!();
    }
}

pub fn main() {
}

