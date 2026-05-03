use crate::components::toast::ToastProvider;
use dioxus::prelude::*;
use dioxus_desktop;
use dioxus_desktop::wry::dpi::PhysicalSize;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_router::components::{HistoryProvider, Router};
use std::env;
use std::path::PathBuf;
use views::*;
mod views;
use components::*;
mod components;
use helpers::*;
mod helpers;
mod proxy;
use proxy::*;
mod persistent_history;
use persistent_history::*;
use std::rc::Rc;
pub const USER_SETTINGS_KEY: &str = "settings";
use dioxus_sdk_storage::LocalStorage;
use dioxus_sdk_storage::use_synced_storage;
use views::AppSettings;
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const THEME_CSS: Asset = asset!("/assets/dx-components-theme.css");
#[derive(Clone, Routable)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    #[redirect("/:.._s", |_s:Vec<String>|Route::Home{})]
    Home {},
    #[route("/spaces/:space_id")]
    Space { space_id: String },
    #[route("/spaces/:space_id/lists/:list_id")]
    ObjectList { space_id: String, list_id: String },
    #[route("/login")]
    Login {},
}
fn main() {
    dioxus::logger::initialize_default();
    tracing::info!("starting app");
    let window_config = WindowBuilder::new()
        .with_title("AnyTasks")
        .with_visible(true)
        .with_focused(true)
        .with_inner_size(PhysicalSize::new(900, 1300));
    let cfg = if cfg!(target_os = "windows") {
        let user_data_dir = env::var("LOCALAPPDATA").expect("env var LOCALAPPDATA not found");
        tracing::debug!("user path is {:#?}", user_data_dir);
        dioxus_sdk_storage::set_dir!();
        dioxus_desktop::Config::new()
            .with_data_directory(user_data_dir)
            .with_window(window_config)
    } else if cfg!(target_os = "linux") {
        let user_data_dir = env::var("XDG_DATA_HOME").unwrap_or_else(|_| {
            let home_dir = env::var("HOME").expect("env var HOME not found");
            format!("{}/.local/share", home_dir)
        });
        dioxus_sdk_storage::set_dir!();
        Config::new()
            .with_data_directory(PathBuf::from(user_data_dir).join("AnyTasks"))
            .with_window(window_config)
    } else if cfg!(target_os = "android") {
        let app_files_dir = PathBuf::from("/data/user/0/com.Tumypmyp.AnyTask/files");
        let _ = std::fs::create_dir_all(&app_files_dir);
        dioxus_sdk_storage::set_directory(app_files_dir);
        Config::new()
    } else {
        Config::new()
    };
    let cfg = cfg.with_background_color((0, 0, 0, 0));
    tracing::info!("config is ready");
    dioxus_desktop::launch::launch(App, vec![], vec![Box::new(cfg)]);
}

#[component]
fn App() -> Element {
    tracing::info!("App is started");
    let settings =
        use_synced_storage::<LocalStorage, AppSettings>(USER_SETTINGS_KEY.into(), || AppSettings {
            token: "".to_string(),
            server: "127.0.0.1:31010".to_string(),
        });

    use_context_provider(|| settings);
    {
        let s = settings.read();
        let mut client = API_CLIENT.write();
        if client.get_token() != s.token || client.get_server() != s.server {
            client.set_token(s.token.clone());
            client.set_server(s.server.clone());
        }
    }

    // start the proxy server
    use_effect(|| {
        tokio::task::spawn(async move {
            if let Err(e) = run_proxy_server().await {
                tracing::error!("Proxy server failed: {}", e);
            }
        });
        tracing::info!("Background proxy task started.");
    });

    // update client on settings change
    use_effect(move || {
        let s = settings.read();
        let mut client = API_CLIENT.write();
        client.set_token(s.token.clone());
        client.set_server(s.server.clone());
    });

    rsx! {
        ToastProvider {
            document::Link { rel: "icon", href: FAVICON }
            document::Stylesheet { href: MAIN_CSS }
            document::Stylesheet { href: THEME_CSS }
            document::Stylesheet { href: asset!("/src/components/button/style.css") }
            HistoryProvider {
                history: move |_| {
                    Rc::new(PersistentHistory::default().with_prefix("/any-task")) as Rc<dyn History>
                },
                Router::<Route> {}
            }
        }
    }
}
