use crate::API_CLIENT;
use crate::Route;
use crate::hooks::*;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use crate::info;
use crate::error;
pub const USER_SETTINGS_KEY: &str = "user_settings";
#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct AppSettings {
    pub token: String,
    pub server: String,
}
async fn remove_token() {
    let settings = AppSettings {
        token: "".to_string(),
        server: "".to_string(),
    };
    match save_data(USER_SETTINGS_KEY, &settings) {
        Ok(_) => {
            tracing::debug!("Settings saved successfully!");
            info("Settings removed".to_string());
            let nav = navigator();
            nav.push(Route::Login {});
        }
        Err(e) => {
            tracing::error!("Error saving settings: {}", e);
            error("Error removing settings".to_string());
        }
    }
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
    let mut settings = use_signal(|| {
        get_data::<AppSettings>(USER_SETTINGS_KEY)
            .unwrap_or_else(|e| {
                tracing::error!("Error loading settings: {}", e);
                AppSettings {
                    token: "".to_string(),
                    server: "127.0.0.1:31009".to_string(),
                }
            })
    });
    let _validate_settings = use_resource(move || {
        let settings_clone = settings.clone();
        async move {
            tracing::info!("Validating API token");
            let new_token = settings_clone.read().token.trim().to_string();
            API_CLIENT.write().set_token(new_token.clone());
            API_CLIENT.write().set_server(settings_clone.read().server.clone());
            match API_CLIENT.read().list_spaces().await {
                Ok(_) => {
                    tracing::debug!("Token valid, spaces listed successfully.");
                    info("Login successful".to_string());
                    match save_data(USER_SETTINGS_KEY, &*settings_clone.read()) {
                        Ok(_) => tracing::debug!("Settings saved successfully!"),
                        Err(e) => tracing::error!("Error saving settings: {}", e),
                    }
                    let nav = navigator();
                    nav.push(Route::Home {});
                }
                Err(e) => {
                    tracing::error!("Token check failed: {:#?}", e);
                    info("Login unsuccessful".to_string());
                }
            }
        }
    });
    let mut token = use_signal(|| settings.read().token.to_string());
    let mut server = use_signal(|| settings.read().server.to_string());
    rsx! {
        div { id: "login-holder",
            div { class: "button-holder",
                input {
                    class: "input",
                    placeholder: "Anytype API server",
                    style: "width: 30vw",
                    value: "{server.read()}",
                    oninput: move |event| {
                        *server.write() = event.value();
                    },
                }
            }
            div { class: "button-holder",
                input {
                    class: "input",
                    placeholder: "Paste your Anytype API token",
                    style: "width: 50vw",
                    value: "{token.read()}",
                    oninput: move |event| {
                        *token.write() = event.value();
                    },
                }
            }
            div { class: "button-holder",
                button {
                    class: "button",
                    "data-style": "secondary",
                    onclick: move |_| {
                        settings
                            .set(AppSettings {
                                token: token(),
                                server: server(),
                            });
                    },
                    "Enter"
                }
            }
        }
    }
}
