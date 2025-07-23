use dioxus::prelude::*;

use views::*;
mod views;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const THEME_CSS: Asset = asset!("/assets/styling/theme.css");

#[derive(Clone, Routable)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},

    #[route("/space/:id")]
    Space { id: String },
    
 }
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: THEME_CSS }
        Router::<Route> {}
    }   
}
