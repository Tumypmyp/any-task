use dioxus::prelude::*;
use crate::Route;
use crate::API_CLIENT;


#[component]
pub fn Home() -> Element {   
    _ = document::eval("document.documentElement.setAttribute('data-theme', 'dark');");
    
    let spaces = use_resource(|| async move {
        API_CLIENT.read().list_spaces().await
    });
    
    match &*spaces.read() {
        Some(Ok(s)) => {
            rsx!{
                div { id: "space-list",
                    for space in s.clone().data.unwrap().clone() {
                        Link {
                            class: "button",
                            "data-style": "primary",
                            to: Route::Space {
                                id: space.clone().id.unwrap(),
                            },
                            "{space.clone().name.unwrap()}"
                        }
                    }
                }
            }
        },
        Some(Err(e)) => {
            tracing::debug!("error: {:#?}", e);
            let nav = navigator();
            nav.push(Route::Token{});
            
            crate::Error()
        }
        _ => rsx! ()
    }
}
