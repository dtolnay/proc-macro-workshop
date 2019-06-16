use crate::bounds::Bounds;
use proc_macro2::extra::DelimSpan;
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, bracketed, parenthesized, token, Ident, LitInt, Token};

pub struct Input {
    pub bounds: Bounds,
    pub body: Template,
}

pub struct Template {
    pub content: Vec<Token>,
}

pub enum Token {
    Var(Ident),
    Paste(Paste),
    Section(Section),
    Group(Group),
    Token(TokenTree),
}

pub struct Paste {
    pub prefix: Ident,
    pub tilde_token: Token![~],
    pub var: Ident,
}

pub struct Section {
    pub pound_token: Token![#],
    pub paren_token: token::Paren,
    pub template: Template,
    pub star_token: Token![*],
}

pub struct Group {
    pub delimiter: Delimiter,
    pub span: DelimSpan,
    pub template: Template,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let var: Ident = input.parse()?;
        let _: Token![in] = input.parse()?;
        let lo: LitInt = input.parse()?;

        let lookahead = input.lookahead1();
        let bounds = if lookahead.peek(Token![..=]) {
            let _: Token![..=] = input.parse()?;
            let hi: LitInt = input.parse()?;
            Bounds::Inclusive(lo.base10_parse()?, hi.base10_parse()?)
        } else if lookahead.peek(Token![..]) {
            let _: Token![..] = input.parse()?;
            let hi: LitInt = input.parse()?;
            Bounds::Exclusive(lo.base10_parse()?, hi.base10_parse()?)
        } else {
            return Err(lookahead.error());
        };

        let content;
        braced!(content in input);
        let body = Template::parse(&content, &var)?;

        Ok(Input { bounds, body })
    }
}

impl Template {
    fn parse(input: ParseStream, var: &Ident) -> Result<Self> {
        let mut content = Vec::new();
        while !input.is_empty() {
            content.push(Token::parse(input, var)?);
        }
        Ok(Template { content })
    }
}

impl Token {
    fn parse(input: ParseStream, var: &Ident) -> Result<Self> {
        if let Some(ident) = input.parse::<Option<Ident>>()? {
            if ident == *var {
                return Ok(Token::Var(ident));
            }
            if input.peek(Token![~]) && input.peek2(Ident) {
                let ahead = input.fork();
                let _: Token![~] = ahead.parse()?;
                let next: Ident = ahead.parse()?;
                if next == *var {
                    return Ok(Token::Paste(Paste {
                        prefix: ident,
                        tilde_token: input.parse()?,
                        var: input.parse()?,
                    }));
                }
            }
            return Ok(Token::Token(TokenTree::Ident(ident)));
        }

        if input.peek(Token![#]) && input.peek2(token::Paren) {
            return Section::parse(input, var).map(Token::Section);
        }

        let content;
        let (delimiter, span) = if input.peek(token::Paren) {
            let token = parenthesized!(content in input);
            (Delimiter::Parenthesis, token.span)
        } else if input.peek(token::Brace) {
            let token = braced!(content in input);
            (Delimiter::Brace, token.span)
        } else if input.peek(token::Bracket) {
            let token = bracketed!(content in input);
            (Delimiter::Bracket, token.span)
        } else {
            return input.parse().map(Token::Token);
        };

        let template = Template::parse(&content, var)?;
        Ok(Token::Group(Group {
            delimiter,
            span,
            template,
        }))
    }
}

impl Section {
    fn parse(input: ParseStream, var: &Ident) -> Result<Self> {
        let content;
        Ok(Section {
            pound_token: input.parse()?,
            paren_token: parenthesized!(content in input),
            template: Template::parse(&content, var)?,
            star_token: input.parse()?,
        })
    }
}

impl ToTokens for Template {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for token in &self.content {
            token.to_tokens(tokens);
        }
    }
}

impl ToTokens for Token {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Token::Var(ident) => ident.to_tokens(tokens),
            Token::Paste(paste) => paste.to_tokens(tokens),
            Token::Section(section) => section.to_tokens(tokens),
            Token::Group(group) => group.to_tokens(tokens),
            Token::Token(tt) => tt.to_tokens(tokens),
        }
    }
}

impl ToTokens for Paste {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.prefix.to_tokens(tokens);
        self.tilde_token.to_tokens(tokens);
        self.var.to_tokens(tokens);
    }
}

impl ToTokens for Section {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.pound_token.to_tokens(tokens);
        self.paren_token.surround(tokens, |tokens| {
            self.template.to_tokens(tokens);
        });
        self.star_token.to_tokens(tokens);
    }
}

impl ToTokens for Group {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let content = |tokens: &mut _| self.template.to_tokens(tokens);
        match self.delimiter {
            Delimiter::Parenthesis => token::Paren(self.span).surround(tokens, content),
            Delimiter::Brace => token::Brace(self.span).surround(tokens, content),
            Delimiter::Bracket => token::Bracket(self.span).surround(tokens, content),
            Delimiter::None => unreachable!(),
        }
    }
}
