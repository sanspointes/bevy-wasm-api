use proc_macro2::Ident;
use syn::{ImplItemFn, FnArg, Error, Result};

use crate::bevy_wasm_api_2::analyze::utils::TypescriptArg;

use super::utils::{TypescriptType, extract_first_method_argument};

#[derive(Debug)]
pub struct ImplItemFnModel {
    pub original_method_ident: Ident,
    pub typescript_arguments: Vec<TypescriptArg>,
    pub typescript_return_type: TypescriptType,
}

pub fn analyze_impl_item_fn(method: &ImplItemFn) -> syn::Result<ImplItemFnModel> {
    let method_name = &method.sig.ident;
    let method_inputs = &method.sig.inputs;
    let method_output = &method.sig.output;

    let (first, remaining) = extract_first_method_argument(method_inputs)?;

    // First check function validatiy
    if let FnArg::Receiver(_) = &first {
        return Err(Error::new_spanned(first.clone(), "Can't use receiver arguments.  First argument must be `arg1: &mut World`."))
    }

    let FnArg::Typed(ref first_typed) = first else {
        return Err(Error::new_spanned(first.clone(), "First argument in function is not typed. First argument must be `arg1: &mut World`. Instead found {:?}."));
    };

    if let syn::Type::Reference(reference) = &*first_typed.ty {
        if reference.mutability.is_none() {
            return Err(Error::new_spanned(first, "First argument in function is not a mutable reference.  First argument must be `arg1: &mut World`."));
        }
    } else {
        return Err(Error::new_spanned(first, "First argument in function is not a reference.  First argument must be `arg1: &mut World`."));
    }

    let typescript_arguments = remaining.iter().map(|fn_arg| {
        TypescriptArg::try_from(fn_arg)
    }).collect::<Result<Vec<TypescriptArg>>>()?;

    Ok(ImplItemFnModel {
        original_method_ident: method_name.clone(),
        typescript_arguments,
        typescript_return_type: TypescriptType::try_from(method_output)?,
    })
}
