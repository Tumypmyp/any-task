use crate::components::button::*;
use crate::components::separator::*;
use crate::components::label::*;
use crate::components::list::*;
use crate::components::row::*;
use crate::components::slider::*;
use crate::helpers::*;
use dioxus::prelude::*;
use std::vec;
#[component]
pub fn PropertiesRow(properties: Store<Vec<(PropertyInfo, PropertySettings)>>) -> Element {
    rsx! {
        List {
            for (index , property) in properties.read().clone().iter().enumerate() {
                if property.1.show {
                    Property {
                        key: "{property.0.id.as_str()}",
                        index,
                        properties,
                    }
                    Separator {}
                }
            }
        }
    }
}
#[component]
pub fn Property(index: usize, properties: Store<Vec<(PropertyInfo, PropertySettings)>>) -> Element {
    let property = properties.get(index).unwrap();
    let name = property().0.name;
    rsx! {
        Row {
            Button { variant: ButtonVariant::Secondary, "{name}" }
            Button {
                variant: ButtonVariant::Destructive,
                onclick: move |_| {
                    properties
                        .with_mut(|v| {
                            if index < v.len() {
                                v.remove(index);
                            }
                        });
                },
                "X"
            }
        }
        Label { html_for: "width_slider", "Width" }
        Slider {
            id: "width_slider",
            label: "Property width",
            width: "50vw",
            horizontal: true,
            min: 5.0,
            max: 100.0,
            step: 1.0,
            default_value: SliderValue::Single(properties.get(index).unwrap()().1.width),
            on_value_change: move |value: SliderValue| {
                let SliderValue::Single(v) = value;
                properties.get_mut(index).unwrap().1.width = v;
            },
            SliderTrack {
                SliderRange {}
                SliderThumb {}
            }
        }
        Label { html_for: "height_slider", "Height" }
        Slider {
            id: "height_slider",
            label: "Property width",
            width: "50vw",
            horizontal: true,
            min: 5.0,
            max: 100.0,
            step: 1.0,
            default_value: SliderValue::Single(properties.get(index).unwrap()().1.height),
            on_value_change: move |value: SliderValue| {
                let SliderValue::Single(v) = value;
                properties.get_mut(index).unwrap().1.height = v;
            },
            SliderTrack {
                SliderRange {}
                SliderThumb {}
            }
        }
    }
}
