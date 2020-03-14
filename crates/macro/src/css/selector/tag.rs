use crate::Peek;
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result};
use syn::Ident;

pub struct Tag(Ident);

impl Parse for Tag {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident = input.parse::<Ident>()?;
        Ok(Tag(ident))
    }
}

impl Peek<'_, Self> for Tag {
    fn peek(cursor: Cursor) -> Option<(Self, Cursor)> {
        let (ident, cursor) = cursor.ident()?;
        Some((Tag(ident), cursor))
    }
}

impl ToTokens for Tag {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Tag(ident) = self;
        let tag = ident.to_string();
        tokens.extend(quote! {
            #tag
        })
    }
}
