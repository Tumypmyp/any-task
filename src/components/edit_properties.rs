use std::vec;
use dioxus::prelude::*;
use crate::components::base::*;
use crate::helpers::*;
#[component]
pub fn PropertiesOrder(properties_order: Store<Vec<PropertyViewInfo>>) -> Element {
    rsx! {
        div { width: "95vw", display: "flex", "flex-direction": "row",
            for (index , property) in properties_order.read().clone().iter().enumerate() {
                if property.show {
                    PropertyHolder {
                        key: "{property.id.as_str()}",
                        index,
                        properties_order,
                    }
                }
            }
        }
    }
}
#[component]
pub fn PropertyHolder(
    index: usize,
    properties_order: Store<Vec<PropertyViewInfo>>,
) -> Element {
    let property = properties_order.get(index).unwrap();
    let mut open = use_signal(|| false);
    let mut current_value = use_signal(|| property.width);
    rsx! {
        PopoverRoot {
            open: open(),
            on_open_change: move |v| {
                open.set(v);
            },
            ButtonHolder {
                PopoverTrigger { width: "{current_value}vw", "{property.clone().name}" }
            }
            PopoverContent {
                div { style: "margin-bottom: 15px; font-size: 16px; font-weight: bold;",
                    "{current_value:.0}"
                }
                Slider {
                    label: "Demo Slider",
                    horizontal: true,
                    min: 10.0,
                    max: 50.0,
                    step: 1.0,
                    default_value: SliderValue::Single(current_value()),
                    on_value_change: move |value: SliderValue| {
                        let SliderValue::Single(v) = value;
                        current_value.set(v);
                        properties_order.get_mut(index).unwrap().width = v;
                    },
                    SliderTrack {
                        SliderRange {}
                        SliderThumb {}
                    }
                }
            }
        }
    }
}
