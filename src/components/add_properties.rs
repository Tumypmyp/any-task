use crate::components::button::*;
use crate::components::row::*;
use crate::helpers::*;
use dioxus::prelude::*;
use std::vec;
#[component]
pub fn AddProperties(
    properties: Store<Vec<Vec<(PropertyInfo, PropertySettings)>>>,
    all_properties: Store<Vec<PropertyInfo>>,
) -> Element {
    rsx! {
        Row { position: Position::Middle,
            Button { variant: ButtonVariant::Secondary, "Add Properties" }
        }
        for property in all_properties.read().clone().iter() {
            ShowProperty {
                key: "{property.id.as_str()}",
                property: property.clone(),
                properties,
            }
        }
    }
}
#[component]
pub fn ShowProperty(
    property: PropertyInfo,
    properties: Store<Vec<Vec<(PropertyInfo, PropertySettings)>>>,
) -> Element {
    let name = property.clone().name;
    rsx! {
        Button {
            variant: ButtonVariant::Ghost,
            onclick: move |_| {
                let settings = match property.optional {
                    OptionalInfo::Date => PropertySettings::Date(DateSettings::default()),
                    OptionalInfo::Checkbox => {
                        PropertySettings::Checkbox(CheckboxSettings::default())
                    }
                    _ => PropertySettings::default(),
                };
                properties
                    .with_mut(|v| {
                        if let Some(last_row) = v.last_mut() {
                            last_row.push((property.clone(), settings));
                        } else {
                            v.push(vec![(property.clone(), settings)]);
                        }
                    });
            },
            "{name}"
        }
    }
}
