use bevy_wasm_api_macro::bevy_wasm_api;
use wasm_bindgen::prelude::*;
use bevy_ecs::world::World;

#[wasm_bindgen(skip_typescript)]
struct MyApi;
#[bevy_wasm_api]
impl MyApi {
    pub fn test_f64_slice_ref(_world: &mut World, _help: &[f64]) {
        todo!();
    }
    pub fn test_f32_slice_ref(_world: &mut World, _help: &[f32]) {
        todo!();
    }
    pub fn test_i64_slice_ref(_world: &mut World, _help: &[i64]) {
        todo!();
    }
    pub fn test_i32_slice_ref(_world: &mut World, _help: &[i32]) {
        todo!();
    }
    pub fn test_i16_slice_ref(_world: &mut World, _help: &[i16]) {
        todo!();
    }
    pub fn test_i8_slice_ref(_world: &mut World, _help: &[i8]) {
        todo!();
    }
    pub fn test_u64_slice_ref(_world: &mut World, _help: &[u64]) {
        todo!();
    }
    pub fn test_u32_slice_ref(_world: &mut World, _help: &[u32]) {
        todo!();
    }
    pub fn test_u16_slice_ref(_world: &mut World, _help: &[u16]) {
        todo!();
    }
    pub fn test_u8_slice_ref(_world: &mut World, _help: &[u8]) {
        todo!();
    }
    pub fn test_f64_slice_ref_mut(_world: &mut World, _help: &mut [f64]) {
        todo!();
    }
    pub fn test_f32_slice_ref_mut(_world: &mut World, _help: &mut [f32]) {
        todo!();
    }
    pub fn test_i64_slice_ref_mut(_world: &mut World, _help: &mut [i64]) {
        todo!();
    }
    pub fn test_i32_slice_ref_mut(_world: &mut World, _help: &mut [i32]) {
        todo!();
    }
    pub fn test_i16_slice_ref_mut(_world: &mut World, _help: &mut [i16]) {
        todo!();
    }
    pub fn test_i8_slice_ref_mut(_world: &mut World, _help: &mut [i8]) {
        todo!();
    }
    pub fn test_u64_slice_ref_mut(_world: &mut World, _help: &mut [u64]) {
        todo!();
    }
    pub fn test_u32_slice_ref_mut(_world: &mut World, _help: &mut [u32]) {
        todo!();
    }
    pub fn test_u16_slice_ref_mut(_world: &mut World, _help: &mut [u16]) {
        todo!();
    }
    pub fn test_u8_slice_ref_mut(_world: &mut World, _help: &mut [u8]) {
        todo!();
    }
}

pub fn mafn() {
}
