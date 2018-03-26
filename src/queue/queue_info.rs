use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct QueueInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")] pub project_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")] pub message_timeout: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")] pub message_expiration: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")] pub size: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")] pub total_messages: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")] pub push: Option<PushInfo>,
    #[serde(skip_serializing_if = "Option::is_none")] pub alerts: Option<Alert>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub queue_type: Option<String>,
}

impl QueueInfo {
    pub fn new(name: String) -> QueueInfo {
        QueueInfo {
            name,
            project_id: None,
            message_timeout: None,
            message_expiration: None,
            queue_type: None,
            size: None,
            total_messages: None,
            push: None,
            alerts: None,
        }
    }

    pub fn name(&mut self, name: String) -> &mut QueueInfo {
        self.name = name;

        self
    }

    pub fn message_timeout(&mut self, message_timeout: u32) -> &mut QueueInfo {
        self.message_timeout = Some(message_timeout);

        self
    }

    pub fn message_expiration(&mut self, message_expiration: u32) -> &mut QueueInfo {
        self.message_expiration = Some(message_expiration);

        self
    }

    pub fn push(&mut self, push: PushInfo) -> &mut QueueInfo {
        self.push = Some(push);

        self
    }

    pub fn alerts(&mut self, alerts: Alert) -> &mut QueueInfo {
        self.alerts = Some(alerts);

        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PushInfo {
    retries_delay: u32,
    retries: u32,
    subscribers: Vec<QueueSubscriber>,
    error_queue: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueueSubscriber {
    name: String,
    url: String,
    headers: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Alert {
    #[serde(rename = "type")] alert_type: String,
    trigger: u32,
    direction: String,
    queue: String,
    snooze: u32,
}
