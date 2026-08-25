// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Hex)]
/// Derives lower- and upper-hexadecimal formatting for a serializable type.
///
/// # Security
///
/// Do not derive `Hex` for types containing secrets or other confidential
/// material. Hexadecimal formatting exposes the complete serialized
/// representation.
pub fn derive_hex(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) =
        input.generics.split_for_impl();

    (quote! {
        impl #impl_generics ::core::fmt::LowerHex for #ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                if f.alternate() {
                    ::core::write!(f, "0x")?
                }

                for byte in self.to_bytes() {
                    ::core::write!(f, "{byte:02x}")?
                }

                ::core::result::Result::Ok(())
            }
        }

        impl #impl_generics ::core::fmt::UpperHex for #ident #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                if f.alternate() {
                    ::core::write!(f, "0x")?
                }

                for byte in self.to_bytes() {
                    ::core::write!(f, "{byte:02X}")?
                }

                ::core::result::Result::Ok(())
            }
        }
    })
    .into()
}

#[proc_macro_derive(HexDebug)]
/// Derives hexadecimal formatting and renders the serialized value for
/// [`Debug`](::core::fmt::Debug).
///
/// # Security
///
/// Do not derive `HexDebug` for types containing secrets or other confidential
/// material. It exposes the complete serialized representation through both
/// hexadecimal and ordinary debug formatting.
pub fn derive_hex_debug(item: TokenStream) -> TokenStream {
    let mut hex: TokenStream = derive_hex(item.clone());
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) =
        input.generics.split_for_impl();

    let dbg: TokenStream = (quote! {
    impl #impl_generics ::core::fmt::Debug for #ident #ty_generics #where_clause {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            // `Formatter` does not publicly expose the debug-hex case. Bit 5 is
            // `FlagV1::DebugUpperHex` in `core`:
            // <https://github.com/rust-lang/rust/blob/90442458ac46b1d5eed752c316da25450f67285b/library/core/src/fmt/mod.rs#L1817-L1825>
            const DEBUG_UPPER_HEX: u32 = 1 << 5;

            #[allow(deprecated)]
            if f.flags() & DEBUG_UPPER_HEX != 0 {
                ::core::fmt::UpperHex::fmt(self, f)
            } else { // LowerHex is always the default for debug
                ::core::fmt::LowerHex::fmt(self, f)
            }
        }
    }})
    .into();

    hex.extend(dbg);
    hex
}
