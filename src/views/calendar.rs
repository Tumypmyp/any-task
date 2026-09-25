use crate::components::button::*;
use crate::components::column::*;
use crate::components::object::*;
use crate::components::row::*;
use crate::components::scroll_area::*;
use dioxus_primitives::scroll_area::ScrollDirection;
use dioxus_primitives::scroll_area::ScrollType;

use crate::helpers::*;
use crate::protos::anytype_model::block::content::dataview::Filter;
use dioxus::prelude::*;
use std::collections::HashMap;
use std::vec;

#[component]
pub fn Calendar(
    space_id: ReadSignal<String>,
    list_id: ReadSignal<String>,
    view_id: ReadSignal<String>,
    positions: ReadSignal<TileTree>,
    all_properties: ReadSignal<HashMap<RelationKey, RelationInfo>>,
) -> Element {
    let keys = use_memo(move || pane_keys(&positions.read()));

    let today_ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let mut selected_date = use_signal(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    });
    let date_relation_key = use_memo(move || {
        let meta = SET_META.read();
        meta.views
            .iter()
            .find(|v| v.id == meta.active_view_id)
            .map(|v| v.group_relation_key.clone())
            .unwrap_or_default()
    });
    let api_client = use_context::<ApiClient>();
    use_resource(move || {
        let _reconnect = RECONNECT_COUNT.read();
        let sid = space_id.read().clone();
        let lid = list_id.read().clone();

        let date_ts = selected_date();
        let date_key = date_relation_key();

        tracing::debug!("date_ts: {}, date_key: {}", date_ts, date_key);
        let meta = SET_META.read();
        let set_of_ids = meta.set_of.clone();
        let active_view_id = meta.active_view_id.clone();
        let (mut filters, sorts) = meta
            .views
            .iter()
            .find(|v| v.id == active_view_id)
            .map(|v| (v.filters.clone(), v.sorts.clone()))
            .unwrap_or_default();

        if !date_key.is_empty() {
            filters.push(Filter {
                id: String::new(),
                operator: 0,
                relation_key: date_key,
                relation_property: String::new(),
                condition: 1, // Equal
                value: Some(prost_types::Value {
                    kind: Some(prost_types::value::Kind::NumberValue(date_ts as f64)),
                }),
                quick_option: 0, // ExactDate
                format: 4,
                include_time: false,
                nested_filters: vec![],
            });
        }
        drop(meta);

        let client = api_client.0.read().as_ref().cloned();
        async move {
            let Some(client) = client else { return };
            if set_of_ids.is_empty() {
                return;
            }
            // preload first objects of a set
            let phase1 = tokio::spawn({
                let client = client.clone();
                let sid = sid.clone();
                let lid = lid.clone();
                let set_of_ids = set_of_ids.clone();
                let keys = keys();
                let filters = filters.clone();
                let sorts = sorts.clone();
                async move {
                    client
                        .subscribe_list_objects(&sid, &lid, set_of_ids, keys, filters, sorts, 25)
                        .await
                }
            });

            match phase1.await {
                Ok(Ok(resp)) => {
                    let mut new_order = Vec::new();
                    let mut new_details = HashMap::new();
                    for record in resp.records {
                        let id = extract_string(record.fields.get("id"));
                        let det = parse_object_details(&id, &record.fields);
                        new_order.push(id.clone());
                        new_details.insert(id, det);
                    }
                    let mut state = LIST_OBJECTS.write();
                    state.order = new_order;
                    state.details = new_details;
                }
                Ok(Err(e)) => tracing::error!("subscribe_list_objects phase1: {e:#}"),
                Err(e) => tracing::error!("subscribe_list_objects phase1 panicked: {e}"),
            }
        }
    });

    use_drop(move || {
        *LIST_OBJECTS.write() = ListObjectsState::default();
        let lid = list_id.peek().clone();
        spawn(async move {
            if let Some(client) = api_client.0.read().as_ref().cloned() {
                client.unsubscribe_list_objects(&lid).await.ok();
            }
        });
    });
    let items: Vec<ObjectDetails> = {
        let state = LIST_OBJECTS.read();
        state
            .order
            .iter()
            .filter_map(|id| state.details.get(id).cloned())
            .collect()
    };
    let mut first_run = use_signal(|| true);
    use_effect(move || {
        let _ = selected_date();
        let behavior = if *first_run.peek() {
            "instant"
        } else {
            "smooth"
        };
        *first_run.write() = false;
        spawn(async move {
            let _ = document::eval(&format!(
                r#"document.getElementById("selected-day")?.scrollIntoView({{ inline: "center", behavior: "{behavior}", block: "nearest" }})"#,
            ))
            .await;
        });
    });
    const SECONDS_PER_DAY: i64 = 86_400;
    const DAYS_RANGE: i64 = 20;
    rsx! {
        Column { style: "width: 98vw;",
            ScrollArea {
                style: "width: 100%; overflow-x: auto; overflow-y: hidden; \
                        scrollbar-width: none; -ms-overflow-style: none;",
                direction: ScrollDirection::Horizontal,
                scroll_type: ScrollType::Hidden,
                // style: "width: 100%;",
                Row {
                    for offset in -DAYS_RANGE..=DAYS_RANGE {
                        {
                            let day_ts = selected_date() + offset * SECONDS_PER_DAY;
                            let is_selected = offset == 0;
                            let today = (today_ts / SECONDS_PER_DAY) == (day_ts / SECONDS_PER_DAY);
                            let weekend = is_weekend(day_ts);
                            rsx! {
                                Button {
                                    key: "{day_ts}",
                                    id: if is_selected { "selected-day" } else { "" },
                                    variant: if is_selected { ButtonVariant::Primary }
                                    else if today { ButtonVariant::Outline }
                                    else if weekend { ButtonVariant::Destructive }
                                    else { ButtonVariant::Ghost },
                                    onclick: move |_| {
                                        selected_date.set(day_ts);
                                    },
                                    "{day_of_month(day_ts)} {month_short_name(day_ts)}"
                                }
                            }
                        }
                    }
                }
            }
            for det in items {
                Object {
                    key: "{det.id}",
                    positions,
                    details: det,
                    all_properties,
                }
            }
        }
    }
}

