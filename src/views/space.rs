use crate::views::Search;
use dioxus::prelude::*;
use crate::views::actions::Actions;
use crate::views::*;
use crate::Route;

#[component]
pub fn Space(id: String) -> Element {
    let value = use_signal(|| id.clone());
    let mut name = use_signal(|| "".to_string());    
    let space = use_resource(move || async move {
           API_CLIENT.read().get_space(&value.read()).await           
    });
    
    match &*space.read() {
        Some(Ok(s)) => {
            name.set(s.clone().space.clone().unwrap().name.unwrap());
        },
        Some(Err(e)) => {
            tracing::debug!("error: {:#?}", e);
            let nav = navigator();
            nav.push(Route::Home{});
        },
        _ => {}
    }

    rsx! {
        div { id: "title-holder",
            button { class: "button", "data-style": "ghost", "{name}" }
        }
        Search { space_id: id.clone() }
        Actions {}
    }
}