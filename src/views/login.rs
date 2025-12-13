use crate::API_CLIENT;
use crate::Route;
use crate::components::action::*;
use crate::components::base::message;
use crate::components::button::{Button, ButtonHolder, ButtonVariant};
use crate::components::input::Input;
use crate::components::list::List;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct AppSettings {
    pub token: String,
    pub server: String,
}

#[component]
pub fn Logout() -> Element {
    let nav = navigator();
    rsx! {
        Button {
            onclick: move |_| {
                API_CLIENT.write().set_token("token".to_string());
                tracing::info!("removed the token");
                nav.push(Route::Login {});
            },

            "Logout"
        }
    }
}

#[component]
pub fn Login() -> Element {
    LoginWithCode()
}
#[component]
pub fn LoginWithCode() -> Element {
    let mut server = use_signal(|| "127.0.0.1:31009".to_string());
    let mut challenge_id = use_signal(|| "".to_string());
    let mut code = use_signal(|| "".to_string());
    let mut token = use_signal(|| "settings.read().token".to_string());

    let _validate_settings = use_resource(move || {
        let client = API_CLIENT.read().clone();
        async move {
            match client.list_spaces().await {
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
            Button {
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
            Button {
                variant: ButtonVariant::Secondary,
                onclick: move |_| {
                    let mut client = API_CLIENT.cloned();
                    client.set_server(server());

                    let client_create_key = client.clone();
                    spawn(async move {
                        match client_create_key.create_api_key(challenge_id(), code()).await {
                            Ok(r) => {
                                match r.api_key {
                                    Some(key) => {
                                        token.set(key);
                                        tracing::debug!("token: {}", token());
                                        client.set_token(token());
                                        *API_CLIENT.write() = client;
                                        tracing::debug!("api_client: {:#?}", API_CLIENT.read());
                                        let nav = navigator();
                                        nav.push(Route::Home {});
                                    }
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
