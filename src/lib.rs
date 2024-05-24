mod sync;
pub mod convert;

pub mod reexports {
    pub use wasm_bindgen;
    pub use wasm_bindgen_futures;
    pub use js_sys;
    pub use serde_wasm_bindgen;
}

use bevy_ecs::schedule::ScheduleLabel;
pub use bevy_wasm_api_macro::bevy_wasm_api;
pub use sync::execute_in_world;
pub use sync::ExecutionChannel;
pub use sync::execute_world_tasks;

/// BevyWasmApiPlugin will execute the queued tasks from the bevy_wasm_api
#[derive(Debug)]
pub struct BevyWasmApiPlugin {
    schedule_label: Box<dyn ScheduleLabel>,
}

impl Default for BevyWasmApiPlugin {
    fn default() -> Self {
        Self {
            schedule_label: Box::new(bevy_app::PreUpdate),
        }
    }
}

impl BevyWasmApiPlugin {
    pub fn with_schedule_label(mut self, schedule_label: impl ScheduleLabel) -> Self {
        self.schedule_label = Box::new(schedule_label);
        self
    }
}

impl bevy_app::Plugin for BevyWasmApiPlugin {
    fn name(&self) -> &str {
        "BevyWasmApi"
    }
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(bevy_app::Last, execute_world_tasks);
    }
}
