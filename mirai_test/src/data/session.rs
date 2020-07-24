use mirai::session::{AboutResponse, AboutData};

pub struct About;

impl About {
    pub fn response() -> AboutResponse {
        AboutResponse::new(
            0,
            AboutData::new("mirai_rs_mock".to_string()),
        )
    }
}