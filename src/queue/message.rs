#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub body: String,
    #[serde(skip_serializing_if = "Option::is_none")] pub delay: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")] pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub reserved_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")] pub reservation_id: Option<String>,
}

impl Message {
    pub fn new(body: &str, delay: u32) -> Message {
        Message {
            body: String::from(body),
            delay: Some(delay),
            id: None,
            reserved_count: None,
            reservation_id: None,
        }
    }

    pub fn with_body(body: &str) -> Message {
        Message {
            body: String::from(body),
            delay: None,
            id: None,
            reserved_count: None,
            reservation_id: None,
        }
    }
}