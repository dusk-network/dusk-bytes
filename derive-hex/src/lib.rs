// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(Hex)]
pub fn derive_hex(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let ident = &input.ident;

    (quote! {
        impl core::fmt::LowerHex for #ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let bytes = self.to_bytes();

                if f.alternate() {
                    write!(f, "0x")?
                }

                for byte in &bytes[..] {
                    write!(f, "{:02x}", &byte)?
                }

                Ok(())
            }
        }

        impl core::fmt::UpperHex for #ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let bytes = self.to_bytes();

                if f.alternate() {
                    write!(f, "0x")?
                }

                for byte in &bytes[..] {
                    write!(f, "{:02X}", &byte)?
                }

                Ok(())
            }
        }
    })
    .into()
}

#[proc_macro_derive(HexDebug)]
pub fn derive_hex_debug(item: TokenStream) -> TokenStream {
    let mut hex: TokenStream = derive_hex(item.clone());
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let ident = &input.ident;

    let dbg: TokenStream = (quote! {
    impl core::fmt::Debug for #ident {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            // Once we format an object using the debug notation (e.g. `{:x?}`)
            // there is absolutely NO WAY to detect the flag for the lowerhex
            // or upperhex, and therefore forwarding to the relevant formatter.
            // Two methods for this purpose exists, but they're not exposed
            // because they didn't agree on a name yet, see:
            // <https://github.com/rust-lang/rust/blob/90442458ac46b1d5eed752c316da25450f67285b/library/core/src/fmt/mod.rs#L1817-L1825>
            //
            // Therefore the only way is using the deprecated method `flags`,
            // implementing the same logic of the forementioned methods.

            // We also do not have access to the `FlagV1` enum since it's
            // private.
            let FlagV1_DebugUpperHex = 5_u32;

            #[allow(deprecated)]
            if f.flags() & (1 << FlagV1_DebugUpperHex) !=0 {
                core::fmt::UpperHex::fmt(self, f)
            } else { // LowerHex is always the default for debug
                core::fmt::LowerHex::fmt(self, f)
            }
        }
    }})
    .into();

    hex.extend(dbg);
    hex
}
