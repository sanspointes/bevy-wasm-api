use std::fmt::Display;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Error, FnArg, GenericArgument, Pat,
    ReturnType, Type, TypePath,
};

#[derive(Debug)]
pub struct TypescriptArg {
    pub ident: Ident,
    pub ty: TypescriptType,
}

impl Display for TypescriptArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.ident, self.ty)
    }
}

impl TryFrom<&FnArg> for TypescriptArg {
    type Error = syn::Error;
    fn try_from(value: &FnArg) -> Result<Self, Self::Error> {
        match value {
            FnArg::Receiver(receiver) => Err(Error::new(
                receiver.span(),
                "Cannot create a typescript type from a receiver argument.",
            )),
            FnArg::Typed(pat_ty) => {
                let ty = match *pat_ty.ty {
                    syn::Type::Path(ref path) => TypescriptType::try_from(path)?,
                    ref unknown => return Err(Error::new(unknown.span(), format!("Cannot create typescript arg from typed argument `{unknown:?}`.  Unexpected argument type."))),
                };
                let ident = match *pat_ty.pat {
                    syn::Pat::Ident(ref ident_pat) => ident_pat.ident.clone(),
                    ref unknown => return Err(Error::new(unknown.span(), format!("Cannot create typescript arg from typed argument `{unknown:?}`.  Unexpected argument identifier."))),
                };

                Ok(TypescriptArg { ident, ty })
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum TypescriptType {
    Void,
    String,
    Number,
    Boolean,
    Struct(String),
    Promise(Box<TypescriptType>),
    Array(Box<TypescriptType>),
    // Tuple(Vec<TypescriptType>),
}

impl TypescriptType {
    pub fn wrapped_with_promise(&self) -> Self {
        match self {
            TypescriptType::Promise(inner) => TypescriptType::Promise(inner.clone()),
            other => TypescriptType::Promise(Box::new(other.clone())),
        }
    }
}

impl Display for TypescriptType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypescriptType::Void => write!(f, "void"),
            TypescriptType::String => write!(f, "string"),
            TypescriptType::Number => write!(f, "number"),
            TypescriptType::Boolean => write!(f, "boolean"),
            TypescriptType::Struct(struct_name) => write!(f, "{}", struct_name),
            TypescriptType::Promise(inner) => write!(f, "Promise<{}>", *inner),
            TypescriptType::Array(inner) => write!(f, "{}[]", *inner),
        }
    }
}

impl TryFrom<&TypePath> for TypescriptType {
    type Error = syn::Error;
    fn try_from(value: &TypePath) -> Result<Self, Self::Error> {
        let last_segment = value.path.segments.last().ok_or(Error::new(
            value.span(),
            "Cannot create a typescript type from type. No segments in type path.",
        ))?;
        match last_segment.ident.to_string().as_str() {
            "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "usize" | "f32" => {
                Ok(TypescriptType::Number)
            }
            "bool" => Ok(TypescriptType::Boolean),
            "String" | "str" => Ok(TypescriptType::String),
            "Result" => {
                let args = match &last_segment.arguments {
                    syn::PathArguments::AngleBracketed(args) => args,
                    ref unknown => {
                        return Err(Error::new(
                            last_segment.arguments.span(),
                            format!("Cannot create a typescript type from Result.  Unexpected arguments type: {unknown:?}."),
                        ))
                    }
                };
                let ok_type_path = match &args.args[0] {
                    GenericArgument::Type(Type::Path(ok_type_path)) => ok_type_path,
                    ref unknown => {
                        return Err(Error::new(
                            args.args[0].span(),
                            format!("Cannot create a typescript type from Result.  No type in result arguments: {unknown:?}."),
                        ))
                    }
                };

                let ts_type = TypescriptType::try_from(ok_type_path)?;

                Ok(TypescriptType::Promise(Box::new(ts_type)))
            },
            "Vec" => {
                let args = match &last_segment.arguments {
                    syn::PathArguments::AngleBracketed(args) => args,
                    ref unknown => {
                        return Err(Error::new(
                            last_segment.arguments.span(),
                            format!("Cannot create a typescript type from Vec<T>.  Unexpected arguments type: {unknown:?}."),
                        ))
                    }
                };
                let ok_type_path = match &args.args[0] {
                    GenericArgument::Type(Type::Path(ok_type_path)) => ok_type_path,
                    ref unknown => {
                        return Err(Error::new(
                            args.args[0].span(),
                            format!("Cannot create a typescript type from Vec<T>.  No type in result arguments: {unknown:?}."),
                        ))
                    }
                };

                let ts_type = TypescriptType::try_from(ok_type_path)?;

                Ok(TypescriptType::Array(Box::new(ts_type)))
            }
            class => Ok(TypescriptType::Struct(class.to_string())),
        }
    }
}

