use crate::API_CLIENT;
use crate::Route;
use crate::hooks::*;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct AppSettings {
    pub token: String,
}
async fn check_and_save_token(
    token_signal: Signal<String>,
    mut settings_signal: Signal<AppSettings>,
) {
    let new_token = token_signal.read().trim().to_string();
    API_CLIENT.write().set_token(new_token.clone());
    match API_CLIENT.read().list_spaces().await {
        Ok(_) => {
            let mut settings = settings_signal.write();
            settings.token = new_token;
            match save_data("user_settings", &*settings) {
                Ok(_) => tracing::debug!("Settings saved successfully!"),
                Err(e) => tracing::error!("Error saving settings: {}", e),
            }
            let nav = navigator();
            nav.push(Route::Home {});
        }
        Err(e) => {
            tracing::error!("Token check failed: {:#?}", e);
        }
    }
}
#[component]
pub fn Login() -> Element {
    let settings = use_signal(|| {
        get_data::<AppSettings>("user_settings")
            .unwrap_or_else(|e| {
                tracing::error!("Error loading settings: {}", e);
                AppSettings {
                    token: "some-token".to_string(),
                }
            })
    });
    let mut token = use_signal(|| settings.read().token.to_string());
    spawn(check_and_save_token(token, settings));
    rsx! {
        div { id: "token-holder",
            input {
                class: "input",
                placeholder: "Paste your Anytype token",
                value: "{token.read()}",
                oninput: move |event| {
                    *token.write() = event.value();
                    spawn(check_and_save_token(token, settings));
                },
            }
            button {
                class: "button",
                "data-style": "secondary",
                onclick: move |_| {
                    spawn(check_and_save_token(token, settings));
                },
                "Enter"
            }
        }
    }
}
