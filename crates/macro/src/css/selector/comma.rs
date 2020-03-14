use super::ParseVariable;
use super::{MultiSelector, Selector};
use crate::Peek;
use boolinator::Boolinator;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use std::collections::HashSet;
use syn::buffer::Cursor;
use syn::parse::{ParseStream, Result};
use syn::{Ident, Token};

pub struct Comma(Selector, Token![,], Box<MultiSelector>);

impl Peek<'_, Self> for Comma {
    fn peek(cursor: Cursor) -> Option<(Self, Cursor)> {
        let (s1, cursor) = Selector::peek(cursor)?;

        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == ',').as_option()?;

        let (s2, cursor) = MultiSelector::peek(cursor)?;

        Some((
            Comma(s1, Token![,](Span::call_site()), Box::new(s2)),
            cursor,
        ))
    }
}

impl ParseVariable for Comma {
    fn parse(input: ParseStream, vars: &mut HashSet<Ident>) -> Result<Self> {
        let s1 = Selector::parse(input, vars)?;
        let comma = input.parse::<Token![,]>()?;
        let s2 = MultiSelector::parse(input, vars)?;

        Ok(Comma(s1, comma, Box::new(s2)))
    }
}

impl ToTokens for Comma {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Comma(s1, _, s2) = self;
        let s1 = s1.to_token_stream();
        let s2 = s2.to_token_stream();
        tokens.extend(quote! { format!("{},\n{}", #s1, #s2) });
    }
}
