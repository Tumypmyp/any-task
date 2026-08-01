use crate::components::toast::ToastProvider;
use futures_util::StreamExt;
use tonic::Streaming;
mod engine;
use engine::*;
mod mnemonic_store;
use mnemonic_store::*;
mod protos;
use dioxus::prelude::*;
use dioxus_desktop;
use dioxus_desktop::wry::dpi::PhysicalSize;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_router::components::{HistoryProvider, Router};
use std::path::PathBuf;
use views::home::*;

use crate::protos::Event;
use views::*;
mod components;
mod views;
use helpers::API_CLIENT;
use helpers::*;
mod helpers;
use serde::{Deserialize, Serialize};
mod persistent_history;
use persistent_history::*;
use std::rc::Rc;
pub const USER_SETTINGS_KEY: &str = "settings";
use crate::helpers::api_client::Client;
use dioxus_sdk_storage::LocalStorage;
use dioxus_sdk_storage::use_synced_storage;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct AppSettings {
    pub account_id: String,
}
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const THEME_CSS: Asset = asset!("/assets/dx-components-theme.css");
static LICENSES: Asset = asset!("/assets/LICENSE-ANYTYPE.md");

#[derive(Clone, Routable)]
#[rustfmt::skip]
enum Route {
    #[layout(AuthGuard)]
        #[route("/")]
        #[redirect("/:.._s", |_s: Vec<String>| Route::Home {})]
        Home {},
        #[route("/login")]
        Login {},
        #[nest("/space/:space_id")]
            #[layout(SpaceLayout)]
                #[route("/")]
                Space { space_id: String },
                #[route("/list/:list_id")]
                List { space_id: String, list_id: String },
            #[end_layout]
        #[end_nest]
        #[route("/settings")]
        Settings {},
}

#[component]
fn AuthGuard() -> Element {
    let navigator = use_navigator();
    let route = use_route::<Route>();

    use_effect(move || {
        // only redirect once the client-init check has finished
        if AUTH_CHECKED() && API_CLIENT.read().is_none() && !matches!(route, Route::Login {}) {
            navigator.replace(Route::Login {});
        }
    });

    rsx! {
        Outlet::<Route> {}
    }
}

#[cfg_attr(feature = "bundle", windows_subsystem = "windows")]
fn main() {
    dioxus::logger::initialize_default();
    tracing::info!("starting app");
    let window_config = WindowBuilder::new()
        .with_title("AnyTask")
        .with_visible(true)
        .with_focused(true)
        .with_inner_size(PhysicalSize::new(900, 1300));
    let addr = "127.0.0.1:31020";
    tracing::info!("Initializing Anytype Engine...");
    if let Err(e) = start_engine(addr) {
        tracing::error!("Failed to start Anytype Engine: {}", e);
        return;
    }
    let data_dir = get_app_data_dir();
    std::fs::create_dir_all(&data_dir).expect("Failed to create application data directory");
    tracing::debug!("User data path is {:#?}", data_dir);
    dioxus_sdk_storage::set_directory(data_dir.clone());
    let cfg = if cfg!(target_os = "windows") {
        dioxus_desktop::Config::new()
            .with_data_directory(data_dir)
            .with_window(window_config)
    } else if cfg!(target_os = "linux") {
        Config::new()
            .with_data_directory(PathBuf::from(data_dir).join("AnyTask"))
            .with_window(window_config)
    } else if cfg!(target_os = "android") {
        Config::new()
    } else {
        Config::new()
    };
    let cfg = cfg.with_background_color((0, 0, 0, 255));
    tracing::info!("config is ready");
    keyring::use_native_store(false).expect("failed to open native credential store");
    dioxus_desktop::launch::launch(App, vec![], vec![Box::new(cfg)]);
}
#[component]
fn App() -> Element {
    set_theme(true);
    tracing::info!("App is started");
    let settings =
        use_synced_storage::<LocalStorage, AppSettings>(USER_SETTINGS_KEY.into(), || AppSettings {
            account_id: "".to_string(),
        });
    use_context_provider(|| settings);
    use_future(move || async move {
        let account_id = settings.peek().account_id.clone();
        if !account_id.is_empty() {
            if let Ok(mnemonic) = load_mnemonic() {
                let root_path_str = get_app_data_dir().to_string_lossy().to_string();
                match Client::init_from_mnemonic(mnemonic, account_id, root_path_str).await {
                    Ok(client) => {
                        *API_CLIENT.write() = Some(client);
                    }
                    Err(_) => {}
                }
            }
        }
        *AUTH_CHECKED.write() = true;
    });
    use_drop(move || {
        tracing::info!("App closing. Stopping engine...");
        stop_engine();
    });

    let event_loop = use_coroutine(|mut rx: UnboundedReceiver<Client>| async move {
        let Some(mut client): Option<Client> = rx.next().await else {
            return;
        };

        loop {
            *RECONNECT_COUNT.write() += 1;
            match client.clone().listen_session_events().await {
                Ok(resp) => {
                    let mut stream: Streaming<Event> = resp.into_inner();
                    loop {
                        tokio::select! {
                            new_client = rx.next() => {
                                match new_client {
                                    Some(c) => { client = c; break; }
                                    None => return,
                                }
                            }
                            msg = stream.message() => {
                                match msg {
                                    Ok(Some(event)) => {
                                        for msg in event.messages { handle_msg(&event.context_id.clone(), msg); }
                                    }
                                    Ok(None) => { tracing::warn!("stream closed, reconnecting"); break; }
                                    Err(e) => { tracing::warn!("stream error: {e}, reconnecting"); break; }
                                }
                            }
                        }
                    }
                }
                Err(e) => tracing::warn!("event stream error: {e}, reconnecting"),
            }
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });

    use_effect(move || {
        if let Some(client) = API_CLIENT.read().as_ref().cloned() {
            event_loop.send(client);
        }
    });
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: THEME_CSS }
        document::Stylesheet { href: asset!("/src/components/button/style.css") }
        ToastProvider {
            HistoryProvider {
                history: move |_| {
                    Rc::new(PersistentHistory::default().with_prefix("/any-task")) as Rc<dyn History>
                },
                Router::<Route> {}
            }
        }
    }
}

fn set_theme(dark_mode: bool) {
    let theme = if dark_mode { "dark" } else { "light" };
    _ = document::eval(&format!(
        "document.documentElement.setAttribute('data-theme', '{theme}');",
    ));
}
