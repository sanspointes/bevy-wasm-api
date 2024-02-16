use proc_macro2::{TokenStream, Ident, Span};
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, FnArg, token::Comma, Error};

use crate::{analyze::Model, bevy_wasm_api::{analyze::{MethodModel, ApiReturnType}, utils::{get_ident_of_fn_arg, get_ts_type_of_fn_arg, generate_return_type_tokens}}};

pub fn codegen(model: Model) -> TokenStream {
    let Model {
        original_tokens,
        struct_name,
        method_definitions,
    } = model;

    // Build the typescript method definitions for extra ts types
    let mut ts_method_definitions = String::new();
    for def in &method_definitions {
        ts_method_definitions += &def.js_method_definiton;
    }
    let wasm_class_def = format!(
        "\nexport class {} {{\n\tfree();\n\n{}}}",
        struct_name, ts_method_definitions
    );

    let mut rs_method_definitions = vec![];
    for def in method_definitions {
        let MethodModel { method_name, js_method_name: _, js_method_definiton: _, api_return_type, world_ident, remaining_inputs } = def;

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

        // Handle the conversion of the return type into a jsvalue, here we use IntoWasmAbi to
        // create the JsValues.
        let return_tokens = generate_return_type_tokens(Ident::new("ret_val", Span::call_site()), api_return_type);

        rs_method_definitions.push(quote! {
            #[wasm_bindgen(skip_typescript)]
            pub fn #wasm_method_name(#(#input_method_args),*) -> bevy_wasm_api::Promise {
                bevy_wasm_api::future_to_promise(bevy_wasm_api::execute_in_world(bevy_wasm_api::ExecutionChannel::FrameStart, move |#world_ident|{
                    let ret_val = #struct_name::#method_name(#original_method_args);
                    #return_tokens
                }))
            }
        });
    }

    let ts_result = quote! {
        #original_tokens

        #[wasm_bindgen(typescript_custom_section)]
        const TS_APPEND_CONTENT: &'static str = #wasm_class_def;

        #[wasm_bindgen(skip_typescript)]
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
