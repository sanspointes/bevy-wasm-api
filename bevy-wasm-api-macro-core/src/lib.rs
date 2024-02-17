mod parse;
mod analyze;
mod codegen;
use proc_macro2::TokenStream;

pub fn bevy_wasm_api(
    _attrs: TokenStream,
    ts: TokenStream,
) -> TokenStream {
    #[cfg(feature = "debug")]
    {
        let file = syn::parse_file(&ts.to_string());
        match file {
            Ok(string) => {
                let formatted = prettyplease::unparse(&string);
                println!("\nSTART bevy_wasm_api input:\n{}\nEND bevy_wasm_api output\n", formatted);
            }
            Err(reason) => {
                println!("bevy_wasm_api: Could not parse input.  This should probably never happen. \nreason: {reason:?}");
            }
        }
        let file = syn::parse_file(&_attrs.to_string());
        match file {
            Ok(string) => {
                let formatted = prettyplease::unparse(&string);
                println!("bevy_wasm_api input attributes: {}", formatted);
            }
            Err(reason) => {
                println!("bevy_wasm_api: Could not parse attributes.  This should probably never happen. \nreason: {reason:?}");
            }
        }
    }
    let ast = match parse::parse(ts) {
        Ok(ast) => ast,
        Err(err) => return err.to_compile_error(),
    };
    let model = match analyze::analyze(ast) {
        Ok(model) => model,
        Err(err) => return err.to_compile_error(),
    };

    let tokens = codegen::codegen(model);
    #[cfg(feature = "debug")]
    {
        let file = syn::parse_file(&tokens.to_string());
        match file {
            Ok(string) => {
                let formatted = prettyplease::unparse(&string);
                println!("\nSTART bevy_wasm_api output:\n{}\nEND bevy_wasm_api output\n", formatted);
            }
            Err(reason) => {
                println!("bevy_wasm_api: Could not parse output.  This should probably never happen.\nreason: {reason:?}");
            }
        }
    }
    tokens
}
