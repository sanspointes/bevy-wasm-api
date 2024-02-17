use bevy_wasm_api_macro::bevy_wasm_api;

pub fn main() {
    todo!();
}

struct MyApi;
#[bevy_wasm_api]
impl MyApi {
    pub fn test(&self, help: i32) {
        todo!();
    }
}
