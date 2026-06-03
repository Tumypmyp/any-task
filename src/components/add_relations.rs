use crate::components::button::*;
use crate::components::row::*;
use crate::helpers::*;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::vec;
#[component]
pub fn AddRelations(
    properties: Store<HashMap<RelationKey, (RelationInfo, PropertySettings)>>,
    all_properties: ReadSignal<Vec<RelationInfo>>,
) -> Element {
    rsx! {
        Row { position: Position::Middle,
            Button { variant: ButtonVariant::Secondary, "Add Properties" }
        }
        for property in all_properties.read().clone().iter() {
            ShowProperty {
                key: "{property.key.as_str()}",
                property: property.clone(),
                properties,
            }
        }
    }
}
#[component]
pub fn ShowProperty(
    property: RelationInfo,
    properties: Store<HashMap<RelationKey, (RelationInfo, PropertySettings)>>,
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
                        v.insert(property.clone().key, (property.clone(), settings));
                    });
            },
            "{name}"
        }
    }
}
