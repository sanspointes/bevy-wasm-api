//! Extra utilities to help the proc_macro when converting from T into a JsValue
//! Required because the conversion has to be performed in one big (sometimes massive) 
//! expression to avoid the complexity of adding variables / closures.

use wasm_bindgen::JsValue;

pub fn vec_to_js_value(vec: Vec<JsValue>) -> wasm_bindgen::JsValue  {
    let array = js_sys::Array::new_with_length(vec.len().try_into().unwrap());
    for (i, item) in vec.into_iter().enumerate() {
        array.set(i.try_into().unwrap(), item);
    }
    JsValue::from(array)
}

pub struct JsArrayBuilder {
    array: js_sys::Array,
}

impl JsArrayBuilder {
    pub fn new() -> Self {
        Self { array: js_sys::Array::new() }
    }
    pub fn with_js_value(self, value: &wasm_bindgen::JsValue) -> Self {
        self.array.push(&value);
        self
    }

    pub fn build(self) -> js_sys::Array {
        self.array
    }
    pub fn build_as_js_value(self) -> wasm_bindgen::JsValue {
        JsValue::from(self.array)
    }
}
