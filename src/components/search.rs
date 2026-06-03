use crate::API_CLIENT;
use crate::ObjectRow;
use crate::components::column::Column;
use crate::helpers::*;
use dioxus::prelude::*;
use openapi::models::Object as ApimodelObject;
use std::vec;
struct Object {
    name: String,
    object_id: String,
    space_id: String,
    data: ApimodelObject,
}
#[component]
pub fn Search(space_id: String, types: Vec<String>) -> Element {
    let space_id2 = space_id.clone();
    let resp = use_resource(move || {
        let client = API_CLIENT.read().clone();
        let types = types.clone();
        let space_id = space_id2.clone();
        async move { client.get_types(space_id, types).await }
    });
    let Some(result) = &*resp.read() else {
        return rsx! { "Loading..." };
    };
    let objects: Vec<_> = match result {
        Ok(s) => s
            .data
            .as_ref()
            .map(|data| {
                data.iter()
                    .map(|o| Object {
                        name: o.name.clone().unwrap_or_default(),
                        object_id: o.id.clone().unwrap_or_default(),
                        space_id: space_id.clone(),
                        data: o.clone(),
                    })
                    .collect()
            })
            .unwrap_or_default(),
        Err(e) => {
            tracing::error!("Got error loading objects: {:#?}", e);
            return rsx! { "Error: {e}" };
        }
    };
    let properties: Store<Vec<Vec<(PropertyInfo, PropertySettings)>>> = use_store(|| {
        vec![vec![(
            PropertyInfo {
                id: PropertyID(NAME_PROPERTY_ID_STR.to_string()),
                name: "Name".to_string(),
                optional: OptionalInfo::Other,
            },
            NAME_PROPERTY_SETTINGS,
        )]]
    });
    rsx! {
        Column {
            for obj in objects.iter() {
                ObjectRow {
                    key: "{obj.object_id}",
                    name: obj.name.clone(),
                    space_id: obj.space_id.clone(),
                    object_id: obj.object_id.clone(),
                    properties,
                    data: obj.data.clone(),
                }
            }
        }
    }
}
