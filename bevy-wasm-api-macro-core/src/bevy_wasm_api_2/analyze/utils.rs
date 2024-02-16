use proc_macro2::Ident;
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Error, FnArg, GenericArgument, Type,
    TypePath, Pat, ReturnType,
};

#[derive(Debug)]
pub(super) struct TypescriptArg {
    pub ident: Ident,
    pub ty: TypescriptType,
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

                Ok(TypescriptArg {
                    ident,
                    ty,
                })
            },
        }
    }
}

#[derive(Debug)]
pub(super) enum TypescriptType {
    Void,
    String,
    Number,
    Class(String),
    Promise(Box<TypescriptType>),
    // Array(Box<TypescriptType>),
    // Tuple(Vec<TypescriptType>),
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
            }
            class => Ok(TypescriptType::Class(class.to_string())),
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

pub(super) fn extract_first_method_argument(
    args: &Punctuated<FnArg, Comma>,
) -> syn::Result<(FnArg, Vec<FnArg>)> {
    let mut iter = args.iter();
    let first = iter.next();

    println!("Args length: {}", args.len());

    if first.is_none() {
        return Err(Error::new_spanned(
            args,
            "Missing first argument on function.  First argument must be of type &mut World.",
        ));
    }

    let remaining: Vec<_> = iter.cloned().collect();
    Ok((first.unwrap().clone(), remaining))
}
