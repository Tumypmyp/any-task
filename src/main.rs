// use crate::components::toast::ToastProvider;
mod protos;
use protos::anytype::client_commands_client::ClientCommandsClient;
use protos::*;
mod engine;
use dioxus::prelude::*;
use dioxus_desktop;
use dioxus_desktop::wry::dpi::PhysicalSize;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus_router::components::{HistoryProvider, Router};
use engine::*;
use std::env;
use std::path::PathBuf;
use views::*;
mod views;
// use components::*;
mod components;
use helpers::*;
mod helpers;
use serde::{Deserialize, Serialize};
mod persistent_history;
use persistent_history::*;
use std::rc::Rc;
pub const USER_SETTINGS_KEY: &str = "settings-~aaabbcccdee";
use dioxus_sdk_storage::LocalStorage;
use dioxus_sdk_storage::use_synced_storage;

use crate::helpers::api_client::Client;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct AppSettings {
    pub mnemonic: String,
    pub account_id: String,
    // pub api_key: String,
    // pub account_key: String,
    // pub server: String,
}

// use views::AppSettings;
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const THEME_CSS: Asset = asset!("/assets/dx-components-theme.css");
#[derive(Clone, Routable)]
#[rustfmt::skip]
enum Route {
#[route("/")]
#[redirect("/:.._s", |_s:Vec<String>|Route::Home{})]
Home {},
// #[route("/spaces/:space_id")]
// Space { space_id: String },
// #[route("/spaces/:space_id/lists/:list_id")]
// ObjectList { space_id: String, list_id: String },
// #[route("/login")]
// Login {},
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

#[derive(Clone, Debug)]
pub enum AppState {
    StartingEngine,
    NeedsAccount,
    Processing(String),
    Ready,
    Error(String),
}

#[component]
fn App() -> Element {
    tracing::info!("App is started");

    let mut settings =
        use_synced_storage::<LocalStorage, AppSettings>(USER_SETTINGS_KEY.into(), || AppSettings {
            account_id: "".to_string(),
            mnemonic: "".to_string(),
        });

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
                    *API_CLIENT.write() = client;
                    app_state.set(AppState::Ready);
                }
                Err(e) => app_state.set(AppState::Error(e)),
            }
        }
    });

    use_drop(move || {
        tracing::info!("App closing. Stopping engine...");
        stop_engine();
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

            match Client::init_new_account(root_path_str).await {
                Ok((mnemonic, client)) => {
                    settings.write().mnemonic = mnemonic;
                    settings.write().account_id =
                        client.account_id.clone().expect("account id was not saved");
                    *API_CLIENT.write() = client;
                    app_state.set(AppState::Ready);
                }
                Err(e) => {
                    app_state.set(AppState::Error(e));
                }
            }
        });
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Stylesheet { href: MAIN_CSS }
        document::Stylesheet { href: THEME_CSS }
        document::Stylesheet { href: asset!("/src/components/button/style.css") }

        match &*app_state.read() {
            AppState::StartingEngine => rsx! {
                div { style: "color: white; padding: 20px;", "Starting Anytype Engine..." }
            },
            AppState::Processing(msg) => rsx! {
                div { style: "color: white; padding: 20px;", "{msg}" }
            },
            AppState::Error(e) => rsx! {
                div { style: "color: red; padding: 20px;", "Error: {e}" }

            },
            AppState::NeedsAccount => rsx! {
                div { style: "padding: 40px; text-align: center; color: white;",
                    h2 { "Welcome to AnyTask" }
                    p { style: "margin-bottom: 20px;", "No existing account found." }
                    button {
                        onclick: handle_create_account,
                        style: "padding: 10px 20px; cursor: pointer; font-size: 16px;", // Add your custom classes here
                        "Create New Account"
                    }
                }
            },
            AppState::Ready => rsx! {
                HistoryProvider {
                    history: move |_| {
                        Rc::new(PersistentHistory::default().with_prefix("/any-task")) as Rc<dyn History>
                    },
                    Router::<Route> {}
                }
            },
        }
    }
}
// #[component]
// fn App() -> Element {
//     tracing::info!("App is started");

