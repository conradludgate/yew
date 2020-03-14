// use super::hyphenate::to_hyphen_case;
// use super::ParseVariable;
// use crate::html_tree::html_dashed_name::HtmlDashedName as DashedName;
use crate::Peek;
// use crate::PeekValue;
// use boolinator::Boolinator;
// use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result};
// use syn::Token;

#[derive(Clone)]
pub struct StyleValues();

impl ToTokens for StyleValues {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(quote! {
            "{}"
        });
    }
}

impl Parse for StyleValues {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            Ok(StyleValues())
        } else {
            Err(input.error("No content allowed"))
        }
    }
}

impl Peek<'_, Self> for StyleValues {
    fn peek(cursor: Cursor) -> Option<(Self, Cursor)> {
        Some((StyleValues(), cursor))
    }
}
