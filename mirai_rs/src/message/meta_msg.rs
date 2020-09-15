use crate::message::{MessageID, TimeStamp};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct MessageSource {
    pub(crate) id: MessageID,
    pub(crate) time: TimeStamp,
}

#[serde(tag = "type")]
#[derive(Debug, Deserialize, Serialize)]
pub enum MessageMeta {
    Source(MessageSource),
    Quote {
        id: MessageID
    },
}
