use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Algorithm)]
pub fn algorithm(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let expanded = quote! {
        impl std::ops::Deref for #struct_name {
            type Target = crate::core::AlgorithmCommon;
            fn deref(&self) -> &Self::Target {
                &self.common
            }
        }

        impl std::ops::DerefMut for #struct_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.common
            }
        }
    };

    TokenStream::from(expanded)
}
