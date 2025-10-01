use std::vec;
use dioxus::prelude::*;
use crate::components::base::*;
use crate::API_CLIENT;
use dioxus_primitives::scroll_area::ScrollDirection;
use crate::helpers::*;
#[component]
pub fn ShowPropertiesSetting(
    space_id: Signal<String>,
    properties_order: Store<Vec<PropertyViewInfo>>,
) -> Element {
    use_effect(move || {
        spawn(async move {
            let space_id = space_id();
            let properties = API_CLIENT.read().list_properties(&space_id).await;
            match properties {
                Ok(props) => {
                    for prop in props.data.unwrap() {
                        let property_id = PropertyID(prop.id.clone().unwrap());
                        let property_name = prop.name.clone().unwrap();
                        let select_property_options = API_CLIENT
                            .read()
                            .list_select_property_options(
                                &space_id,
                                property_id.clone().as_str(),
                            )
                            .await;
                        let options = match select_property_options {
                            Ok(o) => o.data.unwrap(),
                            _ => vec![],
                        };
                        properties_order
                            .write()
                            .push(PropertyViewInfo {
                                id: property_id.clone(),
                                name: property_name,
                                show: false,
                                options,
                                width: 15.0,
                            });
                    }
                }
                Err(e) => {
                    tracing::error!("error loading property list: {:#?}", e);
                }
            }
        });
    });
    let mut open = use_signal(|| false);
    rsx! {
        ButtonHolder { "flex-shrink": "0", width: "20vw",
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
                        for (i , property) in properties_order.read().clone().iter().enumerate() {
                            if property.show {
                                ShowProperty {
                                    key: "{property.id.as_str()}-chosen",
                                    index: i,
                                    properties_order,
                                }
                            }
                        }
                        for (i , property) in properties_order.read().clone().iter().enumerate() {
                            if !property.show {
                                ShowProperty {
                                    key: "{property.id.as_str()}",
                                    index: i,
                                    properties_order,
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
    index: usize,
    properties_order: Store<Vec<PropertyViewInfo>>,
) -> Element {
    let name = properties_order.get(index).unwrap().name.clone();
    let mut show = use_signal(|| properties_order.get(index).unwrap().show);
    rsx! {
        ButtonWithHolder {
            variant: if show() { ButtonVariant::Primary } else { ButtonVariant::Ghost },
            onclick: move |_| {
                show.set(!show());
                if let Some(mut val) = properties_order.get_mut(index) {
                    val.show = show();
                }
            },
            "{name}"
        }
    }
}
