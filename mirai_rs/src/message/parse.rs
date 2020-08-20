#![cfg(feature = "parser")]

use crate::message::{SingleMessage, Message, MessageID};
use crate::message::message::MessageSource;

use pest::{iterators::Pairs, Parser};
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Index<'s> {
    Number(usize),
    Named(&'s str),
}

pub trait PatternComponent: Debug {
    fn apply_to(self, message: Message);
    fn clone(&self) -> Box<dyn PatternComponent>;
}

#[derive(Debug)]
pub enum Component<'idx> {
    Component(Box<dyn PatternComponent>),
    Placeholder(Index<'idx>),
}

impl Clone for Component<'_> {
    fn clone(&self) -> Self {
        match self {
            Component::Component(inner) => Component::Component((*inner).clone()),
            Component::Placeholder(idx) => Component::Placeholder(idx.clone()),
        }
    }
}

impl PatternComponent for SingleMessage {
    fn apply_to(self, mut message: Message) {
        message.message_chain.push(self);
    }

    fn clone(&self) -> Box<dyn PatternComponent> {
        Box::new(Clone::clone(self))
    }
}

impl PatternComponent for MessageID {
    fn apply_to(self, mut message: Message) {
        message.quote = Some(self)
    }

    fn clone(&self) -> Box<dyn PatternComponent> {
        Box::new(Clone::clone(self))
    }
}

impl PatternComponent for MessageSource {
    fn apply_to(self, mut message: Message) {
        message.source = self
    }

    fn clone(&self) -> Box<dyn PatternComponent> {
        Box::new(Clone::clone(self))
    }
}

#[derive(Parser)]
#[grammar = "grammar/mirai_msg.pest"]
struct MessageParser;

type Pattern<'idx> = Vec<Component<'idx>>;

#[derive(Debug, Clone)]
pub struct MessagePattern<'idx> {
    pattern: Pattern<'idx>
}

