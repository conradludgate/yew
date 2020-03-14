use super::ParseVariable;
use crate::Peek;
use boolinator::Boolinator;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use std::collections::HashSet;
use syn::buffer::Cursor;
use syn::parse::{ParseStream, Result};
use syn::{Ident, Token};

pub struct Variable(Token![$], Ident);

impl Peek<'_, Self> for Variable {
    fn peek(cursor: Cursor) -> Option<(Self, Cursor)> {
        let (punct, cursor) = cursor.punct()?;
        (punct.as_char() == '$').as_option()?;

        let (ident, cursor) = cursor.ident()?;
        Some((Variable(Token![$](Span::call_site()), ident), cursor))
    }
}

impl ParseVariable for Variable {
    fn parse(input: ParseStream, vars: &mut HashSet<Ident>) -> Result<Self> {
        let q = input.parse::<Token![$]>()?;
        let ident = input.parse::<Ident>()?;

        vars.insert(ident.clone());

        Ok(Variable(q, ident.clone()))
    }
}

impl ToTokens for Variable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Variable(_, ident) = self;
        tokens.extend(quote! { format!(".{}", #ident) });
    }
}
