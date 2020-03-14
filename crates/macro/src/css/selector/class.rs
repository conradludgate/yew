use crate::html_tree::html_dashed_name::HtmlDashedName as DashedName;
use crate::Peek;
use boolinator::Boolinator;
use proc_macro2::{Spacing, Span};
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result};
use syn::Token;

pub struct Class(Token![.], DashedName);

impl Peek<'_, Self> for Class {
    fn peek(cursor: Cursor) -> Option<(Self, Cursor)> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '.').as_option()?;

        (punct.spacing() == Spacing::Alone).as_option()?;

        let (name, cursor) = DashedName::peek(cursor)?;

        Some((Class(Token![.](Span::call_site()), name), cursor))
    }
}

impl Parse for Class {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Class(
            input.parse::<Token![.]>()?,
            input.parse::<DashedName>()?,
        ))
    }
}

impl ToTokens for Class {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Class(_, name) = self;
        let name = format!(".{}", name);
        tokens.extend(quote! { #name.to_string() });
    }
}