impl MessagePattern<'_> {
    pub fn compile(source: &str) -> Self {
        fn __compile(source: &str) -> Result<Pattern, pest::error::Error<Rule>> {
            let pairs: Pairs<Rule> = MessageParser::parse(Rule::template, source)?;

            for pair in pairs {
                // match pair.as_rule() {
                //     Rule::
                // }
            }

            todo!()
        }

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use pest::*;
    use super::{MessageParser, Rule};

    fn debug(rule: Rule, input: &'static str) -> &'static str {
        println!("Debugging {:#?}", input);
        println!("{:#?}", MessageParser::parse(rule, input).unwrap());
        input
    }

    #[test]
    fn test_meta() {
        parses_to! {
            parser: MessageParser,
            input: debug(Rule::meta, "#[quote:114514]"),
            rule: Rule::meta,
            tokens: [
                meta(0, 15, [
                    component(1, 15, [
                        raw_component(1, 15, [
                            inner(2, 14, [
                                String(2, 7),
                                String(8, 14)
                            ])
                        ])
                    ])
                ])
            ]
        }

        parses_to! {
            parser: MessageParser,
            input: debug(Rule::meta, "#[quote:[[[[]"),
            rule: Rule::meta,
            tokens: [
                meta(0, 13, [
                    component(1, 13, [
                        raw_component(1, 13, [
                            inner(2, 12, [
                                String(2, 7),
                                String(8, 12)
                            ])
                        ])
                    ])
                ])
            ]
        }
    }

    #[test]
    #[should_panic]
    fn failed_test_meta() {
        parses_to! {
            parser: MessageParser,
            input: "#[quote:[[[]",
            rule: Rule::meta,
            tokens: []
        }
    }

    #[test]
    fn test_holder() {
        parses_to! {
            parser: MessageParser,
            input: "{at}",
            rule: Rule::component,
            tokens: [
                component(0, 4, [
                    place_holder(0, 4, [
                        Index(1, 3, [
                            String(1, 3)
                        ])
                    ])
                ])
            ]
        }

        parses_to! {
            parser: MessageParser,
            input: "{1}",
            rule: Rule::component,
            tokens: [
                component(0, 3, [
                    place_holder(0, 3, [
                        Index(1, 2, [
                            Number(1, 2)
                        ])
                    ])
                ])
            ]
        }

        parses_to! {
            parser: MessageParser,
            input: "{}",
            rule: Rule::component,
            tokens: [
                component(0, 2, [
                    place_holder(0, 2)
                ])
            ]
        }
    }

    #[test]
    fn test_holder_meta() {
        parses_to! {
            parser: MessageParser,
            input: debug(Rule::meta, "#{quote}"),
            rule: Rule::meta,
            tokens: [
                meta(0, 8, [
                    component(1, 8, [
                        place_holder(1, 8, [
                            Index(2, 7, [
                                String(2, 7)
                            ])
                        ])
                    ])
                ])
            ]
        }

        parses_to! {
            parser: MessageParser,
            input: debug(Rule::meta, "#{12345}"),
            rule: Rule::meta,
            tokens: [
                meta(0, 8, [
                    component(1, 8, [
                        place_holder(1, 8, [
                            Index(2, 7, [
                                Number(2, 7)
                            ])
                        ])
                    ])
                ])
            ]
        }

        parses_to! {
            parser: MessageParser,
            input: debug(Rule::meta, "#{}"),
            rule: Rule::meta,
            tokens: [
                meta(0, 3, [
                    component(1, 3, [
                        place_holder(1, 3, [

                        ])
                    ])
                ])
            ]
        }
    }

    #[test]
    fn test_raw_content() {
        parses_to! {
            parser: MessageParser,
            input: debug(Rule::template, r#"#[quote:123456]
[at:111222333@qwq]
I quoted a message,
the id of which is 123456"#),
            rule: Rule::template,
            tokens: [
                template(0, 80, [
                    meta_line(0, 16, [
                        meta(0, 15, [
                            component(1, 15, [
                                raw_component(1, 15, [
                                    inner(2, 14, [
                                        String(2, 7),
                                        String(8, 14)
                                    ])
                                ])
                            ])
                        ]),
                    ]),

                    single_content(16, 34, [
                        component(16, 34, [
                            raw_component(16, 34, [
                                inner(17, 33, [
                                    String(17, 19),
                                    String(20, 33)
                                ])
                            ])
                        ])
                    ]),

                    single_content(34, 80, [
                        String(34, 80)
                    ]),

                    EOI(80, 80)
                ])
            ]
        }
    }

    #[test]
    fn test_holder_content() {
        parses_to! {
            parser: MessageParser,
            input: debug(Rule::template, r#"#{quote}
{at}
I quoted a message again,
the id of which is {id}.
I have no way to know it at compile time, because it is a HOLDER!"#),
            rule: Rule::template,
            tokens: [
                template(0, 130, [
                    meta_line(0, 9, [
                        meta(0, 8, [
                            component(1, 8, [
                                place_holder(1, 8, [
                                    Index(2, 7, [
                                        String(2, 7)
                                    ])
                                ])
                            ])
                        ])
                    ]),

                    single_content(9, 13, [
                        component(9, 13, [
                            place_holder(9, 13, [
                                Index(10, 12, [
                                    String(10, 12)
                                ])
                            ])
                        ])
                    ]),

                    single_content(13, 59, [
                        String(13, 59)
                    ]),

                    single_content(59, 63, [
                        component(59, 63, [
                            place_holder(59, 63, [
                                Index(60, 62, [
                                    String(60, 62)
                                ])
                            ])
                        ])
                    ]),

                    single_content(63, 130, [
                        String(63, 130)
                    ]),

                    EOI(130, 130)
                ])
            ]
        }
    }
}

// nom parser

// use nom::{
//     error::ErrorKind,
//     character::complete::char,
//     bytes::complete::{tag, take_while, take_until},
//     branch::alt,
//     IResult
// };
//
// use crate::message::{MessageID, SingleMessage};
// use nom::lib::std::fmt::Formatter;
// use crate::Target;
// use nom::lib::std::collections::LinkedList;
// use crate::message::single::{MessageMeta, MessageComponent};
//
// pub type Result<'s, O> = IResult<&'s str, O>;

// #[derive(Clone, Debug)]
// pub enum ErrorKind {
//     Nom(nom::error::ErrorKind),
//     NoSuchType
// }
//
// impl std::fmt::Display for ErrorKind {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ErrorKind::Nom(ne) => ne.fmt(f),
//             ErrorKind::NoSuchType => f.write_str("no such type"),
//         }
//     }
// }
//
// impl From<nom::error::ErrorKind> for ErrorKind {
//     fn from(ne: nom::error::ErrorKind) -> Self {
//         ErrorKind::Nom(ne)
//     }
// }
//
// impl <I> nom::error::ParseError<I> for (I, ErrorKind) {
//     fn from_error_kind(input: I, kind: ErrorKind) -> Self {
//         (input, kind)
//     }
//
//     fn append(_input: I, _kind: ErrorKind, other: Self) -> Self {
//         other
//     }
// }


// impl MessageElement for Quote {
//     fn into_message(self) -> SingleMessage {
//         SingleMessage::Quote {
//             id: self.0,
//             group_id: 0,
//             sender_id: 0,
//             target_id: 0,
//             origin: vec![]
//         }
//     }
// }


// const LB: char = '[';
// const RB: char = ']';
// const COLON: char = ':';
// const COMMA: char = ',';
// const SPACE: char = ' ';
//
// fn spaces(input: &str) -> Result<&str> {
//     take_while(|it| it == SPACE)(input)
// }
//
// /// ## Example
// ///
// /// ```rust
// /// use mirai::message::parse::{meta, Component};
// /// use mirai::message::{SingleMessage, MessageID};
// /// use mirai::message::single::MessageMeta;
// ///
// /// fn quote<'a>(id: MessageID) -> Component<'a> {
// ///     Component::Single(MessageMeta::Quote { id }.into())
// /// }
// ///
// /// let input = r#"#[quote:123, quote:456]"#;
// /// assert_eq!(meta(input).map(|(rest, it)| (rest, it.collect::<Vec<Component>>())), Ok(("", vec![quote(123), quote(456)])));
// /// ```
// pub fn meta(input: &str) -> Result<impl Iterator<Item = Component>> {
//     /// this will parse:
//     ///
//     /// ```ignore
//     /// #[quote:123, quote: 456]
//     ///   ^~~~~~~~^
//     /// ```
//     /// wont eat the delimited character
//     fn try_meta(input: &str) -> Result<Component> {
//         let (input, ty) = take_while(|it| it != COLON)(input)?;
//         let (input, _) = char(COLON)(input)?;
//         let (input, arg) = take_while(|it| it != COMMA && it != RB)(input)?;
//
//         let data = match ty {
//             "quote" => {
//                 Component::Single(MessageMeta::Quote { id: arg.parse().map_err(|_| nom::Err::Failure((arg, ErrorKind::ParseTo)))? }.into())
//             }
//
//             _ => Err(nom::Err::Failure((ty, ErrorKind::ParseTo)))?
//         };
//
//         Ok((input, data))
//     }
//
//     let (input, _) = char('#')(input)?;
//     let (input, _) = char(LB)(input)?;
//
//     /// this will parse:
//     ///
//     /// ```ignore
//     /// #[quote:123, quote:456]
//     ///   ^~~~~~~~~~~~~~~~~~~~^
//     /// ```
//     ///
//     /// will eat ']'
//     fn parse_all_meta(input: &str) -> Result<LinkedList<Component>> {
//         let (input, _) = spaces(input)?;
//         let (input, meta) = try_meta(input)?;
//
//         let (input, _) = spaces(input)?;
//         let (input, delimited) = alt((char(COMMA), char(RB)))(input)?;
//
//         match delimited {
//             COMMA => {
//                 let (input, _) = spaces(input)?;
//                 let (input, mut rest) = parse_all_meta(input)?;
//                 rest.push_front(meta);
//                 Ok((input, rest))
//             },
//             RB => {
//                 let mut init = LinkedList::new();
//                 init.push_front(meta);
//
//                 Ok((input, init))
//             },
//             _ => unreachable!()
//         }
//     }
//
//     let (input, inner) = parse_all_meta(input)?;
//     Ok((input, inner.into_iter()))
// }

// rubbish parser

// use std::str::Chars;
//
// use crate::Target;
// use std::iter::{Peekable, SkipWhile};
// use crate::message::{SingleMessage, Message, MessageChain};
// use std::collections::HashMap;
// use serde::export::Formatter;
// use std::ops::Deref;
//
// #[derive(Clone)]
// pub enum Index {
//     Number(usize),
//     String(String),
//     Default,
// }
//
// #[derive(Clone)]
// pub enum Component {
//     At {
//         target: Target,
//         display: String,
//     },
//     RawString {
//         raw: String
//     },
//     Placeholder {
//         index: Index
//     },
// }
//
//
// pub type Result<'a, T> = std::result::Result<T, FormatError<'a>>;
//
// #[derive(Debug)]
// pub struct FormatError<'a> {
//     kind: ErrorKind<'a>
// }
//
// impl<'a> FormatError<'a> {
//     pub fn missing_name(name: &'a str) -> FormatError<'a> {
//         FormatError {
//             kind: ErrorKind::MissingName(name)
//         }
//     }
//
//     pub fn missing_index(index: usize) -> FormatError<'static> {
//         FormatError {
//             kind: ErrorKind::MissingIndex(index)
//         }
//     }
// }
//
// impl<'a> std::fmt::Display for FormatError<'a> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let msg = match self.kind {
//             ErrorKind::MissingName(name) => {
//                 format!(r#"Missing argument with name "{}""#, name)
//             },
//             ErrorKind::MissingIndex(index) => {
//                 format!(r#"Missing argument with index {}"#, index)
//             },
//         };
//
//         f.write_str(&msg)
//     }
// }
//
// #[derive(Debug)]
// pub enum ErrorKind<'a> {
//     MissingName(&'a str),
//     MissingIndex(usize),
// }
//
// #[derive(Clone)]
// pub struct Pattern {
//     pub components: Vec<Component>
// }
//
// /// A message pattern which can receive some arguments to do something
// /// ## Example
// /// ```rust
// /// use mirai::message::parse::{Pattern, Index};
// /// use mirai::message::parse::Component::*;
// /// use mirai::message::SingleMessage;
// /// use std::collections::HashMap;
// /// use std::iter::FromIterator;
// ///
// /// let pat = Pattern {
// ///     components: vec![RawString { raw: "Hello ".to_string() }, Placeholder { index: Index::String("at".to_string()) }, RawString { raw: " !".to_string() }]
// /// };
// ///
// /// pat.format(&[], &HashMap::from_iter(vec![("at", SingleMessage::At { target: 1005042620, display: "hoshino".to_string() })].into_iter()));
// /// ```
// impl Pattern {
//     pub fn compile<AsStr: AsRef<str>>(source: AsStr) -> Pattern {
//         let source = source.as_ref();
//         unimplemented!()
//     }
//
//     pub fn format(&self, args: &[SingleMessage], named: &HashMap<&str, SingleMessage>) -> Result<Message> {
//         let mut default_index = 0usize;
//         let result = self.components.iter().map(|comp| {
//             let comp = match comp {
//                 Component::At { target, display } => SingleMessage::At { target: *target, display: display.clone() },
//                 Component::RawString { raw } => SingleMessage::Plain { text: raw.clone() },
//                 Component::Placeholder { index } => match index {
//                     Index::Number(index) => {
//                         args.get(*index).ok_or(FormatError::missing_index(*index))?.clone()
//                     },
//                     Index::String(name) => {
//                         named.get(name.as_str()).ok_or(FormatError::missing_name(name.as_str()))?.clone()
//                     },
//                     Index::Default => {
//                         let msg = args.get(default_index).ok_or(FormatError::missing_index(default_index))?.clone();
//                         default_index += 1;
//                         msg
//                     },
//
//                     _ => panic!("qaq")
//                 },
//             };
//
//             Ok(comp)
//         });
//
//         Ok(Message::new(None, result.collect::<Result<MessageChain>>()?))
//     }
// }
//
// /// ## Example
// ///
// /// ```rust
// /// use mirai::message::parse::TextIter;
// /// use std::str::Chars;
// ///
// /// let text = r#"12
// /// 1"#;
// /// let mut iter: TextIter<Chars> = text.chars().into();
// ///
// /// assert_eq!((1, 0), (iter.line(), iter.column()));
// /// assert_eq!(Some('1'), iter.next());
// /// assert_eq!((1, 1), (iter.line(), iter.column()));
// /// assert_eq!(Some('2'), iter.next());
// /// assert_eq!((1, 2), (iter.line(), iter.column()));
// /// assert_eq!(Some('\n'), iter.next());
// /// assert_eq!((2, 0), (iter.line(), iter.column()));
// /// assert_eq!(Some('1'), iter.next());
// /// assert_eq!((2, 1), (iter.line(), iter.column()));
// /// assert_eq!(None, iter.next());
// /// assert_eq!((2, 1), (iter.line(), iter.column()));
// /// ```
// pub struct TextIter<I> {
//     inner: I,
//     line: usize,
//     column: usize,
// }
//
// impl<I> TextIter<I> {
//     pub fn line(&self) -> usize {
//         self.line
//     }
//
//     pub fn column(&self) -> usize {
//         self.column
//     }
//
//     pub fn metadata(&self) -> MetaData {
//         MetaData {
//             line: self.line,
//             column: self.column,
//         }
//     }
// }
//
// impl<I: Iterator> From<I> for TextIter<I> {
//     fn from(iter: I) -> Self {
//         TextIter {
//             inner: iter,
//             line: 1,
//             column: 0,
//         }
//     }
// }
//
// impl<I: Iterator<Item=char>> Iterator for TextIter<I> {
//     type Item = char;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         let next = self.inner.next();
//
//         if let Some(next) = next {
//             match next {
//                 '\n' => {
//                     self.column = 0;
//                     self.line += 1;
//                 }
//
//                 _ => {
//                     self.column += 1;
//                 }
//             }
//         }
//
//         next
//     }
// }
//
// /// ## Example
// /// ```rust
// /// use mirai::message::parse::TextIter;
// /// let str = r#"   456"#;
// /// let mut it = TextIter::from(str.chars());
// /// assert_eq!(Some('4'), it.next_not_white());
// /// assert_eq!(Some('5'), it.next_not_white());
// /// assert_eq!(Some('6'), it.next_not_white());
// /// assert_eq!(None, it.next_not_white());
// /// ```
// impl<I: Iterator<Item=char>> TextIter<I> {
//     pub fn next_not_white(&mut self) -> Option<(I::Item)> {
//         self.find(|it| !it.is_ascii_whitespace())
//     }
// }
//
// /// ## Example
// /// ```rust
// /// use mirai::message::parse::TextIter;
// ///
// /// let str = r#"123"#;
// /// let mut it = TextIter::from(str.chars().peekable());
// /// assert_eq!(Some(&'1'), it.peek());
// /// assert_eq!((1, 0), (it.line(), it.column()));
// /// assert_eq!(Some('1'), it.next());
// /// assert_eq!((1, 1), (it.line(), it.column()));
// /// ```
// impl<I: Iterator> TextIter<Peekable<I>> {
//     pub fn peek(&mut self) -> Option<&I::Item> {
//         self.inner.peek()
//     }
// }
//
// pub type Tokens = Vec<Token>;
//
// pub enum Token {
//     RawString(String),
//     // '[' and ']'
//     LB,
//     RB,
//
//     // '[[' and ']]'
//     DLB,
//     DRB,
//     // 123
//     Number(usize),
//     // -? 123
//     NegNumber(isize),
//     // #
//     Meta,
//     // :
//     Colon,
// }
//
// impl Token {
//     pub const LB: char = '[';
//     pub const RB: char = ']';
//     pub const META: char = '#';
//     pub const COLON: char = ':';
// }
//
// #[derive(Debug, Clone, Copy)]
// pub struct MetaData {
//     line: usize,
//     column: usize,
// }
//
// impl MetaData {
//     pub fn line(&self) -> usize {
//         self.line
//     }
//
//     pub fn column(&self) -> usize {
//         self.column
//     }
// }
//
// struct Lexer;
//
// impl Lexer {
//     pub fn lex_meta(iter: &mut TextIter<Chars>) -> Tokens {
//         todo!()
//     }
//
//     pub fn lex_lb(iter: &mut TextIter<chars>) -> Token {
//         todo!()
//     }
//
//     pub fn lex(mut iter: TextIter<Peekable<Chars>>) -> Tokens {
//         let tokens = Tokens::new();
//
//         let start = iter.peek();
//
//         if let Some(start) = start {
//             if let &Token::META = start {
//                 iter.next();
//
//                 if let &Token::LB = iter.peek() {} else {}
//             }
//         }
//
//         tokens
//     }
// }