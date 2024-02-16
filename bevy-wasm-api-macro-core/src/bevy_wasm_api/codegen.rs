use proc_macro2::{TokenStream, Ident, Span};
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, FnArg, token::Comma};

use crate::{analyze::Model, bevy_wasm_api::{analyze::MethodModel, utils::{get_ident_of_fn_arg, get_ts_type_of_fn_arg}}};

pub fn codegen(model: Model) -> TokenStream {
    let Model {
        original_tokens,
        struct_name,
        method_definitions,
    } = model;

    let mut ts_method_definitions = String::new();

    println!("method_definitions: {method_definitions:?}");
    for def in &method_definitions {
        let ts_return_type = match &def.method_output {
            syn::ReturnType::Type(_, ty) => {
                format!("Promise<{}>", ty.into_token_stream())
            }
            syn::ReturnType::Default => "Promise<void>".to_string(),
        };
        let mut args = String::new();
        for (i, arg) in def.remaining_inputs.iter().enumerate() {
            let ident = get_ident_of_fn_arg(arg).unwrap();
            let ts_type = get_ts_type_of_fn_arg(arg);
            args += &format!("{}: {}", ident, ts_type);
            if i < def.remaining_inputs.len() {
                args += ", "
            }
        }

        ts_method_definitions += &format!("{}_wasm({args}) {};\n", def.method_name, ts_return_type);
    }

    let wasm_class_def = format!(
        "\nexport class {} {{\n{} }}",
        struct_name, ts_method_definitions
    );

    let mut rs_method_definitions = vec![];
    for def in method_definitions {
        let MethodModel { method_name, method_output, world_ident, remaining_inputs } = def;

        // Seperate world call (injected via execute_in_world) from rest of args (provided by js)
        let world_arg_ident = get_ident_of_fn_arg(&world_ident).unwrap();
        let remaining_arg_idents = remaining_inputs.iter().map(|arg| {
            get_ident_of_fn_arg(arg).unwrap()
        });

        // Derive a new function called "{{function_name}}_wasm"
        // This function will be exposed via wasm bindgen
        let wasm_method_name = format!("{}_wasm", method_name);
        let wasm_method_name = Ident::new(&wasm_method_name, Span::call_site());

        // Build the input arguments for wasm bindgen, basicall all args excluding
        // the world: &mut World (injected by execute_in_world)
        let mut input_method_args: Vec<TokenStream> = vec![];
        input_method_args.push(quote!{ &self });
        input_method_args.extend(remaining_inputs.iter().map(|fn_arg| {
            quote!{ #fn_arg }
        }));
        println!("\ninput_method_args: {:?}\n", input_method_args);

        // Build the token stream for the params passed to the real method.
        // This is where we use the injected world from execute_in_world
        let mut original_method_args = Punctuated::<Ident, Comma>::new();
        original_method_args.push(world_arg_ident.clone());
        for arg_ident in remaining_arg_idents {
            original_method_args.push(arg_ident.clone());
        }

        let return_tokens = match method_output {
            syn::ReturnType::Type(_, ty) => {
                todo!();
            }
            syn::ReturnType::Default => quote!{ Ok(bevy_wasm_api::JsValue::UNDEFINED) },
        };

        rs_method_definitions.push(quote! {
            #[wasm_bindgen()]
            pub fn #wasm_method_name(#(#input_method_args),*) -> bevy_wasm_api::Promise {
                bevy_wasm_api::future_to_promise(bevy_wasm_api::execute_in_world(bevy_wasm_api::ExecutionChannel::FrameStart, move |#world_ident|{
                    let response = #struct_name::#method_name(#original_method_args);
                    #return_tokens
                }))
            }
        });
    }

    let ts_result = quote! {
        #original_tokens

        #[wasm_bindgen(typescript_custom_section)]
        const TS_APPEND_CONTENT: &'static str = #wasm_class_def;

        #[wasm_bindgen]
        impl #struct_name {
            #( #rs_method_definitions )*
        }
    };
    println!("Result: {}", ts_result);
    ts_result
}
//
// mod tests {
//     use syn::{Token, Type, TypePath};
//
//     use crate::bevy_wasm_api::analyze::{Model, MethodModel};
//
//     pub fn generates_ts_methods_correctly() {
//         let model = Model {
//             struct_name: "MyApi",
//             method_definitions: vec![MethodModel {
//                 method_output: syn::ReturnType::Type(syn::token::RArrow { spans: () }, Box::new(Type::Path(TypePath)))
//             }],
//         };
//     }
// }
