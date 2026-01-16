#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::{Duration, Local};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use log::warn;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_token])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
fn get_token() -> String {
    // Generate your real Twilio JWT here
    create_twilio_token().unwrap_or("NONE".to_string())
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    exp: usize,
    nbf: usize,
    #[serde(rename = "scope")]
    scope: String,
}

pub fn create_twilio_token() -> Option<String> {
    let me = "...";
    let account_sid = "...".to_string();
    let auth_token = "...";
    let app_sid = "...";

    let exp = Local::now() + Duration::hours(1);

    let scope = format!("scope:client:incoming?clientName={} scope:client:outgoing?appSid={}&clientName={}", me, app_sid, me);

    let claims = Claims {
        iss: account_sid.clone(),
        exp: exp.timestamp() as usize,
        nbf: exp.timestamp() as usize,
        scope,
    };

    let key = EncodingKey::from_secret(auth_token.as_ref());
    match encode(&Header::default(), &claims, &key) {
        Ok(token) => Some(token),
        Err(e) => {
            warn!("Error encoding twilio jwt token {}", e);
            None
        }
    }
}