extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Lit, Meta, MetaList, NestedMeta};

#[proc_macro_derive(AnswerFn)]
pub fn derive_answer_fn(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro_derive(new)]
pub fn derive_new(item: TokenStream) -> TokenStream {
    // https://docs.rs/syn/0.15.29/syn/struct.DeriveInput.html
    let ast = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;

    let doc_string = format!("Returns a new {}", name);
    let token_stream2 = quote! {
        impl #name {
            #[doc = #doc_string]
            pub fn new() -> Self {
                Self
            }
        }
    };

    TokenStream::from(token_stream2)
}

#[proc_macro_derive(Foo)]
pub fn derive_foo(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;

    let token_stream2 = quote! {
        impl Foo for #name {
            fn foo() -> u32 {
                42
            }
        }
    };

    TokenStream::from(token_stream2)
}

#[proc_macro_derive(FooBar)]
pub fn derive_foo_bar(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;

    let foo_bar = Ident::new(&format!("{}Bar", name), Span::call_site());

    let token_stream2 = quote! {
        #[derive(Debug)]
        struct #foo_bar;

        impl #name {
            pub fn bar(&self) -> #foo_bar {
                #foo_bar
            }
        }
    };

    TokenStream::from(token_stream2)
}

#[proc_macro_derive(ShowAttribute, attributes(show_attr, random))]
pub fn derive_show_attr(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;

    let attr_to_show = ast
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident("show_attr"))
        .filter_map(|attr| attr.parse_meta().ok())
        .map(|meta| {
            if let Meta::List(MetaList { nested, .. }) = meta {
                nested
                    .iter()
                    .filter_map(|nested_meta| {
                        if let NestedMeta::Literal(Lit::Str(lit_str)) = nested_meta {
                            Some(lit_str.value())
                        } else {
                            None
                        }
                    })
                    .next()
                    .expect("`show_attr` must have a literal argument, e.g. #[show_attr(\"doc\")]")
            } else {
                panic!("`show_attr` must have a literal argument, e.g. #[show_attr(\"doc\")]");
            }
        })
        .next()
        .unwrap_or_else(|| String::from("doc"));

    let attr_list = ast
        .attrs
        .iter()
        .filter(|attr| attr.path.is_ident(&attr_to_show))
        .map(|attr| format!("{}", quote!(#attr)))
        .collect::<Vec<_>>();

    let token_stream2 = quote! {
        impl #name {
            pub fn attr_list(&self) -> Vec<&'static str> {
                vec![#(#attr_list),*]
            }
        }
    };

    TokenStream::from(token_stream2)
}

#[proc_macro_derive(SomeoneElsesDerive, attributes(someone_elses_attr))]
pub fn derive_someone_elses_derive(_item: TokenStream) -> TokenStream {
    TokenStream::from(quote!())
}
