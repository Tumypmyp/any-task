use std::vec;
use openapi::models::*;
use std::collections::HashMap;
use dioxus::prelude::*;
use crate::components::Title;
use crate::API_CLIENT;
use crate::Actions;
use crate::ListEntry;
#[component]
pub fn List(space_id: String, list_id: String) -> Element {
    tracing::info!("loading space {space_id}, list {list_id}");
    let space_id = use_signal(|| space_id);
    let list_id = use_signal(|| list_id);
    let view_id = use_signal(|| "".to_string());
    rsx! {
        ListTitle { space_id, list_id }
        Objects { space_id, list_id, view_id }
        Actions {}
    }
}
#[component]
pub fn ListTitle(space_id: Signal<String>, list_id: Signal<String>) -> Element {
    let mut name = use_signal(|| "".to_string());
    let resp = use_resource(move || async move {
        API_CLIENT.read().get_object(space_id, list_id).await
    });
    match &*resp.read() {
        Some(Ok(p)) => {
            name.set(p.clone().object.unwrap().name.unwrap());
        }
        _ => {}
    }
    rsx! {
        Title { title: "{name}" }
    }
}
#[component]
pub fn Objects(
    space_id: Signal<String>,
    list_id: Signal<String>,
    view_id: Signal<String>,
) -> Element {
    let mut prop_ids_to_options_map = use_store(|| HashMap::<
        String,
        Vec<ApimodelPeriodTag>,
    >::new());
    let mut ids: Signal<Vec<String>> = use_signal(|| vec![]);
    use_effect(move || {
        spawn(async move {
            let space_id = space_id();
            let properties = API_CLIENT.read().list_properties(&space_id).await;
            match properties {
                Ok(props) => {
                    for prop in props.data.clone().unwrap() {
                        ids.write().push(prop.clone().id.unwrap());
                        let property_id = prop.clone().id.unwrap();
                        let options_res = API_CLIENT
                            .read()
                            .list_select_property_options(&space_id, &property_id)
                            .await;
                        if let Ok(o) = options_res {
                            prop_ids_to_options_map
                                .write()
                                .insert(property_id, o.data.unwrap());
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("error: {:#?}", e);
                }
            }
        });
    });
    let resp = use_resource(move || async move {
        API_CLIENT.read().get_list_objects(space_id, list_id, view_id).await
    });
    match &*resp.read() {
        Some(Ok(p)) => {
            rsx! {
                for obj in p.clone().data.unwrap() {
                    ListEntry {
                        key: "{obj.clone().id.unwrap()}",
                        name: obj.clone().name.unwrap(),
                        space_id,
                        object_id: obj.clone().id.unwrap(),
                        show_properties: true,
                        prop_ids_to_options_map,
                        data: obj.clone(),
                    }
                }
            }
        }
        _ => rsx!(),
    }
}
