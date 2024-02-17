use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use super::analyze::{utils::TypescriptType, Model};

/// Contains a token stream that converts an ident into a JsValue
/// Sometimes it will already be wrapped in an Ok() and sometimes not.
/// You can make it always wrapped in an OK() by using as_result_tokens.
///
/// * `is_result`: Whether or not the tokens evaluate to an Ok(T)
/// * `tokens`: The token stream
pub struct IntoJsTokens {
    is_result: bool,
    tokens: TokenStream,
}
impl IntoJsTokens {
    pub fn new(tokens: TokenStream) -> Self {
        Self {
            is_result: false,
            tokens,
        }
    }
    pub fn new_as_result(tokens: TokenStream) -> Self {
        Self {
            is_result: true,
            tokens,
        }
    }

    pub fn as_result_tokens(&self) -> TokenStream {
        let self_tokens = &self.tokens;
        if !self.is_result {
            quote! { Ok(#self_tokens)}
        } else {
            self_tokens.clone()
        }
    }
}

impl TypescriptType {
    pub fn generate_into_js_tokens(&self, ident: Ident) -> IntoJsTokens {
        match self {
            TypescriptType::Void => IntoJsTokens::new(quote! { wasm_bindgen::JsValue::UNDEFINED }),
            TypescriptType::Number => {
                IntoJsTokens::new(quote! { wasm_bindgen::JsValue::from(#ident) })
            }
            TypescriptType::String => {
                IntoJsTokens::new(quote! { wasm_bindgen::JsValue::from(#ident) })
            }
            TypescriptType::Boolean => {
                IntoJsTokens::new(quote! { wasm_bindgen::JsValue::from(#ident) })
            }
            TypescriptType::Struct(_struct_name) => IntoJsTokens::new_as_result(quote! {
                match serde_wasm_bindgen::to_value(&#ident) {
                    Ok(js_value) => Ok(js_value),
                    Err(reason) => {
                        let error = js_sys::Error::new(format!("{reason}").as_str());
                        Err(wasm_bindgen::JsValue::from(error))
                    }
                }
            }),
            TypescriptType::Promise(inner) => {
                let ok_tokens = inner
                    .generate_into_js_tokens(Ident::new("inner", Span::call_site()))
                    .as_result_tokens();

                IntoJsTokens::new_as_result(quote! {
                    match #ident {
                        Ok(inner) => #ok_tokens,
                        Err(reason) => {
                            let error = js_sys::Error::new(format!("{reason}").as_str());
                            Err(wasm_bindgen::JsValue::from(error))
                        },
                    }
                })
            }
            TypescriptType::Array(inner) => {
                let to_js_tokens = inner
                    .generate_into_js_tokens(Ident::new("value", Span::call_site()));
                let to_js_token_stream = to_js_tokens.tokens;

                if to_js_tokens.is_result {
                    IntoJsTokens::new_as_result(quote!{
                        Ok(bevy_wasm_api::convert::vec_to_js_value(
                            #ident.into_iter().map(|value| #to_js_token_stream).collect::<Result<Vec<_>, wasm_bindgen::JsValue>>()?
                        ))
                    })
                } else {
                    IntoJsTokens::new_as_result(quote!{
                        Ok(bevy_wasm_api::convert::vec_to_js_value(
                            #ident.into_iter().map(|value| #to_js_token_stream).collect::<Vec<_>>()
                        ))
                    })
                }
            }
            TypescriptType::Option(inner) => {
                let value_to_js_value_tokens = inner
                    .generate_into_js_tokens(Ident::new("inner", Span::call_site()))
                    .as_result_tokens();

                IntoJsTokens::new_as_result(quote! {
                    match #ident {
                        Some(inner) => #value_to_js_value_tokens,
                        None => Ok(wasm_bindgen::JsValue::UNDEFINED),
                    }
                })
            }
        }
    }
}

pub fn codegen(model: Model) -> TokenStream {
    // Define typescript
    let mut ts_method_definitions = String::new();
    for method in &model.methods {
        let mut s = format!("\t{}(", &method.original_method_ident);

        for (i, arg) in method.typescript_arguments.iter().enumerate() {
            s += format!("{}", arg).as_str();
            if i < method.typescript_arguments.len() - 1 {
                s += ", ";
            }
        }
        s += format!(
            "): {};\n",
            method.typescript_return_type.wrapped_with_promise()
        )
        .as_str();
        ts_method_definitions += &s;
    }
    let ts_class_def = format!(
        "\nexport class {} {{\n\tconstructor();\n{}\tfree(): void;\n}}\n",
        model.struct_name, ts_method_definitions
    );

    // Build the wasm api impl
    let struct_name = &model.struct_name;
    let mut wasm_method_defs = vec![];
    for method in &model.methods {
        let original_method_ident = &method.original_method_ident;
        let world_ident = &method.api_method_args.world_ident;
        let api_args_def = method.api_method_args.api_args_definition_token_stream();
        let original_call_args = method
            .api_method_args
            .original_method_call_args_token_stream();

        let ret_val_tokens = &method
            .typescript_return_type
            .generate_into_js_tokens(Ident::new("ret_val", Span::call_site()))
            .as_result_tokens();

        wasm_method_defs.push(quote!{
            #[wasm_bindgen(skip_typescript)]
            pub fn #original_method_ident(#api_args_def) -> bevy_wasm_api::reexports::js_sys::Promise {
                use bevy_wasm_api::reexports::*;
                wasm_bindgen_futures::future_to_promise(bevy_wasm_api::execute_in_world(bevy_wasm_api::ExecutionChannel::FrameStart, move |#world_ident| {
                    let ret_val = #struct_name::#original_method_ident(#original_call_args);
                    #ret_val_tokens
                }))
            }
        })
    }

    let original = &model.original_ast;
    let js_struct_name = Ident::new(&format!("{}WasmApi", model.struct_name), Span::call_site());
    let struct_name_string = struct_name.to_string();
    quote! {
        #original

        #[wasm_bindgen(typescript_custom_section)]
        const TS_APPEND_CONTENT: &'static str = #ts_class_def;

        #[wasm_bindgen(js_name = #struct_name_string, skip_typescript)]
        struct #js_struct_name;

        #[wasm_bindgen(js_class = #struct_name_string, skip_typescript)]
        impl #js_struct_name {
            #[wasm_bindgen(constructor)]
            pub fn new() -> Self {
                Self
            }

            #( #wasm_method_defs )*
        }
    }
}
