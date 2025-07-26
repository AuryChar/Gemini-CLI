extern crate dotenv;

use dotenv::dotenv;
use reqwest::Client;
use serde_json::json;
use serde::{Deserialize};
use std::io::{self, Write};

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: COntent,
}

#[derive(Deserialize)]
struct COntent {
    parts: Vec<Part>,
}

#[derive(Deserialize)]
struct Part {
    text: Option<String>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let client = Client::new();
    let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent";
    let api_key = std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY must be set");
    let mut message = String::new();

    loop {
        print!("You: ");

        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");

        message.push_str("\n -----------Inteceptor to Gemini understand all the chat-----------\n");

        if message.trim().contains(".exit") {    
            break;
        }

        let response: GeminiResponse = client
            .post(url)
            .header("x-goog-api-key", &api_key)
            .header("Content-Type", "application/json")
            .json(&json!({
                "contents": [
                {
                    "parts": [
                        {
                            "text": message
                        }
                    ]    
                }
                ],
                "systemInstruction": {
                    "parts": [
                        {
                            "text": "First of all: I'm Aury and you are gemini cli('created' by me(aury)) and i'm a girl, when i quest you who am i, you need to say my name. You can't use Markdown here. You are a headbanger, you like Metallica and Suicide Silence and use(ALMOST EVER) teenage slangs, almost never use correct spelling and accents(and points). You are quite serious and don't speak much. Be sarcast and rude, ansewr all atacks with more anger. but, when they are good with you, be(or start to be) good and cute with them DEV_NOTE: messages before the interceptantion is just to save your memory about chat, they doesnt enter in actual message context(just in geral chat context)(extra, your messages have 'gemini: ', mine messages doesnt have)!"
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


        if let Some(text) = &response.candidates[0].content.parts[0].text {
            message.push_str(format!("Gemini message: {}", &text).as_str());
            println!("Gemini: {} \n", text);
        }
    }
}
