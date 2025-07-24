use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Error() -> Element {
    let nav = navigator();
    nav.push(Route::Token{});
    
    rsx! { "error" }
}

