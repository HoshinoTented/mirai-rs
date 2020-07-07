use crate::message::{EventPacket, GroupMember, Group, SingleMessage};
use crate::message::event::MessageEvent;
use crate::message::element::Permission;
use crate::error::Result;

pub struct Mock;

impl Mock {
    pub fn receive_message() -> Result<Vec<EventPacket>> {
        Ok(vec![EventPacket::MessageEvent(
            MessageEvent::GroupMessage {
                message_chain: vec![
                    SingleMessage::Source { id: 1, time: 1 },
                    SingleMessage::Plain { text: "Hello, world!".to_string() }
                ],
                sender: GroupMember {
                    id: 123456,
                    member_name: "Hoshino Tented".to_string(),
                    permission: Permission::Administrator,
                    group: Group {
                        id: 456789,
                        name: "Hoshino Garden".to_string(),
                        permission: Permission::Member,
                    },
                },
            }
        )])
    }
}