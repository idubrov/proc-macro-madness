extern crate proc_macro;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn madness_desugar(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let header = quote! {
        #[derive(madness_macro::Madness)]
    };
    let mut out: proc_macro::TokenStream = header.into();
    out.extend(item);
    out
}

#[proc_macro_attribute]
pub fn madness_direct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input: DeriveInput = parse_macro_input!(item as DeriveInput);
    if let syn::Data::Struct(ref mut ds) = input.data {
        if let syn::Fields::Named(ref mut fields) = ds.fields {
            for field in fields.named.iter_mut() {
                field.attrs.clear();
            }
        }
    }
    input.into_token_stream().into()
}

#[proc_macro_derive(Madness, attributes(madness_tag))]
pub fn maddness_derive(item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item as DeriveInput);
    bencher::black_box(input);
    TokenStream::new()
}
