use dioxus::prelude::*;
#[derive(Default, PartialEq, Clone, Copy)]
pub enum Position {
    #[default]
    Left,
    Middle,
    Right,
}
impl Position {
    pub fn as_str(&self) -> &'static str {
        match self {
            Position::Left => "left",
            Position::Middle => "middle",
            Position::Right => "right",
        }
    }
}
#[component]
pub fn Row(
    #[props(default)]
    position: Position,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        button {
            class: "row",
            "data-position": position.as_str(),
            onclick: move |event| {
                if let Some(f) = &onclick {
                    f.call(event);
                }
            },
            {children}
        }
    }
}
