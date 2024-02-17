mod sync;

pub mod reexports {
    pub use wasm_bindgen;
    pub use wasm_bindgen_futures;
    pub use js_sys;
    pub use serde_wasm_bindgen;
}

pub use bevy_wasm_api_macro::bevy_wasm_api;
pub use sync::execute_in_world;
pub use sync::ExecutionChannel;
use sync::execute_world_tasks_begin;
use sync::execute_world_tasks_end;

/// BevyWasmApiPlugin will execute the queued tasks from the bevy_wasm_api
pub struct BevyWasmApiPlugin;
impl bevy_app::Plugin for BevyWasmApiPlugin {
    fn name(&self) -> &str {
        "BevyWasmApi"
    }
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(bevy_app::First, execute_world_tasks_begin)
            .add_systems(bevy_app::Last, execute_world_tasks_end);
    }
}
