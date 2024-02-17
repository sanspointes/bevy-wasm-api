//! Extra utilities to help the proc_macro when converting from T into a JsValue

use wasm_bindgen::JsValue;

pub fn vec_to_js_value(vec: Vec<JsValue>) -> wasm_bindgen::JsValue  {
    let array = js_sys::Array::new_with_length(vec.len().try_into().unwrap());
    for (i, item) in vec.into_iter().enumerate() {
        array.set(i.try_into().unwrap(), item);
    }
    JsValue::from(array)
}
