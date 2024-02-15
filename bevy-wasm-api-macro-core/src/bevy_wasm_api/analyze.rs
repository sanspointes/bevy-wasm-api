use proc_macro2::{Ident, TokenStream};
use syn::ItemImpl;

pub struct MethodModel {
    method_name: Ident,
}

pub struct Model {
    pub struct_name: Ident,
}

pub fn analyze(ts: TokenStream) -> Model {
    let ast = syn::parse2::<ItemImpl>(ts).unwrap();
    let struct_name = match *ast.self_ty {
        syn::Type::Path(ref path) => path.path.get_ident().expect("Expected identifier"),
        _ => panic!("Expected type path"),
    };

    Model {
        struct_name: struct_name.clone(),
    }
}

mod tests {
    use quote::quote;
    use super::analyze;

    #[test]
    pub fn extracts_struct_name() {
        let ast = quote! {
            impl MyApi {
                pub fn my_fn(&self, arg1: i32) -> String {
                    "Hello".to_string()
                }
            }
        };

        let model = analyze(ast);

        assert_eq!(model.struct_name.to_string(), "MyApi");
    }
}
