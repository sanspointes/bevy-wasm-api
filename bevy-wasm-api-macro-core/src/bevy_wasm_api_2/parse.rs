use proc_macro2::TokenStream;
use syn::Result;

pub type Ast = syn::ItemImpl;

pub fn parse(ts: TokenStream) -> Result<Ast> {
    syn::parse2::<syn::ItemImpl>(ts)
}
