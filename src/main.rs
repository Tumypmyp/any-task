use crate::components::toast::ToastProvider;
mod engine;
use engine::*;
mod protos;
use dioxus::prelude::*;
use dioxus_desktop;
use dioxus_desktop::wry::dpi::PhysicalSize;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_router::components::{HistoryProvider, Router};
use std::env;
use std::path::PathBuf;
use views::*;
mod components;
mod views;
use helpers::*;
mod helpers;
use serde::{Deserialize, Serialize};
mod persistent_history;
use persistent_history::*;
use std::rc::Rc;
pub const USER_SETTINGS_KEY: &str = "settings_65lkj4";
use crate::helpers::api_client::Client;
use dioxus_sdk_storage::LocalStorage;
use dioxus_sdk_storage::use_synced_storage;
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct AppSettings {
    pub mnemonic: String,
    pub account_id: String,
}
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const THEME_CSS: Asset = asset!("/assets/dx-components-theme.css");
#[derive(Clone, Routable)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    #[redirect("/:.._s", |_s:Vec<String>|Route::Home{})]
    Home {},
    #[route("/login")]
    Login {},
    #[route("/space/:space_id")]
    Space { space_id: String },
    #[route("/space/:space_id/list/:list_id")]
    List { space_id: String, list_id: String },
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
    dioxus_desktop::launch::launch(App, vec![], vec![Box::new(cfg)]);
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
#[component]
fn App() -> Element {
    tracing::info!("App is started");
    let settings =
        use_synced_storage::<LocalStorage, AppSettings>(USER_SETTINGS_KEY.into(), || AppSettings {
            account_id: "".to_string(),
            mnemonic: "".to_string(),
        });
    use_context_provider(|| settings);
    use_future(move || async move {
        let mnemonic = settings.peek().mnemonic.clone();
        let account_id = settings.peek().account_id.clone();
        if !mnemonic.is_empty() {
            let root_path_str = get_app_data_dir().to_string_lossy().to_string();
            match Client::init_from_mnemonic(mnemonic, account_id, root_path_str).await {
                Ok(client) => {
                    *API_CLIENT.write() = Some(client);
                }
                Err(_) => {}
            }
        }
    });
    use_drop(move || {
        tracing::info!("App closing. Stopping engine...");
        stop_engine();
    });
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: THEME_CSS }
        // document::Stylesheet { href: asset!("/src/components/button/style.css") }
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
