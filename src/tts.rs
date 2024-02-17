use once_cell::sync::Lazy;
use reqwest::Client;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct CreateAudioQuery {
    pub text: String,
    pub speaker: u32,
}

static ENDPOINT: Lazy<String> =
    Lazy::new(|| env::var("VOICEVOX_ENDPOINT").unwrap_or(String::from("http://localhost:50021")));

pub async fn tts(text: String) -> anyhow::Result<Vec<u8>> {
    let client = Client::new();
    let mut audio_query: serde_json::Value = client
        .post(format!("{}/audio_query", *ENDPOINT))
        .query(&CreateAudioQuery { text, speaker: 1 })
        .send()
        .await?
        .json()
        .await?;
    audio_query["outputSamplingRate"] = serde_json::Value::from(48000);
    println!("{:?}", audio_query);
    let audio = client
        .post(format!("{}/synthesis", *ENDPOINT))
        .json(&audio_query)
        .send()
        .await?
        .bytes()
        .await?;
    Ok(audio.to_vec())
}
