use crate::message::SingleMessage;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Visitor, SeqAccess};
use nom::lib::std::fmt::Formatter;
use serde::ser::SerializeSeq;

pub type MessageChain = Vec<SingleMessage>;
pub type MessageID = i64;
pub type TimeStamp = u64;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct MessageSource {
    id: MessageID,
    time: TimeStamp,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Message {
    pub(crate) source: MessageSource,
    pub(crate) quote: Option<MessageID>,
    pub(crate) message_chain: MessageChain,
}

impl Message {
    pub fn new<I: Into<MessageChain>>(message_chain: I) -> Message {
        Message {
            source: MessageSource {
                id: -114514,
                time: 1919810,
            },
            quote: None,
            message_chain: message_chain.into(),
        }
    }

    pub fn quote(&mut self, quote: MessageID) {
        self.quote = Some(quote);
    }


    pub fn source(&self) -> &MessageSource {
        &self.source
    }

    pub fn quoted(&self) -> Option<MessageID> {
        self.quote
    }

    pub fn message_chain(&self) -> &MessageChain {
        &self.message_chain
    }
}

impl From<SingleMessage> for Message {
    fn from(single: SingleMessage) -> Self {
        Message::new([single])
    }
}

#[serde(tag = "type")]
#[derive(Debug, Deserialize, Serialize)]
enum MessageMeta {
    Source(MessageSource),
    Quote {
        id: MessageID
    },
}

#[serde(untagged)]
#[derive(Deserialize, Serialize)]
enum MessageComponent {
    Meta(MessageMeta),
    Single(SingleMessage),
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
        D: Deserializer<'de> {
        struct MessageVisitor;

        impl<'v> Visitor<'v> for MessageVisitor {
            type Value = Message;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "Message is expecting a map")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, <A as SeqAccess<'v>>::Error> where
                A: SeqAccess<'v>, {
                let mut source = None;
                let mut quote = None;
                let mut chain = Vec::new();

                while let Some(next) = seq.next_element::<MessageComponent>()? {
                    match next {
                        MessageComponent::Meta(meta) => {
                            match meta {
                                MessageMeta::Source(ms) => {
                                    source = Some(ms);
                                }
                                MessageMeta::Quote { id } => {
                                    quote = Some(id);
                                }
                            }
                        }
                        MessageComponent::Single(single) => {
                            chain.push(single);
                        }
                    }
                }

                use serde::de::Error;

                Ok(Message {
                    source: source.ok_or(A::Error::custom("expecting a source but got nothing"))?,
                    quote,
                    message_chain: chain,
                })
            }
        }

        deserializer.deserialize_seq(MessageVisitor)
    }
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> where
        S: Serializer {
        let len = 1 + self.message_chain.len() + self.quote.is_some() as usize;
        let mut seq = serializer.serialize_seq(Some(len))?;

        seq.serialize_element(&Some(MessageComponent::Meta(MessageMeta::Source(self.source.clone()))))?;

        if let Some(quote) = self.quote {
            let quote = MessageComponent::Meta(MessageMeta::Quote { id: quote });
            seq.serialize_element(&Some(quote))?;
        }

        for single in self.message_chain.clone().into_iter() {
            seq.serialize_element(&MessageComponent::Single(single))?;
        }

        seq.end()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::*;
    use crate::message::{Message, SingleMessage};
    use crate::message::message::MessageSource;

    #[test]
    fn test_message_serde() {
        let source = json! {[
            {
                "type": "Source",
                "id": -114514,
                "time": 1919810
            },
            {
                "type": "Quote",
                "id": 19260817
            },
            {
                "type": "At",
                "target": 1005042620,
                "display": "世界第一可爱星野酱"
            },
            {
                "type": "Plain",
                "text": "Hoshino Chan! I am your fan desu!"
            }
        ]};

        let expect = Message {
            source: MessageSource {
                id: -114514,
                time: 1919810,
            },
            quote: Some(19260817),
            message_chain: vec![
                SingleMessage::At {
                    target: 1005042620,
                    display: "世界第一可爱星野酱".to_string(),
                },
                "Hoshino Chan! I am your fan desu!".into()
            ],
        };

        assert_eq!(serde_json::from_value::<Message>(source.clone()).unwrap(), expect);
        assert_eq!(serde_json::to_value(expect.clone()).unwrap(), source);
    }
}