use crate::API_CLIENT;
use crate::components::base::*;
use crate::helpers::models::DateTimeFormat;
use crate::helpers::*;
use dioxus::prelude::*;
use dioxus_primitives::scroll_area::ScrollDirection;
use std::vec;
#[component]
pub fn ShowPropertiesSetting(
    space_id: Signal<String>,
    show_properties: Store<Vec<PropertyViewInfo>>,
    other_properties: Store<Vec<PropertyViewInfo>>,
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
                            .list_select_property_options(&space_id, property_id.clone().as_str())
                            .await;
                        let options = match select_property_options {
                            Ok(o) => o.data.unwrap(),
                            _ => vec![],
                        };
                        other_properties.write().push(PropertyViewInfo {
                            id: property_id.clone(),
                            name: property_name,
                            options,
                            date_format: DateTimeFormat::DateTime,
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
                        for (i , property) in show_properties.read().clone().iter().enumerate() {
                            ShowProperty2 {
                                key: "{property.id.as_str()}-chosen",
                                index: i,
                                id: property.id.clone(),
                                show_properties,
                                other_properties,
                            }
                        }
                        for (i , property) in other_properties.read().clone().iter().enumerate() {
                            ShowProperty3 {
                                key: "{property.id.as_str()}-chosen",
                                index: i,
                                id: property.id.clone(),
                                show_properties,
                                other_properties,
                            }
                        }
                                        // for (i , property) in all_properties.read().clone().iter().enumerate() {
                    //     if property.show {
                    //         ShowProperty {
                    //             key: "{property.id.as_str()}-chosen",
                    //             index: i,
                    //             id: property.id.clone(),
                    //             show_properties,
                    //             all_properties,
                    //         }
                    //     }
                    // }
                    // for (i , property) in all_properties.read().clone().iter().enumerate() {
                    //     if !property.show {
                    //         ShowProperty {
                    //             key: "{property.id.as_str()}",
                    //             index: i,
                    //             id: property.id.clone(),
                    //             show_properties,
                    //             all_properties,
                    //         }
                    //     }
                    // }
                    }
                }
            }
        }
    }
}
// #[component]
// pub fn ShowProperty(
//     index: usize,
//     id: PropertyID,
//     show_properties: Store<Vec<PropertyViewInfo>>,
//     all_properties: Store<Vec<PropertyViewInfo>>,
// ) -> Element {
//     let name = (all_properties.get(index).unwrap())().name.clone();
//     let mut show = use_signal(|| (all_properties.get(index).unwrap())().show);
//     rsx! {
//         ButtonWithHolder {
//             variant: if show() { ButtonVariant::Primary } else { ButtonVariant::Ghost },
//             onclick: move |_| {
//                 show.set(!show());
//                 if let Some(mut val) = all_properties.get_mut(index) {
//                     val.show = show();
//                 }
//                 if show() {
//                     show_properties
//                         .with_mut(|v| {
//                             v.push((all_properties.get(index).unwrap())());
//                         })
//                 } else {
//                     show_properties
//                         .with_mut(|v| {
//                             v.retain(|p| p.id != id);
//                         });
//                 }
//             },
//             "{name}"
//         }
//     }
// }

#[component]
pub fn ShowProperty2(
    index: usize,
    id: PropertyID,
    show_properties: Store<Vec<PropertyViewInfo>>,
    other_properties: Store<Vec<PropertyViewInfo>>,
) -> Element {
    let name = (show_properties.get(index).unwrap())().name.clone();

    rsx! {
        ButtonWithHolder {
            variant: ButtonVariant::Primary,
            onclick: move |_| {
                other_properties
                    .with_mut(|v| {
                        v.push((show_properties.get(index).unwrap())());
                    });
                show_properties
                    .with_mut(|v| {
                        v.retain(|p| p.id != id);
                        let cmp = |p1: &PropertyViewInfo, p2: &PropertyViewInfo| {
                            p1.name.cmp(&p2.name)
                        };
                        v.sort_by(cmp);
                    });
            },
            "{name}"
        }
    }
}

#[component]
pub fn ShowProperty3(
    index: usize,
    id: PropertyID,
    show_properties: Store<Vec<PropertyViewInfo>>,
    other_properties: Store<Vec<PropertyViewInfo>>,
) -> Element {
    let name = (other_properties.get(index).unwrap())().name.clone();

    rsx! {
        ButtonWithHolder {
            variant: ButtonVariant::Ghost,
            onclick: move |_| {
                show_properties
                    .with_mut(|v| {
                        v.push((other_properties.get(index).unwrap())());
                    });
                other_properties
                    .with_mut(|v| {
                        v.retain(|p| p.id != id);
                    });
            },
            "{name}"
        }
    }
}
