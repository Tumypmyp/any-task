use crate::API_CLIENT;
use crate::Route;
use crate::hooks::*;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
pub const USER_SETTINGS_KEY: &str = "user_settings";
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct AppSettings {
    pub token: String,
}
async fn check_and_save_token(
    token_signal: Signal<String>,
    mut settings_signal: Signal<AppSettings>,
) {
    tracing::info!("Validating API token");
    let new_token = token_signal.read().trim().to_string();
    API_CLIENT.write().set_token(new_token.clone());
    match API_CLIENT.read().list_spaces().await {
        Ok(_) => {
            let mut settings = settings_signal.write();
            settings.token = new_token;
            match save_data(USER_SETTINGS_KEY, &*settings) {
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
async fn remove_token() {
    let settings = AppSettings {
        token: "".to_string(),
    };
    match save_data(USER_SETTINGS_KEY, &settings) {
        Ok(_) => tracing::debug!("Settings saved successfully!"),
        Err(e) => tracing::error!("Error saving settings: {}", e),
    }
    let nav = navigator();
    nav.push(Route::Login {});
}
#[component]
pub fn Logout() -> Element {
    rsx! {
        div { id: "actions-holder",
            div { class: "button-holder",
                button {
                    class: "button",
                    "data-style": "outline",
                    onclick: move |_| {
                        spawn(remove_token());
                    },
                    "Logout"
                }
            }
        }
    }
}
#[component]
pub fn Login() -> Element {
    tracing::info!("login page");
    let settings = use_signal(|| {
        get_data::<AppSettings>(USER_SETTINGS_KEY)
            .unwrap_or_else(|e| {
                tracing::error!("Error loading settings: {}", e);
                AppSettings {
                    token: "".to_string(),
                }
            })
    });
    let mut token = use_signal(|| settings.read().token.to_string());
    spawn(check_and_save_token(token, settings));
    rsx! {
        div { id: "login-holder",
            input {
                class: "input",
                placeholder: "Paste your Anytype API token",
                style: "width: 50vw",
                value: "{token.read()}",
                oninput: move |event| {
                    *token.write() = event.value();
                },
            }
            div { class: "button-holder",
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
}
