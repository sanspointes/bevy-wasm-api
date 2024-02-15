use proc_macro2::TokenStream;

mod bevy_wasm_api;
pub use crate::bevy_wasm_api::analyze;
use crate::bevy_wasm_api::codegen;
use crate::bevy_wasm_api::lower;

pub fn bevy_wasm_api(_attr: TokenStream, ts: TokenStream) -> TokenStream {
    let model = analyze::analyze(ts.into());
    let ir = lower::lower(model);
    let _ = codegen::codegen(ir);
    TokenStream::new()
}
