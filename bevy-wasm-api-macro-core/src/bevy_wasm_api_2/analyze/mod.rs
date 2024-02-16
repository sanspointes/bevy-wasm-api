mod item_fn;
mod utils;

use proc_macro2::Ident;
use syn::spanned::Spanned;

use crate::bevy_wasm_api_2::parse::Ast;

use self::item_fn::{analyze_impl_item_fn, ImplItemFnModel};

#[derive(Debug)]
pub struct Model {
    struct_name: Ident,
    methods: Vec<ImplItemFnModel>,
}

impl Model {
    pub fn new(struct_name: Ident) -> Self {
        Self {
            struct_name,
            methods: vec![],
        }
    }
}

pub fn analyze(ast: Ast) -> syn::Result<Model> {
    let struct_name = match *ast.self_ty {
        syn::Type::Path(ref path) => path.path.get_ident(),
        _ => None,
    };
    let struct_name = struct_name.ok_or(syn::Error::new(
        ast.self_ty.span(),
        "Impl block must have a name".to_string(),
    ))?;

    let mut model = Model::new(struct_name.clone());

    for item in ast.items {
        if let syn::ImplItem::Fn(impl_item_fn) = item {
            let impl_item_fn_model = analyze_impl_item_fn(&impl_item_fn)?;
            model.methods.push(impl_item_fn_model);
        }
    }

    Ok(model)
}
