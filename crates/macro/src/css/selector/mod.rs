mod append;
mod class;
mod comma;
mod id;
mod tag;
mod variable;

use super::ParseVariable;
use crate::Peek;
use append::Append;
use class::Class;
use comma::Comma;
use id::Id;
use quote::ToTokens;
use std::collections::HashSet;
use syn::buffer::Cursor;
use syn::parse::{Parse, ParseStream, Result};
use syn::Ident;
use tag::Tag;
use variable::Variable;

pub enum MultiSelector {
    Comma(Comma),
    Append(Append),
    Single(Selector),
}

impl ParseVariable for MultiSelector {
    fn parse(input: ParseStream, vars: &mut HashSet<Ident>) -> Result<Self> {
        let (s, _) = MultiSelector::peek(input.cursor())
            .ok_or(input.error("expected valid selector element"))?;

        Ok(match s {
            MultiSelector::Comma(_) => MultiSelector::Comma(Comma::parse(input, vars)?),
            MultiSelector::Append(_) => MultiSelector::Append(Append::parse(input, vars)?),
            MultiSelector::Single(_) => MultiSelector::Single(Selector::parse(input, vars)?),
        })
    }
}

impl Peek<'_, Self> for MultiSelector {
    fn peek(cursor: Cursor) -> Option<(Self, Cursor)> {
        // Only peek if a single selector can be found first
        let (s, c) = Selector::peek(cursor)?;

        if let Some((comma, cursor)) = Comma::peek(cursor) {
            Some((MultiSelector::Comma(comma), cursor))
        } else if let Some((append, cursor)) = Append::peek(cursor) {
            Some((MultiSelector::Append(append), cursor))
        } else {
            Some((MultiSelector::Single(s), c))
        }
    }
}

impl ToTokens for MultiSelector {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            MultiSelector::Comma(comma) => comma.to_tokens(tokens),
            MultiSelector::Append(append) => append.to_tokens(tokens),
            MultiSelector::Single(single) => single.to_tokens(tokens),
        }
    }
}

pub enum Selector {
    Class(Class),
    Id(Id),
    Variable(Variable),
    Tag(Tag),
}

impl ParseVariable for Selector {
    fn parse(input: ParseStream, vars: &mut HashSet<Ident>) -> Result<Self> {
        let (s, _) =
            Selector::peek(input.cursor()).ok_or(input.error("expected valid selector element"))?;

        Ok(match s {
            Selector::Class(_) => Selector::Class(Class::parse(input)?),
            Selector::Id(_) => Selector::Id(Id::parse(input)?),
            Selector::Variable(_) => Selector::Variable(Variable::parse(input, vars)?),
            Selector::Tag(_) => Selector::Tag(Tag::parse(input)?),
        })
    }
}

impl Peek<'_, Self> for Selector {
    fn peek(cursor: Cursor) -> Option<(Self, Cursor)> {
        if let Some((class, cursor)) = Class::peek(cursor) {
            Some((Selector::Class(class), cursor))
        } else if let Some((id, cursor)) = Id::peek(cursor) {
            Some((Selector::Id(id), cursor))
        } else if let Some((var, cursor)) = Variable::peek(cursor) {
            Some((Selector::Variable(var), cursor))
        } else if let Some((tag, cursor)) = Tag::peek(cursor) {
            Some((Selector::Tag(tag), cursor))
        } else {
            None
        }
    }
}

impl ToTokens for Selector {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Selector::Class(class) => class.to_tokens(tokens),
            Selector::Id(id) => id.to_tokens(tokens),
            Selector::Variable(var) => var.to_tokens(tokens),
            Selector::Tag(tag) => tag.to_tokens(tokens),
        }
    }
}
