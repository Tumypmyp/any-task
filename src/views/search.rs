use dioxus::prelude::*;
use crate::views::*;

const STYLE_CSS: Asset = asset!("/assets/styling/style.css");
#[component]
pub fn Search(space_id: String) -> Element {
    let space_id = use_signal(|| space_id.clone());
    let objects = use_resource(move || async move {
            API_CLIENT.read().get_tasks(&space_id.read()).await
        }
    );
    
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
                                                space_id,
                                                object_id: object.clone().id.unwrap(),
                                            }
                                        }
                                    }
                                    _ => rsx! {},
                                }
                            }
                        }
                    }
                }
            }
        },
        Some(Err(e)) => {
            tracing::debug!("error: {:#?}", e);
            crate::Error()
        },
        _ => rsx! ()
        
    }
}
