use crate::message::{MessageChain, SingleMessage, MessageID, Message};

use regex::{Regex, Captures};

use serde::export::{PhantomData, Formatter};
use std::str::Chars;

use lazy_static::lazy_static;

pub trait MessageParser<O> {
    fn parse(self) -> std::result::Result<O, ParseError>;
}

pub struct TextIterator<'a, I> {
    inner: I,
    line: usize,
    column: usize,
    _marker: PhantomData<&'a I>,
}

impl<'a, I: 'a> TextIterator<'a, I> {
    pub fn new(iter: I) -> TextIterator<'a, I> {
        TextIterator {
            inner: iter,
            line: 1,
            column: 0,
            _marker: PhantomData,
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn occur_error(&self, msg: String) -> ParseError {
        ParseError {
            msg,
            line: self.line,
            column: self.column,
        }
    }
}

impl<I> Iterator for TextIterator<'_, I>
    where I: Iterator<Item=char> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.inner.next();

        if let Some(c) = result {
            match c {
                '\n' => {
                    self.column = 0;
                    self.line += 1;
                }

                _ => {
                    self.column += 1;
                }
            }
        }

        result
    }
}

type Result<O> = std::result::Result<O, ParseError>;

#[derive(Debug)]
pub struct ParseError {
    msg: String,
    line: usize,
    column: usize,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Parsing error! ({}, {}): {}", self.line, self.column, self.msg))
    }
}

impl std::error::Error for ParseError {}

pub struct StringMessageParser {
    source: String
}

type Iter<'a> = TextIterator<'a, Chars<'a>>;

pub struct MetaData {
    quote: MessageID
}

impl StringMessageParser {
    pub fn new<S: Into<String>>(source: S) -> StringMessageParser {
        StringMessageParser {
            source: source.into()
        }
    }

    /// Parsing metadata which is in the start of the text.
    ///
    /// ## Example
    /// ```ignore
    /// let meta = r#"#[123:abc, 456:def]"#;
    /// capture[0] = "#[123:abc, 456:def]";
    /// capture[1] = "123:abc, 456:def";
    /// capture[2] = "123:abc";     // which is meaningless
    /// ```
    pub fn parse_meta(it: &mut Iter) -> Result<MetaData> {
        lazy_static! {
            static ref META: String = String::from("[^:]+:[^,]+");
            static ref META_RE: Regex = Regex::new(META.as_str()).unwrap();

            static ref METAARG: String = format!("({meta},|{meta})*", meta = *META);
            static ref METAS: String = format!("#[({metas})]", metas = *METAARG);

            static ref METAS_RE: Regex = Regex::new(&format!("^({})?", *METAS)).unwrap();
        }

        todo!()
    }

    /// This function should be called after a "[at:", and it will parse and eat chars which is not a ']' (but it also eat ']')
    ///
    /// # Example
    /// ```rust
    /// use mirai::message::parse::{StringMessageParser, TextIterator};
    /// use mirai::message::SingleMessage;
    ///
    /// let good = "1005042620@hoshinokawaii]";
    /// let bad = "qwq@123]";
    ///
    /// assert_eq!(Some(SingleMessage::At {
    ///     target: 1005042620,
    ///     display: "hoshinokawaii".to_string()
    /// }), StringMessageParser::parse_at(&mut TextIterator::new(good.chars())).ok());
    ///
    /// assert_eq!(None, StringMessageParser::parse_at(&mut TextIterator::new(bad.chars())).ok());
    /// ```
    pub fn parse_at(it: &mut Iter) -> Result<SingleMessage> {
        lazy_static! {
            static ref R: Regex = Regex::new(r"^(\d+)@(.*)$").unwrap();
        }

        let inside: String = it.by_ref().take_while(|char| *char != ']').collect();
        let cpt: Option<Captures> = R.captures(&inside);

        match cpt {
            Some(cpt) => {
                let target = &cpt[1];
                let display = &cpt[2];

                Ok(SingleMessage::At {
                    target: target.parse().map_err(|_| it.occur_error(format!("Cannot parse {} as a Target", target)))?,
                    display: display.to_string(),
                })
            }

            None => {
                Err(it.occur_error(format!("Cannot match {} as an AT.", inside)))
            }
        }
    }
}

impl MessageParser<Message> for StringMessageParser {
    /// Parse a string message with the following format:
    /// ```ignore
    /// ESCAPE := '\\' .?;           # parser will parse '\x' as a raw character 'x', even though it is not a special character or is a `EOF`
    /// CHAR := [^{}] | ESCAPE;
    /// EOL := '\n';
    ///
    /// STRING := CHAR *;
    ///
    /// TARGET := \d+
    ///
    /// QUOTE :=
    ///
    /// SINGLE_META :=
    ///
    /// AT := ('at' | 'AT') ':' TARGET '@' STRING;
    ///
    /// COMPONENT := '[' (AT) ']';
    /// ```
    ///
    /// ## Example
    ///
    /// ```rust
    /// use mirai::message::parse::{StringMessageParser, MessageParser, parse_msg};
    /// use mirai::message::SingleMessage;
    ///
    /// let msg = r#"[at:1005042620@hoshino] Hello, world!"#;
    /// let parser = parse_msg(msg.to_string());
    ///
    /// assert_eq!(Some(vec![SingleMessage::At { target: 1005042620, display: String::from("hoshino") }, " Hello, world!".into()]), parser.parse().ok().map(|it| it.message_chain));
    ///
    /// let msg = r#"]"#;
    /// let parser = parse_msg(msg.to_string());
    ///
    /// assert_eq!(None, parser.parse().ok());
    ///
    /// let msg = r#"\[at:1005042620@hoshino\] Hello, world!"#;
    /// let parser = parse_msg(msg.to_string());
    ///
    /// assert_eq!(Some(vec!["[at:1005042620@hoshino] Hello, world!".into()]), parser.parse().ok().map(|it| it.message_chain));
    /// ```
    fn parse(self) -> Result<Message> {
        let mut chain: Vec<SingleMessage> = Vec::new();

        let mut cs: Iter = TextIterator::new(self.source.chars());
        let mut tmp = String::new();

        while let Some(char) = cs.next() {
            match char {
                '[' => {
                    if !tmp.is_empty() {
                        chain.push(tmp.into());
                        tmp = String::new();
                    }

                    // type
                    let t = cs.by_ref().take_while(|it| *it != ':').collect::<String>();

                    let component = match t.as_str() {
                        "at" | "AT" => StringMessageParser::parse_at(&mut cs)?,
                        _ => {
                            return Err(cs.occur_error(format!("Unknown message type: {}", t)));
                        }
                    };

                    chain.push(component);
                }

                ']' => {
                    return Err(cs.occur_error("Unexpected ']'".to_string()));
                }

                '\\' => {
                    if let Some(raw) = cs.next() {
                        tmp.push(raw);
                    }
                }

                otherwise => {
                    tmp.push(otherwise);
                }
            }
        }

        if !tmp.is_empty() {
            chain.push(tmp.into());
        }

        Ok(Message::new(None, chain))
    }
}

pub fn parse_msg(msg: String) -> StringMessageParser {
    StringMessageParser::new(msg)
}