//     let mut settings =
//         use_synced_storage::<LocalStorage, AppSettings>(USER_SETTINGS_KEY.into(), || AppSettings {
//             mnemonic: "".to_string(),
//         });

//     let engine_status: Resource<std::result::Result<(_, _), _>> =
//         use_resource(move || async move {
//             let addr = "127.0.0.1:31020";
//             tracing::info!("Initializing Anytype Engine...");

//             if let Err(e) = start_engine(addr) {
//                 tracing::error!("Engine failed to start: {}", e);
//                 return Err(e);
//             }

//             tracing::info!("Engine started. Checking account status...");

//             let mut client = ClientCommandsClient::connect(format!("http://{}", addr))
//                 .await
//                 .map_err(|e| e.to_string())?;

//             client
//                 .initial_set_parameters(protos::anytype::rpc::initial::set_parameters::Request {
//                     platform: "windows".to_string(), // Updated to reflect your current OS
//                     version: "0.0.1".to_string(),
//                     workdir: get_app_data_dir().to_string_lossy().to_string(),
//                     ..Default::default()
//                 })
//                 .await
//                 .map_err(|e| e.to_string())?;

//             let root_path_str = get_app_data_dir().to_string_lossy().to_string();

//             tracing::info!("Calling WalletCreate at path: {}", root_path_str);
//             let wallet_res = client
//                 .wallet_create(protos::anytype::rpc::wallet::create::Request {
//                     root_path: root_path_str.clone(),
//                     ..Default::default()
//                 })
//                 .await
//                 .map_err(|e| e.to_string())?
//                 .into_inner();

//             let mnemonic = wallet_res.mnemonic;
//             settings.write().mnemonic = mnemonic.clone();

//             tracing::info!("Calling WalletCreateSession...");
//             let session_res = client
//                 .wallet_create_session(protos::anytype::rpc::wallet::create_session::Request {
//                     auth: Some(
//                         protos::anytype::rpc::wallet::create_session::request::Auth::Mnemonic(
//                             mnemonic.clone(),
//                         ),
//                     ),
//                 })
//                 .await
//                 .map_err(|e| e.to_string())?
//                 .into_inner();

//             let token = session_res.token;
//             tracing::info!("Session created. Token obtained.");

//             // Optional: If your engine requires the token in gRPC metadata for subsequent calls,
//             // you would attach it like this:
//             // let mut request = tonic::Request::new(protos::anytype::rpc::account::create::Request { ... });
//             // request.metadata_mut().insert("token", token.parse().unwrap());

//             // --- 3. Create Account ---
//             tracing::info!("Calling AccountCreate...");
//             let account_res = client
//                 .account_create(protos::anytype::rpc::account::create::Request {
//                     name: "My New Account".into(),
//                     store_path: root_path_str,
//                     network_mode: 1, // Usually 1 for local/standard networking
//                     ..Default::default()
//                 })
//                 .await
//                 .map_err(|e| e.to_string())?
//                 .into_inner();

//             let account = account_res
//                 .account
//                 .ok_or("Account data missing from response")?;
//             let account_id = account.id.clone();

//             // The tech space ID is required to scope your search for spaces
//             let tech_space_id = account.info.unwrap_or_default().tech_space_id.clone();
//             tracing::info!(
//                 "Account created successfully! ID: {}, TechSpace: {}",
//                 account_id,
//                 tech_space_id
//             );

//             // --- 4. List Spaces via Subscription ---
//             tracing::info!("Calling ObjectSearchSubscribe to list spaces...");
//             let list_sub_res = client
//                 .object_search_subscribe(protos::anytype::rpc::object::search_subscribe::Request {
//                     space_id: tech_space_id,
//                     sub_id: "space".to_string(), // Matching J.Constant.subId.space
//                     filters: vec![],

