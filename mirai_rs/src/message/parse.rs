#![cfg(feature = "parser")]

use std::str::Chars;

use crate::Target;
use std::iter::{Peekable, SkipWhile};
use crate::message::{SingleMessage, Message, MessageChain};
use std::collections::HashMap;
use serde::export::Formatter;
use std::ops::Deref;

#[derive(Clone)]
pub enum Index {
    Number(usize),
    String(String),
    Default,
}

#[derive(Clone)]
pub enum Component {
    At {
        target: Target,
        display: String,
    },
    RawString {
        raw: String
    },
    Placeholder {
        index: Index
    },
}


pub type Result<'a, T> = std::result::Result<T, FormatError<'a>>;

#[derive(Debug)]
pub struct FormatError<'a> {
    kind: ErrorKind<'a>
}

impl<'a> FormatError<'a> {
    pub fn missing_name(name: &'a str) -> FormatError<'a> {
        FormatError {
            kind: ErrorKind::MissingName(name)
        }
    }

    pub fn missing_index(index: usize) -> FormatError<'static> {
        FormatError {
            kind: ErrorKind::MissingIndex(index)
        }
    }
}

impl<'a> std::fmt::Display for FormatError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = match self.kind {
            ErrorKind::MissingName(name) => {
                format!(r#"Missing argument with name "{}""#, name)
            },
            ErrorKind::MissingIndex(index) => {
                format!(r#"Missing argument with index {}"#, index)
            },
        };

        f.write_str(&msg)
    }
}

#[derive(Debug)]
pub enum ErrorKind<'a> {
    MissingName(&'a str),
    MissingIndex(usize),
}

#[derive(Clone)]
pub struct Pattern {
    pub components: Vec<Component>
}

/// A message pattern which can receive some arguments to do something
/// ## Example
/// ```rust
/// use mirai::message::parse::{Pattern, Index};
/// use mirai::message::parse::Component::*;
/// use mirai::message::SingleMessage;
/// use std::collections::HashMap;
/// use std::iter::FromIterator;
///
/// let pat = Pattern {
///     components: vec![RawString { raw: "Hello ".to_string() }, Placeholder { index: Index::String("at".to_string()) }, RawString { raw: " !".to_string() }]
/// };
///
/// pat.format(&[], &HashMap::from_iter(vec![("at", SingleMessage::At { target: 1005042620, display: "hoshino".to_string() })].into_iter()));
/// ```
impl Pattern {
    pub fn compile<AsStr: AsRef<str>>(source: AsStr) -> Pattern {
        let source = source.as_ref();
        unimplemented!()
    }

    pub fn format(&self, args: &[SingleMessage], named: &HashMap<&str, SingleMessage>) -> Result<Message> {
        let mut default_index = 0usize;
        let result = self.components.iter().map(|comp| {
            let comp = match comp {
                Component::At { target, display } => SingleMessage::At { target: *target, display: display.clone() },
                Component::RawString { raw } => SingleMessage::Plain { text: raw.clone() },
                Component::Placeholder { index } => match index {
                    Index::Number(index) => {
                        args.get(*index).ok_or(FormatError::missing_index(*index))?.clone()
                    },
                    Index::String(name) => {
                        named.get(name.as_str()).ok_or(FormatError::missing_name(name.as_str()))?.clone()
                    },
                    Index::Default => {
                        let msg = args.get(default_index).ok_or(FormatError::missing_index(default_index))?.clone();
                        default_index += 1;
                        msg
                    },

                    _ => panic!("qaq")
                },
            };

            Ok(comp)
        });

        Ok(Message::new(None, result.collect::<Result<MessageChain>>()?))
    }
}

/// ## Example
///
/// ```rust
/// use mirai::message::parse::TextIter;
/// use std::str::Chars;
///
/// let text = r#"12
/// 1"#;
/// let mut iter: TextIter<Chars> = text.chars().into();
///
/// assert_eq!((1, 0), (iter.line(), iter.column()));
/// assert_eq!(Some('1'), iter.next());
/// assert_eq!((1, 1), (iter.line(), iter.column()));
/// assert_eq!(Some('2'), iter.next());
/// assert_eq!((1, 2), (iter.line(), iter.column()));
/// assert_eq!(Some('\n'), iter.next());
/// assert_eq!((2, 0), (iter.line(), iter.column()));
/// assert_eq!(Some('1'), iter.next());
/// assert_eq!((2, 1), (iter.line(), iter.column()));
/// assert_eq!(None, iter.next());
/// assert_eq!((2, 1), (iter.line(), iter.column()));
/// ```
pub struct TextIter<I> {
    inner: I,
    line: usize,
    column: usize,
}

impl<I> TextIter<I> {
    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn metadata(&self) -> MetaData {
        MetaData {
            line: self.line,
            column: self.column,
        }
    }
}

impl<I: Iterator> From<I> for TextIter<I> {
    fn from(iter: I) -> Self {
        TextIter {
            inner: iter,
            line: 1,
            column: 0,
        }
    }
}

impl<I: Iterator<Item=char>> Iterator for TextIter<I> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next();

        if let Some(next) = next {
            match next {
                '\n' => {
                    self.column = 0;
                    self.line += 1;
                }

                _ => {
                    self.column += 1;
                }
            }
        }

        next
    }
}

/// ## Example
/// ```rust
/// use mirai::message::parse::TextIter;
/// let str = r#"   456"#;
/// let mut it = TextIter::from(str.chars());
/// assert_eq!(Some('4'), it.next_not_white());
/// assert_eq!(Some('5'), it.next_not_white());
/// assert_eq!(Some('6'), it.next_not_white());
/// assert_eq!(None, it.next_not_white());
/// ```
impl<I: Iterator<Item=char>> TextIter<I> {
    pub fn next_not_white(&mut self) -> Option<(I::Item)> {
        self.find(|it| !it.is_ascii_whitespace())
    }
}

/// ## Example
/// ```rust
/// use mirai::message::parse::TextIter;
///
/// let str = r#"123"#;
/// let mut it = TextIter::from(str.chars().peekable());
/// assert_eq!(Some(&'1'), it.peek());
/// assert_eq!((1, 0), (it.line(), it.column()));
/// assert_eq!(Some('1'), it.next());
/// assert_eq!((1, 1), (it.line(), it.column()));
/// ```
impl<I: Iterator> TextIter<Peekable<I>> {
    pub fn peek(&mut self) -> Option<&I::Item> {
        self.inner.peek()
    }
}

pub type Tokens = Vec<Token>;

pub enum Token {
    RawString(String),
    // '[' and ']'
    LB,
    RB,

    // '[[' and ']]'
    DLB,
    DRB,
    // 123
    Number(usize),
    // -? 123
    NegNumber(isize),
    // #
    Meta,
    // :
    Colon,
}

impl Token {
    pub const LB: char = '[';
    pub const RB: char = ']';
    pub const META: char = '#';
    pub const COLON: char = ':';
}

#[derive(Debug, Clone, Copy)]
pub struct MetaData {
    line: usize,
    column: usize,
}

impl MetaData {
    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

struct Lexer;

impl Lexer {
    pub fn lex_meta(iter: &mut TextIter<Chars>) -> Tokens {
        todo!()
    }

    pub fn lex_lb(iter: &mut TextIter<chars>) -> Token {
        todo!()
    }

    pub fn lex(mut iter: TextIter<Peekable<Chars>>) -> Tokens {
        let tokens = Tokens::new();

        let start = iter.peek();

        if let Some(start) = start {
            if let &Token::META = start {
                iter.next();

                if let &Token::LB = iter.peek() {} else {}
            }
        }

        tokens
    }
}