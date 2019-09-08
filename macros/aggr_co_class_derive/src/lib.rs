extern crate proc_macro;
use proc_macro::TokenStream;
use syn::ItemStruct;

use std::iter::FromIterator;

mod class_factory;
mod com_struct;
mod com_struct_impl;
mod deref_impl;
mod drop_impl;
mod iunknown_impl;

// Macro expansion entry point.

pub fn expand_derive_aggr_com_class(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as ItemStruct);

    // Parse attributes
    let base_itf_idents = macro_utils::get_base_interface_idents(&input);
    let aggr_itf_idents = macro_utils::get_aggr_map(&input);

    let mut out: Vec<TokenStream> = Vec::new();
    out.push(com_struct::generate(&base_itf_idents, &input).into());
    out.push(com_struct_impl::generate(&base_itf_idents, &aggr_itf_idents, &input).into());
    out.push(iunknown_impl::generate(&input).into());
    out.push(drop_impl::generate(&base_itf_idents, &input).into());
    out.push(deref_impl::generate(&input).into());
    out.push(class_factory::generate(&input).into());

    TokenStream::from_iter(out)
}