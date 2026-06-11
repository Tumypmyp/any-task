use crate::components::button::*;
use crate::components::choose_view::ChooseView;
use crate::components::clean_positions::*;
use crate::components::column::*;
use crate::components::edit_relations::*;
use crate::components::row::*;
use crate::components::scroll_area::*;
use crate::components::separator::Separator;
use crate::components::sheet::*;
use crate::helpers::*;
use dioxus::prelude::*;
use dioxus_primitives::scroll_area::ScrollDirection;
use std::vec;
#[component]
pub fn EditView(
    space_id: String,
    list_id: String,
    view_id: Store<String>,
    positions: Store<TileTree>,
    all_properties: ReadSignal<Vec<RelationInfo>>,
) -> Element {
    let mut open = use_signal(|| false);
    rsx! {
        Button {
            variant: ButtonVariant::Secondary,
            onclick: move |_| open.set(true),
            "Edit view"
        }
        Sheet { open: open(), on_open_change: move |v| open.set(v),
            SheetContent {
                side: SheetSide::Bottom,
                style: "min-height: 50vh; max-height: 80vh;",
                // ScrollArea {
                //     direction: ScrollDirection::Vertical,
                //     style: "min-height: 50vh; max-height: 70vh;",
                Column {
                    Row { position: RowPosition::Middle,
                        ChooseView { space_id, list_id, view_id }
                        CleanPositions { positions }
                    }
                    Separator {}

                    EditPositions {
                        id: positions().root,
                        positions,
                        all_properties,
                    }
                }
                // }
            }
        }
    }
}
