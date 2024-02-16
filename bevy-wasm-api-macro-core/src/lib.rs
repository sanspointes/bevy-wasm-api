use proc_macro2::TokenStream;

mod bevy_wasm_api;
mod bevy_wasm_api_2;
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

pub fn bevy_wasm_api_2(
    _attrs: TokenStream,
    ts: TokenStream,
) -> TokenStream {
    let ast = match bevy_wasm_api_2::parse::parse(ts) {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error(),
    };
    let model = match bevy_wasm_api_2::analyze::analyze(ast) {
        Ok(model) => model,
        Err(err) => return err.to_compile_error(),
    };
    let ir = bevy_wasm_api_2::lower::lower(model);
    bevy_wasm_api_2::codegen::codegen(ir)
}
