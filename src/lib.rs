mod sync;

use wasm_bindgen::convert::FromWasmAbi;
pub use wasm_bindgen_futures::future_to_promise;
pub use js_sys::Promise;
pub use js_sys::Error;
pub use wasm_bindgen::JsValue;

pub use bevy_wasm_api_macro::bevy_wasm_api;
pub use sync::execute_in_world;
pub use sync::ExecutionChannel;
