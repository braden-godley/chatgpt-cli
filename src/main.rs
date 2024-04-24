use std::{env, process::exit};
use dirs;
use std::fs;
use serde_json::json;
use reqwest;

fn get_openai_key() -> Result<String, String> {
    let home_dir = dirs::home_dir();
    if let None = home_dir {
        return Err(String::from("Can't find home directory!"));
    }

    let key_file = home_dir.unwrap().join(".openai-key");

    let key = fs::read_to_string(key_file);
    match key {
        Ok(contents) => {
            Ok(String::from(contents.trim()))
        },
        Err(e) => Err(e.to_string()),
    }
}

fn construct_body(prompt: &str) -> String {
    let body = json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {
                "role": "user",
                "content": prompt,
            },
        ],
    });

    body.to_string()
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ChatGPTResponse {
    id: String,
    object: String,
    created: i32,
    model: String,
    choices: Vec<ChatGPTResponseChoice>,
    usage: ChatGPTResponseUsage,
    system_fingerprint: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatGPTResponseChoice {
    index: i32,
    message: ChatGPTResponseChoiceMessage,
    logprobs: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatGPTResponseChoiceMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ChatGPTResponseUsage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

fn get_response(prompt: &str, openai_key: &str) -> Result<String, String> {
    let request_body = construct_body(&prompt);

    let client = reqwest::blocking::Client::new();
    let response_body = client.post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(openai_key)
        .header("Content-Type", "application/json")
        .body(request_body)
        .send();

    match response_body {
        Ok(response) => {
            let data = response.text();
            if let Err(_) = data {
                return Err(String::from("Can't read response"))
            }

            let data = data.unwrap();

            let parsed_data: Result<ChatGPTResponse, _> = serde_json::from_str(&data);

            match parsed_data {
                Ok(gpt_response) => {
                    match gpt_response.choices.get(0) {
                        Some(choice) => Ok(choice.message.content.clone()),
                        None => Err(String::from("No first message"))
                    }
                },
                Err(_) => Err(String::from("Can't parse ChatGPT API response")),
            }
        },
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let openai_key = get_openai_key();
    
    if let Err(e) = openai_key {
        println!("{}", e);
        exit(1);
    }

    let openai_key = openai_key.unwrap();

    let mut prompt = String::new();

    for arg in args {
        prompt.push_str(&arg);
        prompt.push_str(" ");
    }

    let response = get_response(&prompt, &openai_key);

    match response {
        Ok(response) => println!("ChatGPT: {response}"),
        Err(e) => { dbg!(e); },
    };
}
