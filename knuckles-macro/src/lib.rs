extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

fn extract_ident(item: &Item) -> Option<syn::Ident> {
    match item {
        Item::Struct(s) => Some(s.ident.clone()),
        Item::Enum(e) => Some(e.ident.clone()),
        _ => None,
    }
}

#[proc_macro_attribute]
pub fn pydefault(_: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);
    let python_new = quote! {
        #[new]
        fn python_new(str: &str) -> Self {
            Self::new(str)
        }
    };
    let repr = quote! {
        fn __repr__(&self) -> String {
            format!("{:?}", self)
        }
    };
    if let Some(ident) = extract_ident(&item) {
        let expander = quote! {
            #[pymethods]
            #[allow(non_snake_case)]
            impl #ident {
                #python_new
                #repr
            }
            #item
        };
        TokenStream::from(expander)
    } else {
        TokenStream::from(quote! { #item })
    }
}
