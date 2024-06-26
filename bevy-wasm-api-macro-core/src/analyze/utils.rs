use std::fmt::Display;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Error, FnArg, GenericArgument, Pat,
    ReturnType, Type, TypePath, TypeReference, TypeSlice, TypeTuple,
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
                    syn::Type::Reference(ref type_reference) => TypescriptType::try_from(type_reference)?,
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
    Option(Box<TypescriptType>),
    Tuple(Vec<TypescriptType>),
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
            TypescriptType::Option(inner) => write!(f, "{}|undefined", *inner),
            TypescriptType::Tuple(inner) => {
                write!(f, "[")?;
                for (i, inner_ty) in inner.into_iter().enumerate() {
                    write!(f, "{}", inner_ty)?;
                    if i < inner.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
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

            rs_type_str @ "Option" | rs_type_str @ "Result" | rs_type_str @ "Vec" => {
                let inner_type: TypescriptType = match &last_segment.arguments {
                    // If 
                    syn::PathArguments::AngleBracketed(args) => {
                        match &args.args[0] {
                            GenericArgument::Type(ty) => TypescriptType::try_from(ty)?,
                            ref unknown => {
                                return Err(Error::new(
                                    args.args[0].span(),
                                    format!("Cannot create a typescript type from {rs_type_str}.  No type in result arguments: {unknown:?}."),
                                ))
                            }
                        }
                    },
                    // If these are parenthesized that means it's a tuple
                    syn::PathArguments::Parenthesized(args) => {
                        let tuple_types: Result<Vec<_>, syn::Error> = args.inputs.iter().map(|ty| {
                            TypescriptType::try_from(ty)
                        }).collect();

                        TypescriptType::Tuple(tuple_types?)
                    },
                    ref unknown => {
                        return Err(Error::new(
                            last_segment.arguments.span(),
                            format!("Cannot create a typescript type from Result.  Unexpected arguments type: {unknown:?}."),
                        ))
                    }
                };

                match rs_type_str {
                    "Option" => Ok(TypescriptType::Option(Box::new(inner_type))),
                    "Result" => Ok(TypescriptType::Promise(Box::new(inner_type))),
                    "Vec" => Ok(TypescriptType::Array(Box::new(inner_type))),
                    _ => panic!("Impossible"),
                }
            }
            class => Ok(TypescriptType::Struct(class.to_string())),
        }
    }
}

impl TryFrom<&TypeReference> for TypescriptType {
    type Error = syn::Error;
    fn try_from(value: &TypeReference) -> Result<Self, Self::Error> {
        match *value.elem {
            Type::Path(ref path) => {
                TypescriptType::try_from(path)
            },
            _ => Err(Error::new(
                value.span(),
                format!("Cannot create a typescript type from type reference.  Unexpected arguments type: {value:?}."),
            ))
        }
    }
}

impl TryFrom<&TypeSlice> for TypescriptType {
    type Error = syn::Error;
    fn try_from(value: &TypeSlice) -> Result<Self, Self::Error> {
        let inner_type: Result<TypescriptType, Error> = value.elem.as_ref().try_into();
        Ok(TypescriptType::Array(Box::new(inner_type?)))
    }
}

impl TryFrom<&TypeTuple> for TypescriptType {
    type Error = syn::Error;
    fn try_from(value: &TypeTuple) -> Result<Self, Self::Error> {
        let inner_types: Result<Vec<_>, Error> =
            value.elems.iter().map(TypescriptType::try_from).collect();
        Ok(TypescriptType::Tuple(inner_types?))
    }
}

impl TryFrom<&syn::Type> for TypescriptType {
    type Error = syn::Error;
    fn try_from(value: &syn::Type) -> Result<Self, Self::Error> {
        match value {
            Type::Path(ty_path) => ty_path.try_into(),
            Type::Tuple(ty_tuple) => ty_tuple.try_into(),
            Type::Reference(ty_reference) => ty_reference.try_into(),
            Type::Slice(ty_slice) => ty_slice.try_into(),
            ref unknown => Err(Error::new(
                unknown.span(),
                format!("Cannot create a typescript type from tuple type.  Unknown inner type: {unknown:?}."),
            ))
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
            FnArg::Typed(pat_ty) => TypescriptType::try_from(&*pat_ty.ty),
        }
    }
}

impl TryFrom<&ReturnType> for TypescriptType {
    type Error = syn::Error;
    fn try_from(value: &ReturnType) -> Result<Self, Self::Error> {
        match value {
            ReturnType::Default => Ok(TypescriptType::Void),
            ReturnType::Type(_, ty) => TypescriptType::try_from(&**ty),
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
