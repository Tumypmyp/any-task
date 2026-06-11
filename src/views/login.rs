use crate::API_CLIENT;
use crate::AppSettings;
use crate::Route;
use crate::components::base::message;
use crate::components::button::{Button, ButtonVariant};
use crate::components::column::Column;
use crate::components::input::Input;
use crate::helpers::api_client::Client;
use crate::helpers::*;
use dioxus::prelude::*;
use std::env;
use std::path::PathBuf;
#[component]
pub fn Logout() -> Element {
    let nav = navigator();
    rsx! {
        Button {
            onclick: move |_| {
                *API_CLIENT.write() = None;
                tracing::info!("removed the token");
                nav.push(Route::Login {});
            },
            "Logout"
        }
    }
}
pub fn get_app_data_dir() -> PathBuf {
    if cfg!(target_os = "windows") {
        PathBuf::from(env::var("LOCALAPPDATA").expect("LOCALAPPDATA not found")).join("AnyTask")
    } else if cfg!(target_os = "linux") {
        let base = env::var("XDG_DATA_HOME").unwrap_or_else(|_| {
            format!("{}/.local/share", env::var("HOME").expect("HOME not found"))
        });
        PathBuf::from(base).join("AnyTask")
    } else if cfg!(target_os = "android") {
        PathBuf::from("/data/user/0/com.Tumypmyp.AnyTask/files")
    } else {
        PathBuf::from(".anytask")
    }
}
#[derive(Clone, Debug)]
pub enum AppState {
    StartingEngine,
    NeedsAccount,
    Processing(String),
    Ready,
    Error(String),
}
#[component]
pub fn Login() -> Element {
    let mut settings = use_context::<Signal<AppSettings>>();
    let mut app_state = use_signal(|| AppState::StartingEngine);
    use_future(move || async move {
        let mnemonic = settings.peek().mnemonic.clone();
        let account_id = settings.peek().account_id.clone();
        if mnemonic.is_empty() {
            app_state.set(AppState::NeedsAccount);
        } else {
            app_state.set(AppState::Processing(
                format!("mnemonic is {}.", mnemonic).to_string() + "Recovering existing account...",
            ));
            let root_path_str = get_app_data_dir().to_string_lossy().to_string();
            match Client::init_from_mnemonic(mnemonic, account_id, root_path_str).await {
                Ok(client) => {
                    *API_CLIENT.write() = Some(client);
                    app_state.set(AppState::Ready);
                }
                Err(e) => app_state.set(AppState::Error(e.to_string())),
            }
        }
    });
    let handle_create_account = move |_| {
        if matches!(*app_state.read(), AppState::Processing(_)) {
            return;
        }
        app_state.set(AppState::Processing(
            "Creating new wallet and account...".to_string(),
        ));
        spawn(async move {
            let root_path_str = get_app_data_dir().to_string_lossy().to_string();
            let nav = navigator();
            match Client::init_new_account(root_path_str).await {
                Ok((mnemonic, client)) => {
                    settings.write().mnemonic = mnemonic;
                    settings.write().account_id = client.account_id.clone();
                    *API_CLIENT.write() = Some(client);
                    app_state.set(AppState::Ready);
                }
                Err(e) => {
                    app_state.set(AppState::Error(e.to_string()));
                }
            };
            nav.push(Route::Home {});
        });
    };
    rsx! {
        div { style: "padding: 40px; text-align: center; color: white;",
            h2 { "Welcome to AnyTask" }
            p { style: "margin-bottom: 20px;", "No existing account found." }
            button {
                onclick: handle_create_account,
                style: "padding: 10px 20px; cursor: pointer; font-size: 16px;",
                "Create New Account"
            }
        }
    }
}
