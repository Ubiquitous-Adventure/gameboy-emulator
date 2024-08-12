extern crate proc_macro;
use std::iter::once;

use itertools::{repeat_n, Itertools};
use proc_macro::{Literal, Punct, Spacing, TokenStream, TokenTree};
use quote::quote;

enum Bit {
    One,
    Zero,
    Free,
}

pub fn generate_all_bitstrings(token_stream: TokenStream) -> TokenStream {
    if token_stream.is_empty() {
        return quote! {
            compile_error!("Expected exactly one literal indicating a bitstring pattern.");
        }
        .into();
    }

    let mut iter = token_stream.into_iter();
    let token = iter.next().unwrap();

    if iter.next().is_some() {
        return quote! {
            compile_error!("Expected exactly one literal indicating a bitstring pattern.");
        }
        .into();
    }

    let bitstring = token.to_string();

    if bitstring.len() != 8 {
        return quote! {
            compile_error!("Expected a bitstring pattern of length 8.");
        }
        .into();
    }

    let positions: Vec<Bit> = bitstring
        .chars()
        .filter_map(|c| {
            if c == '_' {
                Some(Bit::Free)
            } else if c == '0' {
                Some(Bit::Zero)
            } else if c == '1' {
                Some(Bit::One)
            } else {
                None
            }
        })
        .collect();

    if positions.len() != 8 {
        return quote! {
            compile_error!("The bitstring pattern may only contain '0', '1', and '_'.");
        }
        .into();
    }

    let mut vals: Vec<u8> = vec![0];
    for (i, position) in positions.into_iter().rev().enumerate() {
        match position {
            Bit::One => vals = vals.into_iter().map(|val| val + (1 << i)).collect(),
            Bit::Zero => {}
            Bit::Free => {
                vals = vals
                    .into_iter()
                    .flat_map(|val| once(val).chain(once(val + (1 << i))))
                    .collect()
            }
        };
    }
    assert!(!vals.is_empty());

    let separators = repeat_n(
        TokenTree::Punct(Punct::new('|', Spacing::Alone)),
        vals.len() - 1,
    );
    let literals = vals
        .into_iter()
        .map(|val| TokenTree::Literal(Literal::u8_suffixed(val)));

    TokenStream::from_iter(literals.interleave(separators))
}
