use std::vec;
use openapi::models::*;
use std::collections::HashMap;
use dioxus::prelude::*;
use crate::components::buttons::*;
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
    let prop_ids_to_show = use_store(|| HashMap::<PropertyID, bool>::new());
    let properties_order: Store<Vec<PropertyID>> = use_store(|| vec![]);
    rsx! {
        ListHeader {
            space_id,
            list_id,
            view_id,
            prop_ids_to_show,
            properties_order,
        }
        Objects {
            space_id,
            list_id,
            view_id,
            prop_ids_to_show,
            properties_order,
        }
        Actions {}
    }
}
#[component]
pub fn ListHeader(
    space_id: Signal<String>,
    list_id: Signal<String>,
    view_id: Signal<String>,
    properties_order: Store<Vec<PropertyID>>,
    prop_ids_to_show: Store<HashMap<PropertyID, bool>>,
) -> Element {
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
        div { "flex-direction": "column",
            Title { title: "{name}" }
            Properties {
                space_id,
                list_id,
                view_id,
                prop_ids_to_show,
                properties_order,
            }
        }
    }
}
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct PropertyID(pub String);
impl PropertyID {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
#[component]
pub fn Properties(
    space_id: Signal<String>,
    list_id: Signal<String>,
    view_id: Signal<String>,
    properties_order: Store<Vec<PropertyID>>,
    prop_ids_to_show: Store<HashMap<PropertyID, bool>>,
) -> Element {
    let mut prop_ids_to_options_map = use_store(|| HashMap::<
        PropertyID,
        Vec<ApimodelPeriodTag>,
    >::new());
    let mut prop_ids_to_prop_map = use_store(|| HashMap::<
        PropertyID,
        ApimodelPeriodProperty,
    >::new());
    use_effect(move || {
        spawn(async move {
            let space_id = space_id();
            let properties = API_CLIENT.read().list_properties(&space_id).await;
            match properties {
                Ok(props) => {
                    for prop in props.data.clone().unwrap() {
                        let property_id = PropertyID(prop.id.clone().unwrap());
                        properties_order.write().push(property_id.clone());
                        prop_ids_to_show.insert(property_id.clone(), true);
                        prop_ids_to_prop_map
                            .write()
                            .insert(property_id.clone(), prop.clone());
                        let options_res = API_CLIENT
                            .read()
                            .list_select_property_options(
                                &space_id,
                                property_id.clone().as_str(),
                            )
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
    rsx! {
        for prop_id in properties_order.read().clone() {
            if let Some(prop) = prop_ids_to_prop_map.read().get(&prop_id.clone()) {
                Property {
                    property_id: prop_id,
                    name: prop.name.clone().unwrap(),
                    prop_ids_to_show,
                }
            }
        }
    }
}
#[component]
pub fn Property(
    property_id: PropertyID,
    name: String,
    prop_ids_to_show: Store<HashMap<PropertyID, bool>>,
) -> Element {
    let mut show = use_signal(|| {
        *prop_ids_to_show.get(property_id.clone()).unwrap().read()
    });
    rsx! {
        Button {
            variant: if show() { ButtonVariant::Primary } else { ButtonVariant::Ghost },
            onclick: move |_| {
                show.set(!show());
                if let Some(val) = prop_ids_to_show.write().get_mut(&property_id.clone()) {
                    *val = show();
                }
            },
            "{name}"
        }
    }
}
#[component]
pub fn Objects(
    space_id: Signal<String>,
    list_id: Signal<String>,
    view_id: Signal<String>,
    properties_order: Store<Vec<PropertyID>>,
    prop_ids_to_show: Store<HashMap<PropertyID, bool>>,
) -> Element {
    let mut prop_ids_to_options_map = use_store(|| HashMap::<
        PropertyID,
        Vec<ApimodelPeriodTag>,
    >::new());
    use_effect(move || {
        spawn(async move {
            let space_id = space_id();
            let properties = API_CLIENT.read().list_properties(&space_id).await;
            match properties {
                Ok(props) => {
                    for prop in props.data.clone().unwrap() {
                        let property_id = prop.clone().id.unwrap();
                        let options_res = API_CLIENT
                            .read()
                            .list_select_property_options(&space_id, &property_id)
                            .await;
                        if let Ok(o) = options_res {
                            prop_ids_to_options_map
                                .write()
                                .insert(PropertyID(property_id), o.data.unwrap());
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
                        properties_order,
                        prop_ids_to_show,
                        prop_ids_to_options_map,
                        data: obj.clone(),
                    }
                }
            }
        }
        _ => rsx!(),
    }
}
