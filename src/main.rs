use dioxus::prelude::*;
use dioxus_desktop;
use std::env;
use views::*;
mod views;
use components::*;
mod components;
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const THEME_CSS: Asset = asset!("/assets/styling/theme.css");
const STYLE_CSS: Asset = asset!("/assets/styling/style.css");
#[derive(Clone, Routable)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    #[redirect("/:..segments", |segments:Vec<String>|Route::Home{})]
    Home {},
    #[route("/space/:id")]
    Space { id: String },
    #[route("/login")]
    Login {},
}
fn main() {
    if cfg!(target_os = "windows") {
        let user_data_dir = env::var("LOCALAPPDATA")
            .expect("env var LOCALAPPDATA not found");
        let cfg = dioxus_desktop::Config::new().with_data_directory(user_data_dir);
        dioxus_desktop::launch::launch(App, vec![], vec![Box::new(cfg)])
    } else {
        dioxus::launch(App);
    }
}
#[component]
fn App() -> Element {
    _ = document::eval("document.documentElement.setAttribute('data-theme', 'dark');");
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: THEME_CSS }
        document::Link { rel: "stylesheet", href: STYLE_CSS }
        Router::<Route> {}
    }
}
