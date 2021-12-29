use anyhow::Result;

pub struct Client {
    client: reqwest::blocking::Client,
    channel_access_token: String,
}

const BROADCAST_URL: &str = "https://api.line.me/v2/bot/message/broadcast";

impl Client {
    pub fn new(channel_access_token:&str) -> Client {
        let client = reqwest::blocking::Client::new();
        Client{client, channel_access_token:channel_access_token.to_string()}
    }

    pub fn broadcast(&self, messages:Vec<&str>) -> Result<String>{
        let body = Messages::new(messages);
        let auth = format!("Bearer {}", self.channel_access_token);
        let retry_key = uuid::Uuid::new_v4().to_hyphenated().to_string();
        let resp = self.client.post(BROADCAST_URL)
            .header("Authorization", auth)
            .header("X-Line-Retry-Key", retry_key)
            .json(&body).send()?;
        let resp_str = resp.text()?;
        Ok(resp_str)
    }
}

#[derive(serde::Serialize)]
struct Messages {
    messages: Vec<Message>,
}

impl Messages {
    fn new(texts:Vec<&str>) -> Messages {
        let mut messages = Vec::new();
        for text in texts {
            messages.push(Message::new(text));
        }
        Messages{messages}
    }
}

#[derive(serde::Serialize)]
struct Message {
    #[serde(rename(serialize = "type"))]
    typ: String,
    text: String,
}

impl Message {
    fn new(text:&str) -> Message {
        Message{typ:"text".to_string(), text:text.to_string()}
    }
}
