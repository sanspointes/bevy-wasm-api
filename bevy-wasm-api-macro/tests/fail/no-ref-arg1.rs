use bevy_wasm_api_macro::bevy_wasm_api;

struct MyApi;

#[bevy_wasm_api]
impl MyApi {
    pub fn test(&self, help: i32) -> String {
        "Test yes!".to_string();
    }
}
