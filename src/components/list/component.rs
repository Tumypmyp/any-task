use dioxus::prelude::*;

#[component]
pub fn List(
    #[props(extends=GlobalAttributes)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div { class: "list", ..attributes, {children} }
    }
}
