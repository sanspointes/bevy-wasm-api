use proc_macro2::TokenStream;
use quote::quote;

use super::analyze::Model;

pub fn codegen(model: Model) -> TokenStream {
    // Define typescript
    let mut ts_method_definitions = String::new();
    for method in &model.methods {
        let mut s = format!("\t{}_js(", method.original_method_ident);

        for arg in &method.typescript_arguments {
            s += format!("{}", arg).as_str();
        }
        s += format!("): {};\n", method.typescript_return_type.wrapped_with_promise()).as_str();
        ts_method_definitions += &s;
    }
    let ts_class_def = format!("\nexport class {} {{\n{}}}\n", model.struct_name, ts_method_definitions);

    quote! {
        #[wasm_bindgen(typescript_custom_section)]
        const TS_APPEND_CONTENT: &'static str = #ts_class_def;
    }
}
