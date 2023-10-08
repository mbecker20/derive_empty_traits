use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(EmptyTraits, attributes(empty_traits))]
pub fn derive_empty_traits(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput { attrs, ident, .. } = syn::parse(input).unwrap();

    let traits = attrs
        .iter()
        .filter(|attr| attr.path().is_ident("empty_traits"))
        .map(|t| {
            t.parse_args::<TokenStream>()
                .expect("failed to parse Trait")
        });

    quote! {
        #(impl #traits for #ident {})*
    }
    .into()
}
