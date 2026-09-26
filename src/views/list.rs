use crate::components::button::*;
use crate::protos::anytype_model::block::content::dataview::view::Type;

use crate::components::column::*;
use crate::components::edit_view::*;
use crate::components::header::{Header, Title};
use crate::components::row::*;
use crate::views::objects::ObjectsList;

use crate::components::select::*;
use crate::helpers::*;
use crate::views::calendar::Calendar;
use dioxus::prelude::*;
use dioxus_icons::lucide::Settings2;
use dioxus_sdk_storage::LocalStorage;
use dioxus_sdk_storage::use_synced_storage;
use std::collections::HashMap;
use std::vec;

#[component]
pub fn List(space_id: ReadSignal<String>, list_id: ReadSignal<String>) -> Element {
    let api_client = use_context::<ApiClient>();
    use_resource(move || {
        let _reconnect = RECONNECT_COUNT.read();
        let client = api_client.0.read().as_ref().cloned();
        async move {
            let Some(client) = client else {
                tracing::warn!("object_open failed: no client");
                return;
            };
            if let Err(e) = client.object_open(&space_id(), &list_id()).await {
                tracing::error!("object_open failed: {e:#}");
            }
        }
    });

    use_drop(move || {
        *SET_META.write() = SetMetaState::default();
        spawn(async move {
            let Some(client) = api_client.0.read().as_ref().cloned() else {
                tracing::warn!("object_close failed: no client");
                return;
            };
            if let Err(e) = client.object_close(&space_id(), &list_id()).await {
                tracing::error!("object_close failed: {e:#}");
            }
        });
    });
    let view_id = use_memo(move || SET_META.resolve().active_view_id().cloned());
    let open_edit = use_signal(|| true);

    rsx! {
        Column {
            ListHeader { space_id, list_id, open_edit }
            for _ in [()] {
                ListWithView {
                    key: "{list_id}-{view_id}",
                    list_id,
                    space_id,
                    view_id,
                    open_edit,
                }
            }
        }
    }
}
#[component]
pub fn ListWithView(
    space_id: ReadSignal<String>,
    list_id: ReadSignal<String>,
    view_id: ReadSignal<String>,
    open_edit: ReadSignal<bool>,
) -> Element {
    let api_client = use_context::<ApiClient>();
    let storage_view_tree_key = format!(
        "list-view-relations-tree-list-{}-view-{}",
        list_id(),
        view_id()
    );

    let mut positions =
        use_synced_storage::<LocalStorage, TileTree>(storage_view_tree_key.clone(), || TileTree {
            root: NodeId(0),
            nodes: HashMap::from([
                (
                    NodeId(0),
                    Node::Split {
                        parent: None,
                        direction: SplitDirection::Row,
                        ratio: 0.5,
                        first: NodeId(1),
                        second: NodeId(2),
                    },
                ),
                (
                    NodeId(1),
                    Node::Pane {
                        parent: Some(NodeId(0)),
                        relation_key: RelationKey("name".to_string()),
                    },
                ),
                (
                    NodeId(2),
                    Node::Pane {
                        parent: Some(NodeId(0)),
                        relation_key: RelationKey("description".to_string()),
                    },
                ),
            ]),
        });
    let positions_store = use_store(|| positions.read().clone());

    use_effect(move || {
        let store_value = positions_store.read().clone();
        *positions.write() = store_value;
    });

    let all_properties_res = use_resource(move || async move {
        let client = api_client
            .0
            .read()
            .as_ref()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("No API client available"))?;
        client.fetch_properties(&space_id()).await
    });
    let all_properties: Memo<HashMap<RelationKey, RelationInfo>> = use_memo(move || {
        all_properties_res
            .read()
            .as_ref()
            .and_then(|r| r.as_ref().ok())
            .cloned()
            .unwrap_or_default()
    });

    match &*all_properties_res.read_unchecked() {
        None => return rsx! { "Loading..." },
        Some(Err(e)) => return rsx! { "Error: {e}" },
        Some(Ok(_)) => {}
    }

    rsx! {
        Column {
            EditView {
                open: open_edit,
                space_id,
                list_id,
                positions,
                all_properties,
            }

            Objects {
                space_id,
                list_id,
                view_id,
                all_properties,
                positions,
            }
        }
    }
}

#[component]
pub fn ListHeader(
    space_id: ReadSignal<String>,
    list_id: ReadSignal<String>,
    open_edit: Signal<bool>,
) -> Element {
    let name = SET_META.resolve().name();
    rsx! {
        Row {
            Row { position: RowPosition::Middle,
                Title { title: "{name}" }
            }
            Row { position: RowPosition::Right,
                Views { list_id, space_id }
                Button {
                    variant: ButtonVariant::Secondary,
                    onclick: move |_| open_edit.toggle(),
                    aria_label: "Edit view",
                    Settings2 {}
                }
            }
        }
    }
}

#[component]
pub fn Views(list_id: ReadSignal<String>, space_id: ReadSignal<String>) -> Element {
    let api_client = use_context::<ApiClient>();
    let views: Vec<(String, String, i32)> = SET_META
        .resolve()
        .views()
        .iter()
        .map(|view| {
            (
                view().id.clone(),
                view().name.clone(),
                view().r#type.clone(),
            )
        })
        .collect();

    let selected = use_memo(move || Some(SET_META.read().active_view_id.clone()));
    let selected_signal: ReadSignal<Option<String>> = selected.into();
    rsx! {
        Select::<String> {
            value: Some(selected_signal),
            on_value_change: move |new_id: Option<String>| {
                if let Some(id) = new_id {
                    // view_id.set(id.clone());
                    SET_META.write().active_view_id = id.clone();
                    spawn(async move {
                        if let Some(client) = api_client.0.read().as_ref().cloned() {
                            client.set_active_view(&space_id(), &list_id(), &id).await.ok();
                        }
                    });
                }
            },
            SelectTrigger { SelectValue {} }
            SelectList {
                SelectGroup {
                    for (i, (view_id, view_name, view_type)) in views.into_iter().enumerate() {
                        SelectOption::<String> {
                            key: "{view_id}",
                            index: i,
                            value: view_id.clone(),
                            text_value: view_name.clone(),
                            "{view_name}"
                            SelectItemIndicator {}
                        }
                    }
                }
            }
        }
    }
}
#[component]
pub fn Objects(
    space_id: ReadSignal<String>,
    list_id: ReadSignal<String>,
    view_id: ReadSignal<String>,
    positions: ReadSignal<TileTree>,
    all_properties: ReadSignal<HashMap<RelationKey, RelationInfo>>,
) -> Element {
    let _reconnect = RECONNECT_COUNT.read();
    let view_type = {
        let meta = SET_META.read();
        meta.views
            .iter()
            .find(|v| v.id == meta.active_view_id)
            .map(|v| Type::try_from(v.r#type).unwrap_or_default())
            .unwrap_or_default()
    };
    match view_type {
        Type::List | Type::Table => rsx! {
            ObjectsList {
                space_id,
                list_id,
                view_id,
                all_properties,
                positions,
            }
        },
        Type::Calendar => rsx! {
            Calendar {
                space_id,
                list_id,
                view_id,
                positions,
                all_properties,
            }
        },
        _ => rsx! {},
    }
}
