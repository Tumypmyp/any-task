use crate::Route;
use crate::components::button::Button;
use dioxus::prelude::*;
use dioxus_icons::lucide::{ArrowLeft, House};
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
pub fn Actions(#[props(default)] position: Position, children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            class: "action-holder",
            "data-position": position.as_str(),
            style: "gap: 30px;",
            {children}
        }
    }
}
#[component]
pub fn BaseActions() -> Element {
    rsx! {
        Actions {
            GoHome {}
            GoBack {}
        }
    }
}

#[component]
pub fn GoHome() -> Element {
    let nav = navigator();
    rsx! {
        Button {
            onclick: move |_| {
                nav.push(Route::Home {});
            },
            aria_label: "Go to home",
            House {}
        }
    }
}

#[component]
pub fn GoBack() -> Element {
    let nav = navigator();
    rsx! {
        if nav.can_go_back() {
            Button {
                onclick: move |_| {
                    nav.go_back();
                },
                aria_label: "Go back",
                ArrowLeft {}
            }
        }
    }
}
