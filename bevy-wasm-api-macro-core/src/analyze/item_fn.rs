use proc_macro2::{Ident, TokenStream, Span};
use quote::quote;
use syn::{ImplItemFn, Error, Result};

use crate::analyze::utils::TypescriptArg;

use super::utils::{TypescriptType, ApiMethodArgs};

#[derive(Debug)]
pub struct ImplItemFnModel {
    pub original_method_ident: Ident,
    pub api_method_args: ApiMethodArgs,
    pub typescript_arguments: Vec<TypescriptArg>,
    pub typescript_return_type: TypescriptType,
}

impl TryFrom<&ImplItemFn> for ImplItemFnModel {
    type Error = Error;
    fn try_from(method: &ImplItemFn) -> std::prelude::v1::Result<Self, Self::Error> {
        let method_name = &method.sig.ident;
        let method_inputs = &method.sig.inputs;
        let method_output = &method.sig.output;


        let api_method_args = ApiMethodArgs::try_from(method_inputs)?;

        let typescript_arguments = api_method_args.api_args.iter().map(|fn_arg| {
            TypescriptArg::try_from(fn_arg)
        }).collect::<Result<Vec<TypescriptArg>>>()?;

        Ok(ImplItemFnModel {
            original_method_ident: method_name.clone(),
            api_method_args,
            typescript_arguments,
            typescript_return_type: TypescriptType::try_from(method_output)?,
        })
    }
}
