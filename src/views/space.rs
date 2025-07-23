use crate::views::Search;
use dioxus::prelude::*;
use crate::views::actions::Actions;
use crate::views::*;

#[component]
pub fn Space(id: String) -> Element {
    let value = use_signal(|| id.clone());
    let mut name = use_signal(|| "".to_string());    
    let space = use_resource(move || async move {
            let mut config = openapi::apis::configuration::Configuration::new();
            config.bearer_access_token = Some(home::TOKEN.read().clone());

            openapi::apis::spaces_api::get_space(&config, "2025-05-20", &value.read())
                .await
                .unwrap()
    });
    
    match space.read().clone() {
        Some(s ) => {
            name.set(s.clone().space.clone().unwrap().name.unwrap());
            tracing::debug!("{:#?}", s.space.unwrap().name.unwrap());
        },
        _ => {}
    };
    rsx! {
        div { id: "title-holder",
            button { class: "button", "data-style": "ghost", "{name}" }
        }
        Search { space_id: id.clone() }
        Actions {}
    }
}
