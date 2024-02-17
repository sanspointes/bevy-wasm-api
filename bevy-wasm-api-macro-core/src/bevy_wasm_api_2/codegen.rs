use proc_macro2::{TokenStream, Ident, Span};
use quote::quote;

use super::analyze::{Model, utils::TypescriptType};

pub fn build_ret_val_tokens(ident: Ident, ts_type: &TypescriptType) -> TokenStream {
    match ts_type {
        TypescriptType::Void => quote!{ Ok(bevy_wasm_api::JsValue::UNDEFINED) },
        TypescriptType::Number => quote!{ Ok(bevy_wasm_api::JsValue::from(#ident)) },
        TypescriptType::String => quote!{ Ok(bevy_wasm_api::JsValue::from(#ident)) },
        TypescriptType::Boolean => quote!{ Ok(bevy_wasm_api::JsValue::from(#ident)) },
        TypescriptType::Class(class_name) => {
            let class_ident = Ident::new(class_name, Span::call_site());

            todo!("{}", class_name)
        }
        TypescriptType::Promise(inner) => {
            let ok_tokens = build_ret_val_tokens(Ident::new("inner", Span::call_site()), inner);

            quote!{
                match #ident {
                    Ok(inner) => #ok_tokens,
                    Err(reason) => {
                        let error = bevy_wasm_api::Error::new(format!("{reason}").as_str());
                        Err(JsValue::from(error))
                    },
                }
            }
        }
    }
}

pub fn codegen(model: Model) -> TokenStream {
    // Define typescript
    let mut ts_method_definitions = String::new();
    for method in &model.methods {
        let mut s = format!("\t{}(", method.js_method_ident());

        for arg in &method.typescript_arguments {
            s += format!("{}", arg).as_str();
        }
        s += format!("): {};\n", method.typescript_return_type.wrapped_with_promise()).as_str();
        ts_method_definitions += &s;
    }
    let ts_class_def = format!("\nexport class {} {{\n{}}}\n", model.struct_name, ts_method_definitions);

    // Build the wasm api impl
    let struct_name = model.struct_name;
    let mut wasm_method_defs = vec![];
    for method in &model.methods {
        let original_method_ident = &method.original_method_ident;
        let world_ident = &method.api_method_args.world_ident;
        let wasm_method_ident = method.js_method_ident();
        let api_args_def = method.api_method_args.api_args_definition_token_stream();
        let original_call_args = method.api_method_args.original_method_call_args_token_stream();

        let ret_val_handler = build_ret_val_tokens(Ident::new("ret_val", Span::call_site()), &method.typescript_return_type);

        wasm_method_defs.push(quote!{
            #[wasm_bindgen(skip_typescript)]
            pub fn #wasm_method_ident(#api_args_def) -> bevy_wasm_api::Promise {
                bevy_wasm_api::future_to_promise(bevy_wasm_api::execute_in_world(bevy_wasm_api::ExecutionChannel::FrameStart, move |#world_ident| {
                    let ret_val = #struct_name::#original_method_ident(#original_call_args);
                    #ret_val_handler
                }))
            }
        })
    }

    let original = &model.original_ast;
    quote! {
        #original

        #[wasm_bindgen(typescript_custom_section)]
        const TS_APPEND_CONTENT: &'static str = #ts_class_def;

        #[wasm_bindgen(skip_typescript)]
        impl #struct_name {
            #( #wasm_method_defs )*
        }
    }
}
