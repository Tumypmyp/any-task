use crate::API_CLIENT;
use crate::components::button::*;
use crate::components::popover::*;
use crate::components::scroll_area::ScrollArea;
use crate::helpers::models::DateTimeFormat;
use crate::helpers::*;
use dioxus::prelude::*;
use std::vec;

#[component]
pub fn ShowPropertiesSetting(
    space_id: Signal<String>,
    properties: Store<Vec<PropertyInfo>>,
) -> Element {
    use_effect(move || {
        let client = API_CLIENT.read();
        spawn(async move {
            let space_id = space_id();
            let resp = client.list_properties(&space_id).await;
            match resp {
                Ok(props) => {
                    for prop in props.data.unwrap() {
                        let property_id = PropertyID(prop.id.clone().unwrap());
                        let property_name = prop.name.clone().unwrap();
                        let select_property_options = client
                            .list_select_property_options(&space_id, property_id.clone().as_str())
                            .await;
                        let options = match select_property_options {
                            Ok(o) => o.data.unwrap(),
                            _ => vec![],
                        };
                        properties.write().push(PropertyInfo {
                            id: property_id.clone(),
                            name: property_name,
                            options,
                            date_format: DateTimeFormat::DateTime,
                            width: 15.0,
                            show: false,
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
        PopoverRoot {
            open: open(),
            on_open_change: move |v| {
                open.set(v);
            },
            PopoverTrigger { "Properties" }
            PopoverContent {
                ScrollArea { style: "max-height: 40vh;",
                    for (i , property) in properties.read().clone().iter().enumerate() {
                        if property.show {
                            ShowProperty {
                                key: "{property.id.as_str()}-chosen",
                                index: i,
                                id: property.id.clone(),
                                properties,
                            }
                        }
                    }
                    for (i , property) in properties.read().clone().iter().enumerate() {
                        if !property.show {
                            ShowProperty {
                                key: "{property.id.as_str()}-chosen",
                                index: i,
                                id: property.id.clone(),
                                properties,
                            }
                        }
                    }
                }
            }
        }
    }
}
#[component]
pub fn ShowProperty(index: usize, id: PropertyID, properties: Store<Vec<PropertyInfo>>) -> Element {
    let name = (properties.get(index).unwrap())().name.clone();

    rsx! {
        Button {
            variant: if (properties.get(index).unwrap())().show { ButtonVariant::Primary } else { ButtonVariant::Ghost },
            onclick: move |_| {
                properties
                    .with_mut(|v| {
                        if let Some(property) = v.iter_mut().find(|p| p.id == id) {
                            property.show = !property.show;
                        }
                    });
            },
            "{name}"
        }
    }
}
