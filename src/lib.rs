pub mod session;
pub mod error;
pub mod message;

pub type Target = u64;
pub type Code = u16;

#[cfg(test)]
mod tests {
    use crate::message::{MessagePackage, GroupSender, Group};
    use serde_json;

    #[test]
    fn message() {
        let msg = r#"{
      "type": "GroupMessage",
      "messageChain": [
        {
          "type": "Source",
          "id": 123456,
          "time": 123456789
        },
        {
          "type": "Plain",
          "text": "Miral牛逼"
        }
      ],
      "sender": {
          "id": 123456789,
          "memberName": "化腾",
          "permission": "MEMBER",
          "group": {
              "id": 1234567890,
              "name": "Miral Technology",
              "permission": "MEMBER"
          }
      }
    }"#;

        let json = serde_json::from_str::<MessagePackage>(msg);
        println!("{:?}", json);
    }
}