extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Expr, ExprLit, Ident, ItemFn, Lit, Stmt};

#[proc_macro]
pub fn function_like(_item: TokenStream) -> TokenStream {
    "fn hello() -> &'static str { \"rara\" }".parse().unwrap()
}

#[proc_macro]
pub fn rename(item: TokenStream) -> TokenStream {
    let mut item_fn = syn::parse::<ItemFn>(item).expect("Failed to parse.");
    let name = {
        let statement = item_fn.block.stmts.first().expect("Expected one statement");
        if let Stmt::Expr(Expr::Lit(ExprLit {
            lit: Lit::Str(lit_str),
            ..
        })) = statement
        {
            lit_str.value()
        } else {
            panic!("Expected single string literal in function body. E.g. `\"tom\"`.")
        }
    };

    item_fn.ident = Ident::new(&name, Span::call_site());
    let token_stream2 = quote! {
        #item_fn
    };
    TokenStream::from(token_stream2)
}
