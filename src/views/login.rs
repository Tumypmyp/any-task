use crate::API_CLIENT;
use crate::Route;
use crate::components::base::ButtonHolder;
use crate::components::base::info;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
pub const USER_SETTINGS_KEY: &str = "settings";
use crate::components::base::{ButtonVariant, ButtonWithHolder};
use dioxus_sdk::storage::LocalStorage;
use dioxus_sdk::storage::use_synced_storage;
use dioxus_sdk::storage::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct AppSettings {
    pub token: String,
    pub server: String,
}

#[component]
pub fn Logout() -> Element {
    let mut settings =
        use_synced_storage::<LocalStorage, _>(USER_SETTINGS_KEY.into(), || AppSettings {
            token: "".to_string(),
            server: "127.0.0.1:31019".to_string(),
        });
    rsx! {
        div { id: "actions-holder",
            ButtonWithHolder {
                onclick: move |_| {
                    tracing::debug!("settings were {:#?}", settings);
                    settings
                        // let nav = navigator();
                        // nav.push(Route::Login{});
                        .set(AppSettings {
                            token: "removed token".to_string(),
                            server: "127.0.0.1:31019".to_string(),
                        });
                    tracing::info!("removed the token");
                    tracing::debug!("settings are {:#?}", settings);
                },
                "Logout"
            }
        }
    }
}

#[component]
pub fn Login() -> Element {
    tracing::info!("login page");
    let mut settings =
        use_synced_storage::<LocalStorage, _>(USER_SETTINGS_KEY.into(), || AppSettings {
            token: "".to_string(),
            server: "127.0.0.1:31019".to_string(),
        });
    tracing::debug!("settings loaded as {:#?}", settings.read());

    let _validate_settings = use_resource(move || async move {
        tracing::info!("Validating API token");
        let new_token = settings().token.trim().to_string();
        API_CLIENT.write().set_token(new_token);
        API_CLIENT.write().set_server(settings().server);
        match API_CLIENT.read().list_spaces().await {
            Ok(_) => {
                tracing::debug!("Token valid, spaces listed successfully.");
                info("Login successful".to_string());
                tracing::debug!("settings saved as {:#?}", settings.read());
                let nav = navigator();
                nav.push(Route::Home {});
            }
            Err(e) => {
                tracing::error!("Token check failed: {:#?}", e);
                info("Login unsuccessful".to_string());
            }
        }
    });
    let mut token = use_signal(|| settings.read().token.to_string());
    let mut server = use_signal(|| settings.read().server.to_string());
    rsx! {
        div { id: "login-holder",
            ButtonHolder {
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
            ButtonHolder {
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
            ButtonWithHolder {
                variant: ButtonVariant::Secondary,
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
