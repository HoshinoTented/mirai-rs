extern crate proc_macro;

use std::iter::FromIterator;

use proc_macro::{TokenTree, TokenStream, Group, Delimiter, Ident, Punct, Spacing};

#[proc_macro_attribute]
pub fn bot_event(attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let mut token_stream = Vec::new();

    for token in tokens.clone().into_iter() {
        if let TokenTree::Group(group) = token {
            if let Delimiter::Brace = group.delimiter() {
                let mut defs: Vec<TokenTree> = group.stream().into_iter().collect();

                for elem in attr.clone().into_iter() {
                    match elem {
                        TokenTree::Ident(id) => {
                            let mut def = vec![
                                TokenTree::Ident(id.clone()),
                                TokenTree::Group(Group::new(Delimiter::Brace, TokenStream::from_iter(vec![
                                    TokenTree::Ident(Ident::new("qq", id.span())),
                                    TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                                    TokenTree::Ident(Ident::new("Target", id.span())),
                                ].into_iter()))),
                                TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                            ];

                            defs.append(&mut def);
                        }

                        _ => {}
                    }
                }

                token_stream.push(TokenTree::Group(Group::new(group.delimiter(), TokenStream::from_iter(defs.into_iter()))));
            } else {
                token_stream.push(TokenTree::Group(group));
            }
        } else {
            token_stream.push(token);
        }
    }

    println!("{:#?}", token_stream);

    TokenStream::from_iter(token_stream.into_iter())
}