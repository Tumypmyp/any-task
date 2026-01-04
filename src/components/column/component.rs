use dioxus::prelude::*;
#[component]
pub fn Column(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        button {
            class: "column",
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
