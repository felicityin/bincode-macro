extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(Serde)]
pub fn pack_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let name = ast.ident;
    let gen = quote! {
        impl #name {
            pub fn pack(&self) -> Result<Vec<u8>, error::EncodeError> {
                bincode::encode_to_vec(&self, bincode::config::legacy())
            }

            pub fn unpack(&self, buf: &[u8]) -> Result<(Self, usize), error::DecodeError> {
                bincode::decode_from_slice(buf, bincode::config::legacy())
            }
        }
    };
    gen.into()
}
