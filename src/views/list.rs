use crate::API_CLIENT;
use crate::ActionHolder;
use crate::Actions;
use crate::ListEntry;
use crate::components::Header;
use crate::components::Title;
use crate::components::add_properties::ShowPropertiesSetting;
use crate::components::base::message;
use crate::components::choose_view::ChooseView;
use crate::components::edit_properties::PropertiesOrder;
use crate::helpers::models::DateTimeFormat;
use crate::helpers::*;
use dioxus::prelude::*;
use std::vec;
#[component]
pub fn List(space_id: String, list_id: String) -> Element {
    tracing::info!("loading space {space_id}, list {list_id}");
    let space_id = use_signal(|| space_id);
    let list_id = use_signal(|| list_id);
    let view_id = use_store(|| "".to_string());

    let show_properties: Store<Vec<PropertyInfo>> = use_store(|| {
        vec![PropertyInfo {
            id: PropertyID(NAME_PROPERTY_ID_STR.to_string()),
            name: "Name".to_string(),
            date_format: DateTimeFormat::DateTime,
            options: vec![],
            width: 30.0,
        }]
    });
    rsx! {
        ListHeader {
            space_id,
            list_id,
            view_id,
            show_properties,
        }
        Objects {
            space_id,
            list_id,
            view_id,
            show_properties,
        }
        ActionHolder { Actions {} }
    }
}
#[component]
pub fn ListHeader(
    space_id: Signal<String>,
    list_id: Signal<String>,
    view_id: Store<String>,
    show_properties: Store<Vec<PropertyInfo>>,
) -> Element {
    let mut name = use_signal(|| "".to_string());
    let resp =
        use_resource(move || async move { API_CLIENT.read().get_object(space_id, list_id).await });
    match &*resp.read() {
        Some(Ok(p)) => {
            name.set(p.clone().object.unwrap().name.unwrap());
        }
        _ => {}
    }
    let other_properties: Store<Vec<PropertyInfo>> = use_store(|| vec![]);

    rsx! {
        Header {
            Title { title: "{name}" }
            ShowPropertiesSetting { space_id, show_properties, other_properties }
            ChooseView { space_id, list_id, view_id }
        }
        PropertiesOrder { show_properties, other_properties }
    }
}
#[component]
pub fn Objects(
    space_id: Signal<String>,
    list_id: Signal<String>,
    view_id: Store<String>,
    show_properties: Store<Vec<PropertyInfo>>,
) -> Element {
    let resp = use_resource(move || async move {
        API_CLIENT
            .read()
            .get_list_objects(space_id, list_id, view_id)
            .await
    });

    match &*resp.read() {
        Some(Ok(p)) => {
            // tracing::debug!("objects: {:#?}", p.clone().data.unwrap());
            rsx! {
                for obj in p.clone().data.unwrap() {
                    ListEntry {
                        name: obj.clone().name.unwrap(),
                        space_id,
                        object_id: obj.clone().id.unwrap(),
                        show_properties,
                        data: obj.clone(),
                    }
                }
            }
        }
        Some(Err(err)) => {
            message::error("Failed to fetch objects", err);
            rsx! {}
        }
        None => rsx! { "Loading..." },
    }
}
