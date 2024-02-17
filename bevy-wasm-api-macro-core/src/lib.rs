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

    let tokens = bevy_wasm_api_2::codegen::codegen(model);
    #[cfg(feature = "debug")]
    {
        let file = syn::parse_file(&tokens.to_string());
        match file {
            Ok(string) => {
                let formatted = prettyplease::unparse(&string);
                println!("bevy_wasm_api: Output {}", formatted);
            }
            Err(reason) => {
                println!("bevy_wasm_api: Could not parse output.  This should probably never happen. \nreason: {reason:?}");
            }
        }
    }
    tokens
}
