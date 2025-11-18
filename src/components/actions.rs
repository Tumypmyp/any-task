use crate::Route;
use crate::components::button::Button;
use crate::row::Row;
use dioxus::prelude::*;
#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum Position {
    #[default]
    Center,
    Left,
}
impl Position {
    pub fn left_pos(&self) -> &'static str {
        match self {
            Position::Center => "50%",
            Position::Left => "0%",
        }
    }

    pub fn transform_value(&self) -> &'static str {
        match self {
            Position::Center => "translateX(-50%)",
            Position::Left => "translateX(0%)",
        }
    }
}

#[component]
pub fn ActionHolder(#[props(default)] position: Position, children: Element) -> Element {
    let style = format!(
        "position: fixed;
         display: flex;
         flex-direction: row;
         bottom: 0;
         left: {};
         transform: {};
         padding: 0.5vw 2vw;
         gap: 5px;
         z-index: 1000;",
        position.left_pos(),
        position.transform_value(),
    );
    rsx! {
        div { style: "{style}", {children} }
    }
}

#[component]
pub fn Actions() -> Element {
    let nav = navigator();
    rsx! {
        Button {
            onclick: move |_| {
                nav.push(Route::Home {});
            },
            "Home"
        }
        Button {
            onclick: move |_| {
                nav.go_back();
            },
            "Back"
        }
    }
}
