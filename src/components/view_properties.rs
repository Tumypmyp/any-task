use std::vec;
use openapi::models::*;
use std::collections::HashMap;
use dioxus::prelude::*;
use crate::components::base::*;
use crate::API_CLIENT;
use dioxus_primitives::scroll_area::ScrollDirection;
use crate::helpers::models::PropertyID;
#[component]
pub fn ShowPropertiesSetting(
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
    let mut open = use_signal(|| false);
    rsx! {
        ButtonHolder {
            PopoverRoot {
                open: open(),
                on_open_change: move |v| {
                    open.set(v);
                },
                PopoverTrigger { "Properties" }
                PopoverContent {
                    ScrollArea {
                        width: "13em",
                        height: "15em",
                        border: "1px solid var(--primary-color-6)",
                        border_radius: "0.5em",
                        padding: "0 1em 1em 1em",
                        direction: ScrollDirection::Vertical,
                        tabindex: "0",
                        for prop_id in properties_order.read().clone() {
                            if let Some(prop) = prop_ids_to_prop_map.read().get(&prop_id.clone()) {
                                ShowProperty {
                                    property_id: prop_id,
                                    name: prop.name.clone().unwrap(),
                                    prop_ids_to_show,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
#[component]
pub fn ShowProperty(
    property_id: PropertyID,
    name: String,
    prop_ids_to_show: Store<HashMap<PropertyID, bool>>,
) -> Element {
    let mut show = use_signal(|| {
        *prop_ids_to_show.get(property_id.clone()).unwrap().read()
    });
    rsx! {
        ButtonWithHolder {
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
