use super::CompoundSelector;
use super::ParseVariable;
use quote::{quote, ToTokens};
use std::collections::HashSet;
use syn::parse::{ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{token::Token, Ident, Token};

pub struct Selectors(Punctuated<CompoundSelector, Token![,]>);

impl ParseVariable for Selectors {
    fn parse(input: ParseStream, vars: &mut HashSet<Ident>) -> Result<Self> {
        let mut punctuated = Punctuated::new();

        loop {
            let value = CompoundSelector::parse(input, vars)?;
            punctuated.push_value(value);
            if !<Token![,]>::peek(input.cursor()) {
                break;
            }
            let punct = input.parse()?;
            punctuated.push_punct(punct);
        }

        Ok(Selectors(punctuated))
    }
}

impl ToTokens for Selectors {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Selectors(selectors) = self;
        let selectors = selectors.into_iter();
        tokens.extend(quote! {vec![
            #(
                #selectors
            ),*
        ]
        .join(",\n")});
    }
}
