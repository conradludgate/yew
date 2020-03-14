use crate::html_tree::html_dashed_name::HtmlDashedName as DashedName;
use crate::Peek;
use boolinator::Boolinator;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result};
use syn::Token;

pub struct Id(Token![#], DashedName);

impl Peek<'_, Self> for Id {
    fn peek(cursor: Cursor) -> Option<(Self, Cursor)> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '#').as_option()?;

        let (name, cursor) = DashedName::peek(cursor)?;

        Some((Id(Token![#](Span::call_site()), name), cursor))
    }
}

impl Parse for Id {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Id(
            input.parse::<Token![#]>()?,
            input.parse::<DashedName>()?,
        ))
    }
}

impl ToTokens for Id {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Id(_, name) = self;
        let name = format!("#{}", name.to_string());
        tokens.extend(quote! { #name });
    }
}
