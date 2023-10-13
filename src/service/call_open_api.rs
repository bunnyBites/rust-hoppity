use crate::model::common::large_language_model::{ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use std::env;

// call large language model (here - GPT-3.5-turbo)
pub async fn call_gpt(messages: Vec<Message>) {
    // load environment variables from .env in root directory.
    dotenv().ok();

    // extract api key and org id from env
    // both of them are provided in the api-key and settings option respectively in openai dashbord
    let api_key: String =
        env::var("OPEN_AI_API_KEY").expect("Failed to find OPEN_AI_API_KEY from environment file");
    let org_id: String =
        env::var("OPEN_AI_ORG_ID").expect("Failed to find OPEN_AI_ORG_ID from environment file");

    // confirm endpoint (chat completion)
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // create headers
    let mut headers = HeaderMap::new();

    // insert api_key to headers
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );

    // insert org_id to headers
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(&org_id.as_str()).unwrap(),
    );

    // create client
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    // create payload for our chat completion api
    let chat_completion_payload = ChatCompletion {
        model: "gpt-3.5-turbo".to_string(), // provide other model varients if you want
        messages,
        temperature: 0.1,
    };

    // test api call
    let raw_response = client
        .post(url)
        .json(&chat_completion_payload)
        .send()
        .await
        .unwrap();

    dbg!(raw_response);
}