use time::{OffsetDateTime, Weekday};

fn day_of_month(unix_secs: i64) -> u32 {
    OffsetDateTime::from_unix_timestamp(unix_secs)
        .map(|dt| dt.day() as u32)
        .unwrap_or(0)
}

fn month_short_name(unix_secs: i64) -> &'static str {
    OffsetDateTime::from_unix_timestamp(unix_secs)
        .map(|dt| match dt.month() {
            time::Month::January => "Jan",
            time::Month::February => "Feb",
            time::Month::March => "Mar",
            time::Month::April => "Apr",
            time::Month::May => "May",
            time::Month::June => "Jun",
            time::Month::July => "Jul",
            time::Month::August => "Aug",
            time::Month::September => "Sep",
            time::Month::October => "Oct",
            time::Month::November => "Nov",
            time::Month::December => "Dec",
        })
        .unwrap_or("")
}

fn pane_keys(tree: &TileTree) -> Vec<String> {
    let mut keys = vec!["id".to_string(), "name".to_string()];
    for node in tree.nodes.values() {
        if let Node::Pane { relation_key, .. } = node {
            let k = relation_key.as_str().to_string();
            if !keys.contains(&k.clone()) {
                keys.push(k.to_string());
            }
        }
    }
    keys
}

fn is_weekend(unix_secs: i64) -> bool {
    OffsetDateTime::from_unix_timestamp(unix_secs)
        .map(|dt| matches!(dt.weekday(), Weekday::Saturday | Weekday::Sunday))
        .unwrap_or(false)
}
