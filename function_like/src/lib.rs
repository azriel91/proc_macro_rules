extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Expr, ExprLit, Ident, ItemFn, Lit, Stmt};

#[proc_macro]
pub fn function_like(_token_stream: TokenStream) -> TokenStream {
    "fn hello() -> &'static str { \"rara\" }".parse().unwrap()
}

#[proc_macro]
pub fn ensure_empty(token_stream: TokenStream) -> TokenStream {
    if !token_stream.is_empty() {
        panic!("Expected no arguments to this macro.");
    }

    "".parse().unwrap()
}

#[proc_macro]
pub fn rename(item: TokenStream) -> TokenStream {
    // https://docs.rs/syn/0.15.29/syn/struct.ItemFn.html
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
