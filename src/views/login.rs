use crate::API_CLIENT;
use crate::Route;
use crate::actions::*;
use crate::components::base::message;
use crate::components::button::ButtonHolder;
use crate::components::input::Input;
use crate::components::list::List;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
pub const USER_SETTINGS_KEY: &str = "settings";
use crate::components::button::{ButtonVariant, ButtonWithHolder};
use dioxus_sdk_storage::LocalStorage;
use dioxus_sdk_storage::use_synced_storage;

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
            server: "127.0.0.1:31010".to_string(),
        });
    rsx! {
        ActionHolder { position: Position::Left,
            ButtonWithHolder {
                onclick: move |_| {
                    tracing::debug!("settings were {:#?}", settings);
                    settings.write().token = "removed token".to_string();
                    API_CLIENT.write().set_server(settings.read().server.to_string());
                    API_CLIENT.write().set_token(settings.read().token.to_string());

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
    LoginWithCode()
}

#[component]
pub fn LoginWithCode() -> Element {
    tracing::info!("login page");
    let mut settings =
        use_synced_storage::<LocalStorage, _>(USER_SETTINGS_KEY.into(), || AppSettings {
            token: "".to_string(),
            server: "127.0.0.1:31010".to_string(),
        });
    tracing::debug!("settings loaded as {:#?}", settings.read());

    let mut server = use_signal(|| settings.read().server.to_string());
    let mut challenge_id = use_signal(|| "".to_string());
    let mut code = use_signal(|| "".to_string());
    let mut token = use_signal(|| settings.read().token.to_string());

    let _validate_settings = use_resource(move || async move {
        API_CLIENT.write().set_server(server());
        API_CLIENT.write().set_token(token());
        settings.set(AppSettings {
            token: token(),
            server: server(),
        });
        tracing::debug!("settings saved as {:#?}", settings.read());
        match API_CLIENT.cloned().list_spaces().await {
            Ok(_) => {
                tracing::debug!("Token valid, spaces listed successfully.");
                let nav = navigator();
                nav.push(Route::Home {});
            }
            Err(e) => {
                tracing::error!("Auto-Token check failed: {:#?}", e);
                message::info("Auto-Login failed".to_string(), "".to_string());
            }
        }
    });

    rsx! {
        List { style: "padding-top: 40vh;",
            ButtonHolder {
                Input {
                    r#type: "url",
                    placeholder: "Anytype API server",
                    style: "width: 50vw",
                    value: "{server.read()}",
                    oninput: move |event: FormEvent| {
                        *server.write() = event.value();
                    },
                }
            }
            ButtonHolder {
                Input {
                    r#type: "number",
                    placeholder: "Anytype code",
                    style: "width: 30vw",
                    value: "{code.read()}",
                    oninput: move |event: FormEvent| {
                        *code.write() = event.value();
                    },
                }
            }
            ButtonWithHolder {
                variant: ButtonVariant::Secondary,
                onclick: move |_| {
                    let mut client = API_CLIENT.cloned();
                    client.set_server(server());
                    spawn(async move {
                        match client.create_auth_challenge().await {
                            Ok(r) => {
                                match r.challenge_id {
                                    Some(id) => challenge_id.set(id),
                                    _ => {
                                        message::info(
                                            "got empty challange_id".to_string(),
                                            "maybe wrong server/port?".to_string(),
                                        )
                                    }
                                }
                            }
                            Err(e) => {
                                message::error("Challenge request failed", &e);
                            }
                        };
                    });
                },
                "Request Code"
            }
            ButtonWithHolder {
                variant: ButtonVariant::Secondary,
                onclick: move |_| {
                    let mut client = API_CLIENT.cloned();
                    client.set_server(server());
                    let client_create_key = client.clone();
                    spawn(async move {
                        match client_create_key.create_api_key(challenge_id(), code()).await {
                            Ok(r) => {
                                match r.api_key {
                                    Some(key) => token.set(key),
                                    _ => {
                                        message::info(
                                            "got empty key".to_string(),
                                            "maybe wrong server/port?".to_string(),
                                        )
                                    }
                                }
                            }
                            Err(e) => {
                                message::error("Challenge failed", &e);
                            }
                        };
                    });
                },
                "Enter"
            }
        }
    }
}
