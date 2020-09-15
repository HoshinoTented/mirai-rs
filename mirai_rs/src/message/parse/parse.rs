use crate::message::{MessageContent, Message, MessageID};
use crate::message::message::MessageSource;

use pest::{iterators::Pairs, Parser};
use std::fmt::{Debug, Display};
use pest::error::Error;
use nom::lib::std::fmt::Formatter;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "grammar/mirai_msg.pest"]
struct MessageParser;

#[cfg(test)]
mod tests {
    use pest::*;
    use super::{MessageParser, Rule};
    use crate::message::parse::MessagePattern;

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

    #[test]
    fn test_compile() {
        let source = r#"#[quote:123456]
"#;

        debug(Rule::template, source);

        let pattern = MessagePattern::compile(source);
        println!("{:?}", pattern);
    }
}