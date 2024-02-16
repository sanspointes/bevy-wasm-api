use proc_macro2::TokenStream;

use crate::bevy_wasm_api_2::lower::Ir;

pub fn codegen(_ir: Ir) -> TokenStream {
    TokenStream::new()
}
