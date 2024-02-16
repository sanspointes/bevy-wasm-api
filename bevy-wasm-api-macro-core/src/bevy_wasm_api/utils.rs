use proc_macro2::Ident;
use syn::{FnArg, PatType, Type};

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

    if let Some(last_segment) = type_path.path.segments.last() {
        return match last_segment.ident.to_string().as_str() {
            "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "usize" | "f32"
            | "f64" => "number".to_string(),
            "String" | "str" => "string".to_string(),
            _ => "unknown".to_string(),
        }
    } else {
        "unknown".to_string()
    }
}
