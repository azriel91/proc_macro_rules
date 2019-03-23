extern crate proc_macro;

use proc_macro::TokenStream;

/// Returns the item.
#[proc_macro_attribute]
pub fn noop(_args: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Removes the annotated item.
#[proc_macro_attribute]
pub fn remove(_args: TokenStream, _item: TokenStream) -> TokenStream {
    "".parse().unwrap()
}
