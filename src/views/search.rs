use dioxus::prelude::*;
use crate::views::*;
use crate::Route;

const STYLE_CSS: Asset = asset!("/assets/styling/style.css");
#[component]
pub fn Search(space_id: String) -> Element {
    let space_id = use_signal(|| space_id.clone());
    let objects = use_resource(move || async move {
        let mut config = openapi::apis::configuration::Configuration::new();
        config.bearer_access_token = Some(home::TOKEN.read().clone());
        let mut req = openapi::models::ApimodelPeriodSearchRequest::new();
        req.types = vec!["task".to_string()].into();

        openapi::apis::search_api::search_space(&config, "2025-05-20", &space_id.read(), req, None, None)
            .await
    });
    
    match &*objects.read() {
        Some(Ok(s)) => rsx! {
            document::Link { rel: "stylesheet", href: STYLE_CSS }
            div { id: "object-list",
                for object in s.data.clone().unwrap() {
                    for prop in object.clone().properties.unwrap() {
                        match prop {
                            openapi::models::ApimodelPeriodCheckboxPropertyValue { checkbox, name, .. } => {
                                match name {
                                    Some(n) if n.as_str() == "Done" => {
                                        rsx! {
                                            Task {
                                                done: checkbox.unwrap_or_default(),
                                                name: object.clone().name.unwrap(),
                                                id: object.clone().id.unwrap(),
                                            }
                                        }
                                    }
                                    _ => rsx! {},
                                }
                            }
                            _ => rsx! {},
                        }
                    }
                }
            }
        },
        
        Some(Err(e)) => {
            tracing::debug!("error 2.1 {} ", e);
        
            let nav = navigator();
            nav.push(Route::Home{});
        
            rsx! {
                div { "Loading error dogs..." }
            }
        },
        _ => {
            tracing::debug!("error 2 ");
            rsx! {
                div { "Loading dogs..." }
            }
        }
    }
}
