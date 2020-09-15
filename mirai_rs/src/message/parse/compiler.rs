use std::fmt::{Debug, Display, Formatter};

use pest::error::Error;
use pest::iterators::{Pair, Pairs};

use crate::message::{Message, MessageContent, MessageID};
use crate::message::message::MessageSource;

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

impl PatternComponent for MessageSource {
    fn apply_to(self, mut message: Message) {
        message.source = self
    }

    fn clone(&self) -> Box<dyn PatternComponent> {
        Box::new(Clone::clone(self))
    }
}

impl PatternComponent for MessageContent {
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

#[derive(Debug)]
pub enum CompileError<'tg> {
    Lexical(pest::error::Error<Rule>),
    Semantic {
        expect: Vec<Rule>,
        got: Rule,
    },
    Type {
        expect: Vec<&'static str>,
        got: &'tg str,
    },
    Custom(String),
}

impl<'tg> Display for CompileError<'tg> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::Lexical(e) => write!(f, "lexical error: {:?}", e),
            CompileError::Semantic { expect, got } => write!(f, "semantic error: expected '{:?}', but got '{:?}'", expect, got),
            CompileError::Type { expect, got } => write!(f, "type error: expected '{:?}', but got '{:?}'", expect, got),
            CompileError::Custom(msg) => write!(f, "custom error: {}", msg)
        }
    }
}

impl From<Error<Rule>> for CompileError<'_> {
    fn from(e: Error<Rule>) -> Self {
        CompileError::Lexical(e)
    }
}

type CompileResult<'e, R> = Result<R, CompileError<'e>>;
type Pattern<'idx> = Vec<Component<'idx>>;

#[derive(Debug, Clone)]
pub struct MessagePattern<'idx> {
    pattern: Pattern<'idx>
}

impl<'s> MessagePattern<'s> {
    pub fn compile(source: &'s str) -> CompileResult<'s, Self> {
        fn __compile(source: &str) -> CompileResult<Pattern> {
            fn semantic_err<R, RS: Into<Vec<Rule>>>(expected: RS, got: Rule) -> CompileResult<'static, R> {
                Err(CompileError::Semantic {
                    expect: expected.into(),
                    got,
                })
            }

            fn type_err<R, TS: Into<Vec<&'static str>>>(expected: TS, got: &str) -> CompileResult<R> {
                Err(CompileError::Type {
                    expect: expected.into(),
                    got,
                })
            }

            fn solve_meta<'s>(name: &'s str, value: &'s str) -> CompileResult<'s, Box<dyn PatternComponent>> {
                match name {
                    "quote" => {
                        let value = value.parse::<MessageID>().map_err(|e| CompileError::Custom(format!("{:?}", e)))?;

                        Ok(Box::new(value))
                    },
                    otherwise => type_err(["quote"], otherwise)?
                }
            }

            fn solve_meta_line(item: Pair<Rule>) -> CompileResult<Component<'_>> {
                let mut meta_line = item.into_inner();
                let mut meta = meta_line.next().expect("internal error: expecting a 'meta' at the first of 'meta_line'").into_inner();
                let mut component = meta.next().expect("internal error: expecting a 'component' at the first of 'meta'").into_inner();
                let comp_inner = component.next().expect("internal error: expecting [raw_component, place_holder] at the first of 'component'");

                match comp_inner.as_rule() {
                    Rule::raw_component => {
                        let mut raw_component = comp_inner.into_inner();
                        let mut inner = raw_component.next().expect("internal error: expecting 'inner' at the first of 'raw_component'").into_inner();
                        let name = inner.next().expect("internal error: expecting 'String' at the first of 'raw_component'");
                        let value = inner.next().expect("internal error: expecting 'String' at the second of 'raw_component'");

                        let solved_meta = solve_meta(name.as_str(), value.as_str())?;

                        Ok(Component::Component(solved_meta))
                    },

                    _ => panic!("internal error: expecting [raw_component, place_holder] at the first of 'component'")
                }
            }

            let mut pairs: Pairs<Rule> = MessageParser::parse(Rule::template, source)?;

            let first = pairs.next().ok_or(CompileError::Semantic { expect: vec![Rule::template], got: Rule::EOI })?;

            if let Rule::template = first.as_rule() {
                let mut comps = Vec::new();
                let inner = first.into_inner();

                for item in inner {
                    match item.as_rule() {
                        Rule::meta_line => {
                            let meta = solve_meta_line(item)?;
                            comps.push(meta);
                        }

                        Rule::EOI => break,

                        otherwise => semantic_err([Rule::meta_line, Rule::single_content], otherwise)?
                    }
                }

                Ok(comps)
            } else {
                Err(
                    CompileError::Semantic {
                        expect: vec![Rule::template],
                        got: first.as_rule(),
                    }
                )
            }
        }

        Ok(MessagePattern {
            pattern: __compile(source)?
        })
    }
}
