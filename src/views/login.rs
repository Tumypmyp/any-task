use crate::API_CLIENT;
use crate::AppSettings;
use crate::Route;
use crate::components::base::message;
use crate::components::button::{Button, ButtonVariant};
use crate::components::column::*;
use crate::components::header::{Header, Title};

use crate::components::header::*;
use crate::components::input::Input;
use crate::helpers::api_client::Client;
use crate::helpers::*;
use crate::mnemonic_store::*;
use dioxus::prelude::*;
use std::env;
use std::path::PathBuf;
#[component]
pub fn Logout() -> Element {
    let mut settings = use_context::<Signal<AppSettings>>();
    let nav = navigator();
    rsx! {
        Button {
            onclick: move |_| {
                delete_mnemonic().ok();
                settings.write().account_id = String::new();
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
#[derive(Clone, Debug, PartialEq)]
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
        let account_id = settings.peek().account_id.clone();
        if !account_id.is_empty() {
            let Ok(mnemonic) = load_mnemonic() else {
                app_state.set(AppState::NeedsAccount);
                return;
            };
            app_state.set(AppState::Processing(
                "Recovering existing account...".to_string(),
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
                    match save_mnemonic(&mnemonic) {
                        Ok(()) => { /* success */ }
                        Err(e) => {
                            app_state.set(AppState::Error(e.to_string()));
                            return;
                        }
                    }
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
    let nav = navigator();
    rsx! {
        Column {
            style: "padding: 20px; gap: 10px;",
            position: ColumnPosition::Middle,
            Header {
                Title { title: "Welcome to AnyTask!" }
            }
            MnemonicInput {
                loading: *app_state.read() == AppState::Processing("loading".to_string()),
                on_submit: move |mnemonic: String| {
                    app_state.set(AppState::Processing("loading".to_string()));
                    spawn(async move {
                        let root_path_str = get_app_data_dir().to_string_lossy().to_string();
                        let mnemonic_clone = mnemonic.clone();
                        match Client::recover_from_mnemonic(mnemonic_clone, root_path_str).await {
                            Ok(client) => {
                                save_mnemonic(&mnemonic).ok();
                                settings.write().account_id = client.account_id.clone();
                                *API_CLIENT.write() = Some(client);
                                app_state.set(AppState::Ready);
                                nav.push(Route::Home {});
                            }
                            Err(e) => app_state.set(AppState::Error(e.to_string())),
                        }
                    });
                },
            }
            Header {
                Title { title: "or" }
            }
            Button { onclick: handle_create_account, "Create New Account" }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct MnemonicInputProps {
    pub on_submit: EventHandler<String>,
    pub loading: bool,
}

#[component]
pub fn MnemonicInput(props: MnemonicInputProps) -> Element {
    let mut mnemonic = use_signal(|| String::new());

    let on_submit = props.on_submit.clone();
    let handle_submit = move |_| {
        let value = mnemonic.read().trim().to_string();
        if !value.is_empty() {
            on_submit.call(value);
        }
    };

    let on_submit2 = props.on_submit.clone();
    let handle_keydown = move |evt: KeyboardEvent| {
        if evt.key() == Key::Enter {
            let value = mnemonic.read().trim().to_string();
            if !value.is_empty() {
                on_submit2.call(value);
            }
        }
    };

    rsx! {
        Input {
            r#type: "password",
            placeholder: "Paste your mnemonic phrase...",
            value: mnemonic.read().clone(),
            oninput: move |evt: FormEvent| mnemonic.set(evt.value()),
            onkeydown: handle_keydown,
            disabled: props.loading,
        }
        Button {
            onclick: handle_submit,
            disabled: props.loading,
            variant: ButtonVariant::Primary,
            if props.loading {
                "Recovering..."
            } else {
                "Recover Account"
            }
        }
    }
}
