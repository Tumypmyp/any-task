use crate::API_CLIENT;
use crate::Route;
use crate::anytype_cli::*;
use crate::components::base::message;
use crate::components::button::{Button, ButtonHolder, ButtonVariant};
use crate::components::column::Column;
use crate::components::input::Input;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct AppSettings {
    pub api_key: String,
    pub account_key: String,
    pub server: String,
}

#[component]
pub fn Logout() -> Element {
    let nav = navigator();
    rsx! {
        Button {
            onclick: move |_| {
                API_CLIENT.write().set_api_key("token".to_string());
                tracing::info!("removed the token");
                nav.push(Route::Login {});
            },
            "Logout"
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum LoginMethod {
    RemoteCode,
    RemoteToken,
    LocalCli,
}

#[component]
pub fn Login() -> Element {
    let mut active_method = use_signal(|| LoginMethod::LocalCli);

    rsx! {
        div { class: "login-container",
            div { style: "display: flex; justify-content: center; gap: 15px; padding-top: 20px; flex-wrap: wrap;",
                Button {
                    variant: if active_method() == LoginMethod::RemoteCode { ButtonVariant::Primary } else { ButtonVariant::Secondary },
                    onclick: move |_| active_method.set(LoginMethod::RemoteCode),
                    "Login with Code"
                }
                Button {
                    variant: if active_method() == LoginMethod::RemoteToken { ButtonVariant::Primary } else { ButtonVariant::Secondary },
                    onclick: move |_| active_method.set(LoginMethod::RemoteToken),
                    "Login with API Key"
                }
                Button {
                    variant: if active_method() == LoginMethod::LocalCli { ButtonVariant::Primary } else { ButtonVariant::Secondary },
                    onclick: move |_| active_method.set(LoginMethod::LocalCli),
                    "Embedded Node"
                }
            }
            match active_method() {
                LoginMethod::RemoteCode => rsx! {
                    LoginWithCode {}
                },
                LoginMethod::RemoteToken => rsx! {
                    LoginWithToken {}
                },
                LoginMethod::LocalCli => rsx! {
                    LoginToLocalCli {}
                },
            }
        }
    }
}

#[component]
pub fn LoginToLocalCli() -> Element {
    let mut settings = use_context::<Signal<AppSettings>>();
    // let server = use_signal(|| settings.read().server.clone());
    let server = use_signal(|| "127.0.0.1:31020".to_string());
    let mut account_key = use_signal(|| settings.read().account_key.clone());
    let mut api_key = use_signal(|| settings.read().api_key.clone());
    let mut is_loading = use_signal(|| false);

    rsx! {
        Column { style: "padding-top: 30vh;",
            ButtonHolder {
                Input {
                    r#type: "password",
                    value: "{account_key}",
                    oninput: move |e: FormEvent| account_key.set(e.value()),
                    placeholder: "Enter account key",
                }
            }
            ButtonHolder {
                Input {
                    r#type: "password",
                    value: "{api_key}",
                    oninput: move |e: FormEvent| api_key.set(e.value()),
                    placeholder: "Enter api key",
                }
            }
            Button {
                variant: ButtonVariant::Primary,
                disabled: is_loading() || account_key().is_empty() || api_key().is_empty(),
                onclick: move |_| {
                    is_loading.set(true);
                    let key = account_key();
                    spawn(async move {
                       match login_to_account(key).await {
                            Ok(_) => {
                                tracing::info!("CLI authenticated successfully.");

                                settings.write().server = server();
                                settings.write().account_key = account_key();

                                let mut client = API_CLIENT.cloned();
                                client.set_server(server());
                                client.set_api_key(api_key());

                                let api_key_val = api_key();
                                spawn(async move {
                                    match client.list_spaces().await {
                                        Ok(_) => {
                                            settings.write().api_key = api_key_val;
                                            *API_CLIENT.write() = client;
                                            navigator().push(Route::Home {});
                                        }
                                        Err(e) => message::error("Invalid API Key or Server", &e),
                                    }
                                });
                            }
                            Err(login_err) => {
                                message::error_with_description(
                                    "Local CLI login failed",
                                    &login_err,
                                );
                            }
                        }
                       is_loading.set(false);
                    });
                },
                if is_loading() {
                    "Authenticating..."
                } else {
                    "Initialize Local Node"
                }
            }
        }
    }
}
#[component]
pub fn LoginWithToken() -> Element {
    let mut settings = use_context::<Signal<AppSettings>>();
    let mut server = use_signal(|| settings.read().server.clone());
    let mut api_key = use_signal(|| settings.read().api_key.clone());

    rsx! {
        Column { style: "padding-top: 30vh;",
            ButtonHolder {
                Input {
                    r#type: "url",
                    value: "{server}",
                    oninput: move |e: FormEvent| server.set(e.value()),
                    placeholder: "Anytype API server (e.g. 127.0.0.1:31009)",
                }
            }
            ButtonHolder {
                Input {
                    r#type: "password",
                    value: "{api_key}",
                    oninput: move |e: FormEvent| api_key.set(e.value()),
                    placeholder: "API Key",
                }
            }
            Button {
                variant: ButtonVariant::Primary,
                onclick: move |_| {
                    let mut client = API_CLIENT.cloned();
                    client.set_server(server());
                    client.set_api_key(api_key());

                    let token_val = api_key();
                    let server_val = server();

                    spawn(async move {
                        match client.list_spaces().await {
                            Ok(_) => {
                                settings.write().api_key = token_val;
                                settings.write().server = server_val;
                                *API_CLIENT.write() = client;
                                navigator().push(Route::Home {});
                            }
                            Err(e) => message::error("Invalid API Key or Server", &e),
                        }
                    });
                },
                "Connect"
            }
        }
    }
}

#[component]
pub fn LoginWithCode() -> Element {
    let mut settings = use_context::<Signal<AppSettings>>();

    let mut server = use_signal(|| "127.0.0.1:31012".to_string());
    let mut challenge_id = use_signal(|| "".to_string());
    let mut code = use_signal(|| "".to_string());
    let _validate_settings = use_resource(move || async move {
        let client = API_CLIENT.read().clone();
        if client.get_token().is_empty() {
            return;
        }

        match client.list_spaces().await {
            Ok(_) => {
                tracing::debug!("Auto-login successful");
                navigator().push(Route::Home {});
            }
            Err(e) => {
                tracing::error!("Auto-login check failed: {:#?}", e);
                message::error("Auto-login failed", &e);
            }
        }
    });
    rsx! {
        Column { style: "padding-top: 40vh;",
            ButtonHolder {
                Input {
                    r#type: "url",
                    value: "{server}",
                    oninput: move |e: FormEvent| server.set(e.value()),
                    placeholder: "Anytype API server",
                }
            }
            ButtonHolder {
                Input {
                    r#type: "number",
                    value: "{code}",
                    oninput: move |e: FormEvent| code.set(e.value()),
                    placeholder: "Anytype code",
                }
            }
            Button {
                variant: ButtonVariant::Secondary,
                onclick: move |_| {
                    let mut client = API_CLIENT.cloned();
                    client.set_server(server());
                    spawn(async move {
                        if let Ok(r) = client.create_auth_challenge().await {
                            if let Some(id) = r.challenge_id {
                                challenge_id.set(id);
                            }
                        }
                    });
                },
                "Request Code"
            }
            Button {
                variant: ButtonVariant::Secondary,
                onclick: move |_| {
                    let mut client = API_CLIENT.cloned();
                    client.set_server(server());

                    spawn(async move {
                        match client.create_api_key(challenge_id(), code()).await {
                            Ok(r) => {
                                if let Some(key) = r.api_key {
                                    settings.write().api_key = key.clone();
                                    settings.write().server = server();
                                    client.set_api_key(key);
                                    *API_CLIENT.write() = client;
                                    navigator().push(Route::Home {});
                                }
                            }
                            Err(e) => message::error("Challenge failed", &e),
                        }
                    });
                },
                "Enter"
            }
        }
    }
}
