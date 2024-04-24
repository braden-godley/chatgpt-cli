use serde::{Deserialize, Serialize};

/// Data structures for deserializing the response from the ChatGPT API.
#[derive(Serialize, Deserialize, Debug)]
pub struct ChatGPTResponse {
    pub id: String,
    pub object: String,
    pub created: i32,
    pub model: String,
    pub choices: Vec<ChatGPTResponseChoice>,
    pub usage: ChatGPTResponseUsage,
    pub system_fingerprint: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatGPTResponseChoice {
    pub index: i32,
    pub message: ChatGPTResponseChoiceMessage,
    pub logprobs: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatGPTResponseChoiceMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatGPTResponseUsage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}
