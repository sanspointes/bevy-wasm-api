use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn bevy_wasm_api(_attr: TokenStream, ts: TokenStream) -> TokenStream {
    bevy_wasm_api_macro_core::bevy_wasm_api_2(_attr.into(), ts.into()).into()
}
