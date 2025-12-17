use crate::components::button::{Button, ButtonHolder, ButtonVariant};
use crate::components::separator::*;
use crate::components::label::*;
use crate::components::list::*;
use crate::components::row::*;
use crate::components::slider::*;
use crate::helpers::*;
use dioxus::prelude::*;
use std::vec;
#[component]
pub fn PropertiesRow(properties: Store<Vec<PropertyInfo>>) -> Element {
    rsx! {
        List {
            for (index , property) in properties.read().clone().iter().enumerate() {
                if property.show {
                    Property {
                        key: "{property.id.as_str()}",
                        index,
                        id: property.id.clone(),
                        properties,
                    }
                    Separator {}
                }
            }
        }
    }
}
#[component]
pub fn Property(index: usize, id: PropertyID, properties: Store<Vec<PropertyInfo>>) -> Element {
    let property = properties.get(index).unwrap();
    // let mut current_value = use_signal(|| property().width);
    rsx! {
        Row {
            Button { variant: ButtonVariant::Secondary,
                // width: "{current_value}vw",
                "{property().clone().name}"
            }
            Button {
                variant: ButtonVariant::Destructive,
                onclick: move |_| {
                    properties
                        .with_mut(|v| {
                            if let Some(property) = v.iter_mut().find(|p| p.id == id) {
                                property.show = !property.show;
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
            horizontal: true,
            min: 5.0,
            max: 50.0,
            step: 1.0,
            // default_value: SliderValue::Single((properties.get(index))().unwrap().width),
            default_value: SliderValue::Single(15.0),
            on_value_change: move |value: SliderValue| {
                let SliderValue::Single(v) = value;
                // current_value.set(v);
                properties.get_mut(index).unwrap().width = v;
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
            horizontal: true,
            min: 5.0,
            max: 50.0,
            step: 1.0,
            default_value: SliderValue::Single(10.0),
            // default_value: SliderValue::Single((properties.get(index))().unwrap().height),
            on_value_change: move |value: SliderValue| {
                let SliderValue::Single(v) = value;
                // current_value.set(v);
                properties.get_mut(index).unwrap().height = v;
            },
            SliderTrack {
                SliderRange {}
                SliderThumb {}
            }
        }
    }
}