impl TryFrom<&FnArg> for TypescriptType {
    type Error = syn::Error;
    fn try_from(value: &FnArg) -> Result<Self, Self::Error> {
        match value {
            FnArg::Receiver(receiver) => Err(Error::new(
                receiver.span(),
                "Cannot create a typescript type from a receiver argument.",
            )),
            FnArg::Typed(pat_ty) => match *pat_ty.ty {
                syn::Type::Path(ref path) => TypescriptType::try_from(path),
                ref unknown => Err(Error::new(
                    unknown.span(),
                    format!("Cannot create a typescript type from typed argument.  Unknown type: {unknown:?}."),
                )),
            },
        }
    }
}

impl TryFrom<&ReturnType> for TypescriptType {
    type Error = syn::Error;
    fn try_from(value: &ReturnType) -> Result<Self, Self::Error> {
        match value {
            ReturnType::Default => Ok(TypescriptType::Void),
            ReturnType::Type(_, ty) => {
                match **ty {
                    Type::Path(ref ty_path) => {
                        TypescriptType::try_from(ty_path)
                    }
                    ref unknown => Err(Error::new(
                        unknown.span(),
                        format!("Cannot create a typescript type from typed return.  Unknown type: {unknown:?}."),
                    )),
                }
            }
            // FnArg::Receiver(receiver) => Err(Error::new(
            //     receiver.span(),
            //     "Cannot create a typescript type from a receiver argument.",
            // )),
            // FnArg::Typed(pat_ty) => match *pat_ty.ty {
            //     syn::Type::Path(ref path) => TypescriptType::try_from(path),
            //     ref unknown => Err(Error::new(
            //         unknown.span(),
            //         "Cannot create a typescript type from typed argument.  Unknown type.",
            //     )),
            // },
        }
    }
}

#[derive(Debug)]
pub struct ApiMethodArgs {
    pub world_ident: Ident,
    pub api_args: Punctuated<FnArg, Comma>,
}
impl TryFrom<&Punctuated<FnArg, Comma>> for ApiMethodArgs {
    type Error = Error;
    fn try_from(value: &Punctuated<FnArg, Comma>) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Error::new(
                value.span(),
                "Argument must have the at least 1 argument of `world: &mut World`.",
            ));
        }

        let mut iter = value.iter();
        let first = iter.next().unwrap();
        if let FnArg::Receiver(_) = &first {}

        let world_ident = match first {
            FnArg::Receiver(_) => {
                return Err(Error::new_spanned(
                    first.clone(),
                    "First argument must be `world: &mut World`.  Instead found `self`.",
                ))
            }
            FnArg::Typed(ref first_typed) => {
                match *first_typed.ty {
                    Type::Reference(ref reference) => {
                        if reference.mutability.is_none() {
                            return Err(Error::new_spanned(reference, format!("First argument must be `world: &mut World`.  First argument is a reference but is not mutable {:?}.", reference)));
                        }
                    }
                    ref unknown => {
                        return Err(Error::new_spanned(
                            unknown,
                            "First argument is unexpected type.  ",
                        ))
                    }
                }

                match *first_typed.pat {
                    Pat::Ident(ref ident) => {
                        &ident.ident
                    }
                    ref unknown => return Err(Error::new_spanned(unknown, format!("First argument must be `world: &mut World`.  First argument has unexpected pattern {:?}.", unknown)))
                }
            }
        };

        Ok(ApiMethodArgs {
            world_ident: world_ident.clone(),
            api_args: iter.cloned().collect(),
        })
    }
}

impl ApiMethodArgs {
    /// Token stream for defining the args on the external api (exposed to js)
    pub fn api_args_definition_token_stream(&self) -> TokenStream {
        let mut args: Vec<TokenStream> = vec![];
        args.push(quote! { &self });
        args.extend(self.api_args.iter().map(|fn_arg| {
            quote! { #fn_arg }
        }));
        quote! { #(#args),* }
    }

    /// Token stream for passing in the args to the inner function (original method).
    pub fn original_method_call_args_token_stream(&self) -> TokenStream {
        let mut idents = vec![self.world_ident.clone()];
        idents.extend(self.api_args.iter().map(|arg| {
            let FnArg::Typed(pat_ty) = arg else {
                panic!("Impossible")
            };
            let Pat::Ident(ref pat_ident) = *pat_ty.pat else {
                panic!("Impossible");
            };
            pat_ident.ident.clone()
        }));

        quote! { #(#idents),* }
    }
}
