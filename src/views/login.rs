use crate::API_CLIENT;
use crate::AppSettings;
use crate::Route;
use crate::components::base::message;
use crate::components::button::{Button, ButtonVariant};
use crate::components::column::*;
use crate::components::header::{Header, Title};
use crate::components::input::Input;
use crate::components::row::*;
use crate::components::show_hide_button::ShowHideButton;
use crate::helpers::api_client::Client;
use crate::helpers::*;
use crate::mnemonic_store::*;
use dioxus::prelude::*;
use dioxus_icons::lucide::{Eye, EyeOff};
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

#[component]
pub fn Login() -> Element {
    let mut settings = use_context::<Signal<AppSettings>>();
    let nav = navigator();
    let mut loading = use_signal(|| false);
    let root_path = use_memo(|| get_app_data_dir().to_string_lossy().to_string());

    let on_success = use_callback(move |(mnemonic, client): (String, Client)| {
        loading.set(false);
        if let Err(e) = save_mnemonic(&mnemonic) {
            message::error_with_description("Could not save mnemonic", &e.to_string());
            return;
        }
        settings.write().account_id = client.account_id.clone();
        *API_CLIENT.write() = Some(client);
        nav.push(Route::Home {});
    });

    let handle_recover = use_callback(move |mnemonic: String| {
        if loading() {
            return;
        }
        loading.set(true);
        spawn(async move {
            match Client::recover_from_mnemonic(mnemonic.clone(), root_path()).await {
                Ok(client) => on_success((mnemonic, client)),
                Err(e) => {
                    loading.set(false);
                    message::error_with_description(
                        "Could not recover from mnemonic",
                        &e.to_string(),
                    );
                }
            }
        });
    });

    let handle_create_account = move |_| {
        if loading() {
            return;
        }
        loading.set(true);
        spawn(async move {
            match Client::init_new_account(root_path()).await {
                Ok((mnemonic, client)) => on_success((mnemonic, client)),
                Err(e) => {
                    loading.set(false);
                    message::error_with_description("Could not initialize account", &e.to_string())
                }
            }
        });
    };

    rsx! {
        Column {
            style: "padding: 20px; gap: 10px;",
            position: ColumnPosition::Middle,
            Header {
                Title { title: "Welcome to AnyTask!" }
            }
            MnemonicInput { loading: loading(), on_submit: move |m| handle_recover(m) }
            Header {
                Title { title: "or" }
            }
            Button { onclick: handle_create_account, disabled: loading(), "Create New Account" }
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
    let show: Signal<bool> = use_signal(|| false);

    let on_submit = props.on_submit;

    let try_submit = use_callback(move |()| {
        let value = mnemonic.read().trim().to_string();
        if !value.is_empty() {
            on_submit.call(value);
        }
    });
    rsx! {
        Row { style: "position: relative; width: 100%;",
            Input {
                r#type: "text",
                style: format!(
                    "width: 100%; padding-right: 2.5rem; box-sizing: border-box; -webkit-text-security: {};",
                    if show() { "none" } else { "disc" },
                ),
                placeholder: "Paste your mnemonic phrase...",
                oninput: move |evt: FormEvent| mnemonic.set(evt.value()),
                onkeydown: move |evt: KeyboardEvent| {
                    if evt.key() == Key::Enter {
                        try_submit(());
                    }
                },
                disabled: props.loading,
            }
            ShowHideButton { show }
       }
        Button {
            onclick: move |_| try_submit(()),
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
