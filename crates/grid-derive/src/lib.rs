use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, LitChar, parse_macro_input};

#[proc_macro_derive(Cellable, attributes(token))]
pub fn derive_parse_cell(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let out = match input.data {
        syn::Data::Enum(e) => {
            let variants = e
                .variants
                .iter()
                .map(|variant| &variant.ident)
                .collect::<Vec<_>>();
            let values = e
                .variants
                .iter()
                .map(|variant| {
                    variant
                        .attrs
                        .first()
                        .unwrap()
                        .parse_args::<LitChar>()
                        .unwrap()
                })
                .collect::<Vec<_>>();
            quote! {
                impl TryFrom<char> for #name {
                    type Error = grid::ParseGridError;
                    fn try_from(value: char) -> Result<#name, Self::Error> {
                        match value {
                            #(#values => Ok(#name::#variants),)*
                            _ => Err(grid::ParseGridError)
                        }
                    }
                }

                impl std::fmt::Display for #name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        use std::fmt::Write;
                        f.write_char(match self {
                            #(Self::#variants => #values,)*
                        })
                    }
                }
            }
        }
        _ => unreachable!("We only support enums"),
    };

    out.into()
}
