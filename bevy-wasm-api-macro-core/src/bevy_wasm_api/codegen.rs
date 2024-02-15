use proc_macro2::{TokenStream, Group, Ident};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{punctuated::Punctuated, FnArg, token::Comma};

use crate::{analyze::Model, bevy_wasm_api::{analyze::MethodModel, utils::get_ident_of_fn_arg}};

pub fn codegen(model: Model) -> TokenStream {
    let Model {
        struct_name,
        method_definitions,
    } = model;

    let mut ts_method_definitions = String::new();

    println!("method_definitions: {method_definitions:?}");
    for def in &method_definitions {
        let ts_return_type = match &def.method_output {
            syn::ReturnType::Type(_, ty) => {
                format!("Promise<{}>", ty.into_token_stream().to_string())
            }
            syn::ReturnType::Default => "".to_string(),
        };
        ts_method_definitions += &format!("{}_wasm() {};\n", def.method_name, ts_return_type);
    }

    let wasm_class_def = format!(
        "\nexport class {} {{ {} }}",
        struct_name, ts_method_definitions
    );

    let mut rs_method_definitions = vec![];
    for def in method_definitions {
        let MethodModel { method_name, method_output, world_ident, remaining_inputs } = def;

        let mut input_definition = Punctuated::<FnArg, syn::token::Comma>::new();
        for a in &remaining_inputs {
            input_definition.push(a.clone());
        }

        let world_arg_ident = get_ident_of_fn_arg(&world_ident).unwrap();
        let remaining_arg_idents = remaining_inputs.iter().map(|arg| {
            get_ident_of_fn_arg(arg).unwrap()
        });

        let mut original_method_args = Punctuated::<Ident, Comma>::new();
        original_method_args.push(world_arg_ident.clone());
        for arg_ident in remaining_arg_idents {
            original_method_args.push(arg_ident.clone());
        }

        // args_ts.append(world_ident.match)
        let inner_args = quote!{
            self.call_fn(arg1, arg2)
        };
        println!("calling a method: #{inner_args:#?}");

        let wasm_method_name = format!("{}_wasm", method_name.to_string());
        rs_method_definitions.push(quote! {
            pub fn #wasm_method_name #input_definition -> js_sys::Promise {
                wasm_bindgen_futures::future_to_promise(bevy_wasm_api::execute_in_world(|#world_ident|{
                    let response = self.#method_name(#original_method_args);
                }))
            }
        });
    }

    let v = quote! {
        #[wasm_bindgen(typescript_custom_section)]
        const TS_APPEND_CONTENT: &'static str = #wasm_class_def;

        #[wasm_bindgen]
        impl #struct_name {
            #( #rs_method_definitions )*
        }
    }
    .into();

    v
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
