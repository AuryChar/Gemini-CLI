extern crate dotenv;

use dotenv::dotenv;
use reqwest::Client;
use serde_json::json;
use serde::{Deserialize, Serialize};
use std::io::{self, Write, Read};
use std::fs;
use std::fs::{File, exists, OpenOptions};

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: Content,
}

#[derive(Deserialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Deserialize)]
struct Part {
    text: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    message: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let client = Client::new();
    let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent";
    let api_key = std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY must be set");
    let mut message = String::new();
    let instruction = String::from("You are a helpful assistent! DEVELOPER NOTE: The JSON structure that you receive is your chat history, user messages uses user_message and yourself messages uses gemini_messages, and don't use MarkDown, answer just the last message sent and use messages before it just for chat context, not for embasament");
    let file_exists = exists("chat.json").expect("cannot read if file exists or not");

    if !file_exists {
        File::create("chat.json").expect("failed to create chat.json");
    }

    let mut file = OpenOptions::new().read(true).write(true).open("chat.json").expect("failed to open chat.json");
    let mut data = vec![];
    file.read_to_end(&mut data).expect("failed to read from chat.json");
    let mut context = String::from_utf8_lossy(&data).to_string();
    // println!("{} \n", context);
    loop {
        print!("You: ");

        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");

        if message.trim().contains(".exit") {
            fs::remove_file("chat.json").expect("failed to remove chat.json");
            break;
        }

        let user_message = ChatMessage {
            role: "user".to_string(),
            message: message.clone(),
        };

        let user_json = serde_json::to_string(&user_message).expect("failed to convert to json");
        file.write_all(user_json.as_bytes()).expect("failed to write to chat.json");
        let mut cache_reader = OpenOptions::new().read(true).open("chat.json").expect("failed to open chat.json");
        let mut cache_data = vec![];
        cache_reader.read_to_end(&mut cache_data).expect("failed to read from chat.json");
        context = String::from_utf8_lossy(&cache_data).to_string();
        // println!("{}", context);
        let response: GeminiResponse = client
            .post(url)
            .header("x-goog-api-key", &api_key)
            .header("Content-Type", "application/json")
            .json(&json!({
                "contents": [
                {
                    "parts": [
                        {
                            "text": &context
                        }
                    ]
                }
                ],
                "systemInstruction": {
                    "parts": [
                        {
                            "text": instruction
                        }
                    ]
                }
            }))
            .send()
            .await
            .expect("Failed to send request")
            .json()
            .await
            .expect("Failed to parse response");

        message = String::new();

        if let Some(text) = &response.candidates[0].content.parts[0].text {
            let gemini_message = ChatMessage {
                role: "gemini".to_string(),
                message: text.clone(),
            };

            let gemini_json = serde_json::to_string(&gemini_message).expect("failed to convert to json");
            file.write_all(gemini_json.as_bytes()).expect("failed to write to chat.json");
            println!("Gemini: {} \n", text);
        }
    }
}
