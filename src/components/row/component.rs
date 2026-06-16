use dioxus::prelude::*;
#[derive(Default, PartialEq, Clone, Copy)]
pub enum RowPosition {
    #[default]
    Left,
    Middle,
    Right,
}
impl RowPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            RowPosition::Left => "left",
            RowPosition::Middle => "middle",
            RowPosition::Right => "right",
        }
    }
}
#[component]
pub fn Row(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default)] position: RowPosition,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            class: "row",
            "data-position": position.as_str(),
            onclick: move |event| {
                if let Some(f) = &onclick {
                    f.call(event);
                }
            },
            ..attributes,
            {children}
        }
    }
}
