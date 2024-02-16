use proc_macro2::{Ident, TokenStream, Span};
use quote::quote;
use syn::{token::Comma, FnArg, GenericArgument, PatType, PathSegment, Type};

use super::analyze::ApiReturnType;

pub fn get_ident_of_fn_arg(arg: &FnArg) -> Option<&Ident> {
    match arg {
        syn::FnArg::Receiver(_) => None, // Skip `self` receiver
        syn::FnArg::Typed(pat_type) => {
            let pat = &*pat_type.pat;
            if let syn::Pat::Ident(pat_ident) = pat {
                Some(&pat_ident.ident)
            } else {
                None
            }
        }
    }
}

pub fn get_ts_type_of_fn_arg(arg: &FnArg) -> String {
    let type_path = match arg {
        syn::FnArg::Receiver(_) => None,
        syn::FnArg::Typed(PatType { ty, .. }) => match **ty {
            Type::Path(ref type_path) => Some(type_path),
            Type::Reference(ref type_reference) => {
                if let Type::Path(ref type_path) = *type_reference.elem {
                    Some(type_path)
                } else {
                    None
                }
            }
            _ => None,
        },
    };

    let Some(type_path) = type_path else {
        return "unknown".to_string();
    };
    let last_segment = type_path.path.segments.last().unwrap();

    return match last_segment.ident.to_string().as_str() {
        "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "usize" | "f32"
        | "f64" => "number".to_string(),
        "String" | "str" => "string".to_string(),
        _ => "unknown".to_string(),
    };
}

impl TryFrom<&PathSegment> for ApiReturnType {
    type Error = syn::Error;
    fn try_from(value: &PathSegment) -> Result<Self, Self::Error> {
        match value.ident.to_string().as_str() {
            "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "usize" | "f32"
            | "f64" => Ok(ApiReturnType::Primitive),
            "bool" => Ok(ApiReturnType::Primitive),

            "String" | "str" => Ok(ApiReturnType::Primitive),

            "Result" => {
                let args = match &value.arguments {
                    syn::PathArguments::AngleBracketed(args) => args,
                    _ => panic!("Malformed result."),
                };
                assert_eq!(args.args.len(), 2);
                match (&args.args[0], &args.args[1]) {
                    (
                        GenericArgument::Type(Type::Path(ok_type_path)),
                        GenericArgument::Type(Type::Path(err_type_path)),
                    ) => {
                        let ok_api_ret_type =
                            ApiReturnType::try_from(ok_type_path.path.segments.last().unwrap())?;
                        let err_api_ret_type =
                            ApiReturnType::try_from(err_type_path.path.segments.last().unwrap())?;
                        Ok(ApiReturnType::Result(
                            Box::new(ok_api_ret_type),
                            Box::new(err_api_ret_type),
                        ))
                    }
                    _ => panic!("Malformed result"),
                }
            }

            _ => Ok(ApiReturnType::FromWasmAbi),
            // ident => todo!("{ident} not yet implemented."),
        }
    }
}

pub fn generate_return_type_tokens(var: Ident, api_return_type: ApiReturnType) -> TokenStream {
    match api_return_type {
        ApiReturnType::Void => quote!{ Ok(bevy_wasm_api::JsValue::UNDEFINED) },
        ApiReturnType::Primitive => quote!{ Ok(JsValue::from(#var)) },
        ApiReturnType::FromWasmAbi => todo!("generate_return_type_tokens: FromWasmAbi"),
        ApiReturnType::Array(_) => todo!("generate_return_type_tokens: Array"),
        ApiReturnType::Result(ok_type, err_type) => {
            let handle_ok_tokens = generate_return_type_tokens(Ident::new("val", Span::call_site()), *ok_type);
            let handle_err_tokens = generate_return_type_tokens(Ident::new("err", Span::call_site()), *err_type);

            quote!{
                match #var {
                    Ok(val) => #handle_ok_tokens,
                    Err(err) => #handle_err_tokens,
                }
            }
        }
    }
}
