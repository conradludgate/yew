pub mod hyphenate;
pub mod selector;
pub mod values;

use hyphenate::to_hyphen_case;
use quote::{quote, ToTokens};
use selector::Selectors;
use std::collections::HashSet;
use syn::braced;
use syn::parse::{Parse, ParseStream, Result};
use values::StyleValues;

pub struct StyleSheet {
    blocks: Vec<StyleBlock>,
    variables: HashSet<syn::Ident>,
}

impl Parse for StyleSheet {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut stylesheet = StyleSheet {
            blocks: vec![],
            variables: HashSet::new(),
        };

        while !input.is_empty() {
            let block = StyleBlock::parse(input, &mut stylesheet.variables)?;
            stylesheet.blocks.push(block);
        }

        Ok(stylesheet)
    }
}

impl ToTokens for StyleSheet {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let vars = self.variables.iter().zip(0..).map(|(v, i)| {
            let value = format!("{}-{}", to_hyphen_case(&v.to_string()), i);
            quote! {
                #v = #value;
            }
        });

        let nodes = self.blocks.iter();

        tokens.extend(quote! {
            #(
                #vars
            )*
            vec![
                #(
                    #nodes
                ),*
            ].join("\n")
        });
    }
}

pub trait ParseVariable: Sized {
    fn parse(input: ParseStream, vars: &mut HashSet<syn::Ident>) -> Result<Self>;
}

pub struct StyleBlock {
    selector: Selectors,
    contents: StyleValues,
}

impl ParseVariable for StyleBlock {
    fn parse(input: ParseStream, vars: &mut HashSet<syn::Ident>) -> Result<Self> {
        let selector = Selectors::parse(input, vars)?;

        let contents;
        braced!(contents in input);

        Ok(StyleBlock {
            selector,
            contents: contents.parse::<StyleValues>()?,
        })
    }
}

impl ToTokens for StyleBlock {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let selector = self.selector.to_token_stream();
        let contents = self.contents.to_token_stream();
        tokens.extend(quote! {
            format!("{} {}", #selector, #contents)
        });
    }
}
