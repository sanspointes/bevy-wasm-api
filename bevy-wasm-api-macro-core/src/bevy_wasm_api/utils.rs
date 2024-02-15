use proc_macro2::Ident;
use syn::FnArg;

pub fn get_ident_of_fn_arg(arg: &FnArg) -> Option<&Ident> {
    let var_name = match arg {
        syn::FnArg::Receiver(_) => None, // Skip `self` receiver
        syn::FnArg::Typed(pat_type) => {
            let pat = &*pat_type.pat;
            if let syn::Pat::Ident(pat_ident) = pat {
                Some(&pat_ident.ident)
            } else {
                None
            }
        }
    };
    var_name
}
