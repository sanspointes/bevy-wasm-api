use proc_macro2::{Ident, TokenStream};
use syn::{ItemImpl, Error, punctuated::Punctuated, FnArg, token::Comma, ReturnType};

#[derive(Debug)]
pub struct MethodModel {
    pub method_name: Ident,
    pub method_output: ReturnType,
    pub world_ident: FnArg,
    pub remaining_inputs: Vec<FnArg>,
}

#[derive(Debug)]
pub struct Model {
    pub struct_name: Ident,
    pub method_definitions: Vec<MethodModel>,
}

fn extract_first_method_argument(args: &Punctuated<FnArg, Comma>) -> syn::Result<(FnArg, Vec<FnArg>)> {
    let mut iter = args.iter();
    let mut first = iter.next();

    println!("Args length: {}", args.len());

    // Skip the self receiver provided it exists.
    if let Some(f) = first {
        if matches!(f, FnArg::Receiver(_)) {
            first = iter.next();
        }
    }

    if first.is_none() {
        return Err(Error::new_spanned(args, "Missing first argument on function.  First argument must be of type &mut World."));
    }

    let remaining: Vec<_> = iter.cloned().collect();
    Ok((first.unwrap().clone(), remaining))
}

pub fn analyze(ts: TokenStream) -> syn::Result<Model> {
    let ast = syn::parse2::<ItemImpl>(ts).unwrap();
    let struct_name = match *ast.self_ty {
        syn::Type::Path(ref path) => path.path.get_ident().expect("Expected identifier"),
        _ => panic!("Expected type path"),
    };


    let mut method_definitions: Vec<MethodModel> = vec![];

    for item in ast.items {
        if let syn::ImplItem::Fn(method) = item {
            let method_name = &method.sig.ident;
            // let wasm_method_name = format!("{}_wasm", struct_name);
            // let method_vis = &method.vis;
            // let method_block = &method.block;
            let method_inputs = &method.sig.inputs;
            let method_output = &method.sig.output;

            let (first, remaining) = extract_first_method_argument(method_inputs)?;
            println!("\nFirst: {first:?}\nRemaining: {remaining:?}");

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

            // TODO: CHeck if first argument is bevy::prelude::World;

            method_definitions.push( MethodModel {
                method_name: method_name.clone(),
                method_output: method_output.clone(),
                world_ident: first,
                remaining_inputs: remaining,
            });
            //
            // //
            // // let method_inputs_iter = method_inputs.iter();
            //
            // let mut world_param_name = None;
            // if let Some(first_input) = method_inputs_iter.next() {
            //     // world_arg_name = first_input.
            //     if let syn::FnArg::Typed(pat_type) = first_input {
            //         let pat = &pat_type.pat;
            //         if let syn::Pat::Ident(pat_ident) = pat {
            //             world_param_name = Some(&pat_ident.ident);
            //         }
            //
            //         let first_param_type = &pat_type.ty;
            //         if !matches!(**first_param_type, syn::Type::Path(ref path) if path.path.is_ident("usize")) {
            //             return Error::new_spanned(&first_param_type, format!("First parameter must be bevy::prelude::World. Instead found {}", path)).to_compile_error().into();
            //         }
            //     }
            // }
            //
            // let remaining_inputs: Punctuated<FnArg, Comma> = method_inputs_iter.collect();
            //
            // let typescript_repr = format!("{}()",wasm_method_name);
            //
            // method_definitions.push((
            //     method_output.to_string(),
            //     quote! {
            //         #method_vis fn #wasm_method_name #remaining_inputs js_sys::Promise {
            //             future_to_promise(execute_in_world(|| {
            //                 #method_block
            //             }))
            //         }
            //     }
            // ));
        }
    }

    Ok(Model {
        struct_name: struct_name.clone(),
        method_definitions,
    })
}

mod tests {
    use quote::quote;
    use super::analyze;

    #[test]
    pub fn extracts_struct_name() {
        let ast = quote! {
            impl MyApi {
                pub fn my_fn(&self, world: &mut World, arg1: i32) -> String {
                    "Hello".to_string()
                }
            }
        };

        let model = analyze(ast).unwrap();

        assert_eq!(model.struct_name.to_string(), "MyApi");
    }

    #[test]
    pub fn checks_first_arg_is_ref_mut_world() {
        let ast = quote! {
            impl MyApi {
                pub fn my_fn(&self, arg1: i32) -> String {
                    "Hello".to_string()
                }
            }
        };
        let error = analyze(ast).unwrap_err();
        assert!(error.to_string().contains("First argument in function"), "Is letting a pass by value argument through.");

        let ast = quote! {
            impl MyApi {
                pub fn my_fn(&self, arg1: &i32) -> String {
                    "Hello".to_string()
                }
            }
        };
        let error = analyze(ast).unwrap_err();
        assert!(error.to_string().contains("First argument in function"), "Is letting a pass by refrence argument through.");

        let ast = quote! {
            impl MyApi {
                pub fn my_fn(&self, arg1: &mut i32) -> String {
                    "Hello".to_string()
                }
            }
        };
        let result = analyze(ast);
        assert_eq!(result.is_ok(), true);
    }
}
