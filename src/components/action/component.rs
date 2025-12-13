use crate::Route;
use crate::components::button::Button;
use dioxus::prelude::*;
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum Position {
    #[default]
    Center,
    Left,
}
impl Position {
    pub fn as_str(&self) -> &'static str {
        match self {
            Position::Center => "center",
            Position::Left => "left",
        }
    }
}

#[component]
pub fn ActionHolder(#[props(default)] position: Position, children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "action-holder", "data-position": position.as_str(), {children} }
    }
}

#[component]
pub fn BaseActions() -> Element {
    let nav = navigator();
    rsx! {
        Button {
            onclick: move |_| {
                nav.push(Route::Home {});
            },
            "Home"
        }
        if nav.can_go_back() {
            Button {
                onclick: move |_| {
                    nav.go_back();
                },
                "Back"
            }
        }
    }
}
