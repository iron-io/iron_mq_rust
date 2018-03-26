#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub body: String,
    pub delay: u32,
    #[serde(skip_serializing_if = "Option::is_none")] pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub reserved_count: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")] pub reservation_id: Option<String>,
}

impl Message {
    fn new(body: String, delay: u32) -> Message {
        Message {
            body: body,
            delay: delay,
            id: None,
            reserved_count: None,
            reservation_id: None,
        }
    }
}