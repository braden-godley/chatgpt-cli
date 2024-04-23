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

fn get_response(prompt: &str, openai_key: &str) -> Result<String, reqwest::Error> {
    let request_body = construct_body(&prompt);

    println!("request body: {request_body}");
    println!("key: '{openai_key}'");

    let client = reqwest::blocking::Client::new();
    let response_body = client.post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(openai_key)
        .header("Content-Type", "application/json")
        .body(request_body)
        .send();

    println!("got request body");

    match response_body {
        Ok(response) => response.text(),
        Err(e) => Err(e),
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
        Ok(response) => println!("{response}"),
        Err(e) => { dbg!(e); },
    };
}
