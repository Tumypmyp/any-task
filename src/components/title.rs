use dioxus::prelude::*;
#[component]
pub fn Title(title: String) -> Element {
    rsx! {
        div { id: "title-holder",
            button { class: "button", "data-style": "ghost", height: "6vh", "{title}" }
        }
    }
}
