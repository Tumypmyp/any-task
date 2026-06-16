use dioxus::prelude::*;
#[derive(Default, PartialEq, Clone, Copy)]
pub enum ColumnPosition {
    #[default]
    Left,
    Middle,
    Right,
}
impl ColumnPosition {
    pub fn as_str(&self) -> &'static str {
        match self {
            ColumnPosition::Left => "left",
            ColumnPosition::Middle => "middle",
            ColumnPosition::Right => "right",
        }
    }
}

#[component]
pub fn Column(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    #[props(default)] position: ColumnPosition,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            class: "column",
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
