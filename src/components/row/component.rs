use dioxus::prelude::*;

#[component]
pub fn Row(children: Element, onclick: Option<EventHandler<MouseEvent>>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        button {
            class: "row",
            // "data-style": variant.class(),
            onclick: move |event| {
                if let Some(f) = &onclick {
                    f.call(event);
                }
            },
            {children}
        }
    }
}
