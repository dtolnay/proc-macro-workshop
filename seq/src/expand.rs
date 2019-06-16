use self::Counter::*;
use crate::bounds::Bounds;
use crate::parse::{Input, Template, Token};
use proc_macro2::{Group, Ident, Literal, TokenStream, TokenTree};
use quote::ToTokens;
use syn::{Error, Result};

pub fn expand(input: Input) -> Result<TokenStream> {
    let mut out = TokenStream::new();

    if input.body.content.iter().any(Token::has_section) {
        input.body.instantiate(Range(input.bounds), &mut out)?;
    } else {
        for i in input.bounds {
            input.body.instantiate(Once(i), &mut out)?;
        }
    }

    Ok(out)
}

#[derive(Copy, Clone)]
enum Counter {
    Once(u64),
    Range(Bounds),
}

impl Template {
    fn instantiate(&self, counter: Counter, out: &mut TokenStream) -> Result<()> {
        for token in &self.content {
            token.instantiate(counter, out)?;
        }
        Ok(())
    }
}

impl Token {
    fn instantiate(&self, counter: Counter, out: &mut TokenStream) -> Result<()> {
        match self {
            Token::Var(var) => match counter {
                Once(i) => {
                    let mut lit = Literal::u64_unsuffixed(i);
                    lit.set_span(var.span());
                    out.extend(Some(TokenTree::Literal(lit)));
                }
                Range(_) => {
                    out.extend(Some(TokenTree::Ident(var.clone())));
                }
            },
            Token::Paste(paste) => match counter {
                Once(i) => {
                    let pasted = format!("{}{}", paste.prefix, i);
                    let ident = Ident::new(&pasted, paste.prefix.span());
                    out.extend(Some(TokenTree::Ident(ident)));
                }
                Range(_) => {
                    return Err(Error::new_spanned(paste, "unexpected paste"));
                }
            },
            Token::Section(section) => match counter {
                Once(_) => {
                    return Err(Error::new(proc_macro2::Span::call_site(), ""));
                }
                Range(range) => {
                    for i in range {
                        section.template.instantiate(Once(i), out)?;
                    }
                }
            },
            Token::Group(group) => {
                let mut stream = TokenStream::new();
                group.template.instantiate(counter, &mut stream)?;
                let mut token = Group::new(group.delimiter, stream);
                token.set_span(group.span.join());
                out.extend(Some(TokenTree::Group(token)));
            }
            Token::Token(token) => token.to_tokens(out),
        }
        Ok(())
    }

    fn has_section(&self) -> bool {
        match self {
            Token::Section(_) => true,
            Token::Group(group) => group.template.content.iter().any(Token::has_section),
            _ => false,
        }
    }
}
