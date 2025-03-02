mod controller_from_state;

use crate::controller_from_state::controller_from_state_impl;
use proc_macro::TokenStream;

#[proc_macro_derive(ControllerFromState)]
pub fn controller_from_state(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    controller_from_state_impl(ast)
}
