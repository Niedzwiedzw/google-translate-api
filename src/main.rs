#![feature(proc_macro_hygiene, decl_macro)]
use rocket::{
    get,
    routes,
};

use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use reqwest::blocking as req;
use reqwest::{
    header,
};
use std::collections::HashMap;

const GOOGLE_URL: &'static str = "https://translation.googleapis.com/v3/projects/1069704146063:translateText";

pub fn get_translation(token: String, text: String, source: String, target: String) -> Option<String> {
    let client = req::Client::new();
    let response = client.post(GOOGLE_URL)
        .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
        .header(header::AUTHORIZATION, format!("Bearer {}", token))
        .json(&GoogleRequest::new(source, target, text))
        .send()
        .ok()?;
    let translation: GoogleResponse = response.json::<GoogleResponse>().ok()?;
    Some(translation.translations[0].translatedText.clone())
}



#[derive(Serialize, Deserialize, Debug)]
struct WordFastResponse {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GoogleRequest {
    pub sourceLanguageCode: String,
    pub targetLanguageCode: String,
    pub contents: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TranslatedText {
    pub translatedText: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GoogleResponse {
    pub translations: Vec<TranslatedText>,
}

impl GoogleRequest {
    pub fn new(source: String, target: String, contents: String) -> Self {
        Self {
            sourceLanguageCode: source,
            targetLanguageCode: target,
            contents: vec![contents],
        }
    }
}

#[get("/<source>/<target>/<token>/<text>")]
fn index(source: String, target: String, token: String, text: String) -> Option<Json<WordFastResponse>> {
    let text = get_translation(
        token,
        text,
        source,
        target,
    )?;

    Some(Json(WordFastResponse { text }))

    //
    // Json(WordFastResponse {
    //     token,
    //     text,
    // })
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