//                     keys: vec!["targetSpaceId".to_string()],

//                     ..Default::default()
//                 })
//                 .await
//                 .map_err(|e| e.to_string())?;

//             tracing::info!("Successfully subscribed to spaces list.");

//             let resp = list_sub_res.into_inner();
//             tracing::debug!("response: {:#?}", resp.clone());
//             let mut spaces = Vec::new();

//             for record in resp.records {
//                 tracing::debug!("space: {:#?}", record);
//                 let id_field = record
//                     .fields
//                     .get("id")
//                     .or_else(|| record.fields.get("targetSpaceId"));

//                 if let Some(id_val) = id_field {
//                     // Safely extract the string from the gRPC Value enum
//                     if let Some(prost_types::value::Kind::StringValue(id_str)) = &id_val.kind {
//                         spaces.push(id_str.clone());
//                     }
//                 }
//             }
//             Ok((account_id, spaces))
//         });
//     use_drop(move || {
//         tracing::info!("App closing. Stopping engine...");
//         stop_engine();
//     });
//     // use_context_provider(|| settings);
//     // {
//     //     let s = settings.read();
//     //     let mut client = API_CLIENT.write();
//     //     if client.get_token() != s.api_key || client.get_server() != s.server {
//     //         client.set_api_key(s.api_key.clone());
//     //         client.set_server(s.server.clone());
//     //     }
//     // }

//     // update client on settings change
//     // use_effect(move || {
//     //     let s = settings.read();
//     //     let mut client = API_CLIENT.write();
//     //     client.set_api_key(s.api_key.clone());
//     //     client.set_server(s.server.clone());
//     // });

//     rsx! {
//         // ToastProvider {
//         document::Link { rel: "icon", href: FAVICON }
//         document::Stylesheet { href: MAIN_CSS }
//         document::Stylesheet { href: THEME_CSS }
//         document::Stylesheet { href: asset!("/src/components/button/style.css") }

//         match &*engine_status.read() {
//             None => rsx! {
//                 div { "Loading engine..." }
//             },
//             Some(Err(e)) => rsx! {
//                 div { "Error: {e}" }
//             },
//             // Destructure the tuple we returned from the resource
//             Some(Ok((account_id, spaces))) => rsx! {
//                 div {
//                 style: "padding: 20px; font-family: sans-serif; color: white;",

//                     h2 { "Account Created Successfully" }
//                     p {
//                         "Your Account ID is: "
//                         strong { "{account_id}" }
//                     }

//                     h3 { style: "margin-top: 20px;", "Your Spaces:" }

//                     // Check if the spaces list is empty
//                     if spaces.is_empty() {
//                         p { style: "color: #aaa;", "No spaces found or still loading." }
//                     } else {
//                         ul {
//                         style: "background: #222; padding: 15px; border-radius: 8px;",
//                             for space_id in spaces {
//                                 li {
//                                     style: "margin-bottom: 8px;",
//                                     "📦 Space ID: {space_id}"
//                                 }
//                             }
//                         }
//                     }
//                 }
//             },
//         }
//         // match &*engine_status.read() {
//         //     None => rsx! {
//         //         // Login {}
//         //     },
//         //     Some(Err(_)) => rsx! {
//         //         // Login {}
//         //     },
//         //     Some(Ok(account_id, spaces)) => rsx! {
//         //         div {
//         //         style: "padding: 20px; font-family: sans-serif; color: white;",
//         //             h2 { "Account Created Successfully" }
//         //             p {
//         //                 "Your Account ID is: "
//         //                 strong { "{account_id}" }
//         //             }
//         //         }

//         //         //     HistoryProvider {
//         //         //             Rc::new(PersistentHistory::default().with_prefix("/any-task")) as Rc<dyn History>
//         //         //         history: move |_| {
//         //         //         },
//         //         //         Router::<Route> {}
//         //         //     }
//         //     },
//         // }
//     }
// }
