use proc_macro2::TokenStream;

mod bevy_wasm_api;
use crate::bevy_wasm_api::analyze;
use crate::bevy_wasm_api::codegen;

pub fn bevy_wasm_api(_attr: TokenStream, ts: TokenStream) -> TokenStream {
    let model = analyze::analyze(ts);
    let model = match model {
        Ok(model) => model,
        Err(v) => return v.to_compile_error(),
    };
    codegen::codegen(model)
}
