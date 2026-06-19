use crate::protos::anytype_model::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct RelationKey(pub String);
impl RelationKey {
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn empty() -> Self {
        Self(String::new())
    }
}

impl Default for RelationKey {
    fn default() -> Self {
        Self::empty()
    }
}
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct TileTree {
    pub nodes: HashMap<NodeId, Node>,
    pub root: NodeId,
}
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Node {
    Split {
        parent: Option<NodeId>,
        direction: SplitDirection,
        ratio: f32,
        first: NodeId,
        second: NodeId,
    },
    Pane {
        parent: Option<NodeId>,
        relation_key: RelationKey,
    },
}

impl Node {
    pub fn set_parent(&mut self, new_parent: Option<NodeId>) {
        match self {
            Node::Split { parent, .. } => *parent = new_parent,
            Node::Pane { parent, .. } => *parent = new_parent,
        }
    }
    pub fn parent(&self) -> Option<NodeId> {
        match self {
            Node::Split { parent, .. } => *parent,
            Node::Pane { parent, .. } => *parent,
        }
    }
}

#[derive(Copy, PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct NodeId(pub u32);

#[derive(PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SplitDirection {
    Row,
    Column,
}

impl TileTree {
    fn generate_2_ids(&self) -> (NodeId, NodeId) {
        let max_val = self
            .nodes
            .keys()
            .map(|node_id| node_id.0)
            .max()
            .unwrap_or(0);
        (NodeId(max_val + 1), NodeId(max_val + 2))
    }
    pub fn remove_node(&mut self, node_id: NodeId) {
        let parent_id = match self
            .nodes
            .remove(&node_id)
            .expect("node should exist")
            .parent()
        {
            Some(p) => p,
            None => return,
        };

        let (first, second, grand_parent) = match self.nodes.remove(&parent_id) {
            Some(Node::Split {
                first,
                second,
                parent,
                ..
            }) => (first, second, parent),
            _ => return,
        };

        let sibling_id = if node_id == first { second } else { first };

        if let Some(sibling) = self.nodes.get_mut(&sibling_id) {
            sibling.set_parent(grand_parent);
        }

        let Some(gp_id) = grand_parent else {
            self.root = sibling_id;
            return;
        };
        if let Some(Node::Split { first, second, .. }) = self.nodes.get_mut(&gp_id) {
            if *first == parent_id {
                *first = sibling_id;
            } else if *second == parent_id {
                *second = sibling_id;
            }
        }
    }

    pub fn add_right(&mut self, node_id: NodeId) -> () {
        let (id_first, id_second) = self.generate_2_ids();
        let Node::Pane {
            parent,
            relation_key,
        } = self
            .nodes
            .get(&node_id)
            .expect("current node dissapered from tree")
            .clone()
        else {
            return;
        };
        self.nodes.insert(
            id_second,
            Node::Pane {
                parent: Some(node_id),
                relation_key: RelationKey::default(),
            },
        );
        self.nodes.insert(
            node_id,
            Node::Split {
                parent: parent,
                direction: SplitDirection::Row,
                ratio: 0.5,
                first: id_first,
                second: id_second,
            },
        );
        self.nodes.insert(
            id_first,
            Node::Pane {
                parent: Some(node_id),
                relation_key,
            },
        );
    }
    pub fn add_up(&mut self, node_id: NodeId) -> () {
        let (id_first, id_second) = self.generate_2_ids();
        let Node::Pane {
            parent,
            relation_key,
        } = self
            .nodes
            .get(&node_id)
            .expect("parent node dissapered from tree")
            .clone()
        else {
            return;
        };
        self.nodes.insert(
            id_first,
            Node::Pane {
                parent: Some(node_id),
                relation_key: RelationKey::default(),
            },
        );
        self.nodes.insert(
            id_second,
            Node::Pane {
                parent: Some(node_id),
                relation_key,
            },
        );
        self.nodes.insert(
            node_id,
            Node::Split {
                parent,
                direction: SplitDirection::Column,
                ratio: 0.5,
                first: id_first,
                second: id_second,
            },
        );
    }
    pub fn add_down(&mut self, node_id: NodeId) -> () {
        let (id_first, id_second) = self.generate_2_ids();
        let Node::Pane {
            parent,
            relation_key,
        } = self
            .nodes
            .get(&node_id)
            .expect("parent node dissapered from tree")
            .clone()
        else {
            return;
        };
        self.nodes.insert(
            id_second,
            Node::Pane {
                parent: Some(node_id),
                relation_key: RelationKey::default(),
            },
        );
        self.nodes.insert(
            id_first,
            Node::Pane {
                parent: Some(node_id),
                relation_key,
            },
        );
        self.nodes.insert(
            node_id,
            Node::Split {
                parent,
                direction: SplitDirection::Column,
                ratio: 0.5,
                first: id_first,
                second: id_second,
            },
        );
    }
    pub fn add_left(&mut self, node_id: NodeId) -> () {
        let (id_first, id_second) = self.generate_2_ids();
        let Node::Pane {
            parent,
            relation_key,
        } = self
            .nodes
            .get(&node_id)
            .expect("parent node dissapered from tree")
            .clone()
        else {
            return;
        };
        self.nodes.insert(
            id_first,
            Node::Pane {
                parent: Some(node_id),
                relation_key: RelationKey::default(),
            },
        );
        self.nodes.insert(
            id_second,
            Node::Pane {
                parent: Some(node_id),
                relation_key,
            },
        );
        self.nodes.insert(
            node_id,
            Node::Split {
                parent,
                direction: SplitDirection::Row,
                ratio: 0.5,
                first: id_first,
                second: id_second,
            },
        );
    }
}

// #[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
// pub enum DateTimeFormat {
//     #[default]
//     DateTime,
//     Date,
//     Time,
// }
pub const NAME_RELATION_KEY: &str = "name";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationInfo {
    pub key: RelationKey,
    pub name: String,
    pub format: RelationFormat,
}

// #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
// pub enum OptionalInfo {
//     Date,
//     Checkbox,
//     #[default]
//     Other,
// }
// #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
// pub struct GeneralPropertySettings {
// }
// #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
// pub struct DateSettings {
//     pub general: GeneralPropertySettings,
//     pub date_format: DateTimeFormat,
// }
// #[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
// pub struct CheckboxSettings {
//     pub size: f64,
// }
// impl Default for CheckboxSettings {
//     fn default() -> Self {
//         Self { size: 40.0 }
//     }
// }
// impl Default for DateSettings {
//     fn default() -> Self {
//         Self {
//             date_format: DateTimeFormat::Date,
//             general: GeneralPropertySettings {
//            },
//         }
//     }
// }
// impl Default for GeneralPropertySettings {
//     fn default() -> Self {
//         Self {
//        }
//     }
// }
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// pub enum PropertySettings {
//     General(GeneralPropertySettings),
//     Date(DateSettings),
//     Checkbox(CheckboxSettings),
// }
// impl Default for PropertySettings {
//     fn default() -> Self {
//         Self::General(GeneralPropertySettings::default())
//     }
// }
// pub const NAME_PROPERTY_SETTINGS: PropertySettings =
//     PropertySettings::General(GeneralPropertySettings {
//    });

// pub trait PropertyRenderer {
//     fn render(
//         &self,
//         space_id: String,
//         object_id: String,
//         info: RelationInfo,
//         settings: PropertySettings,
//     ) -> Element;
// }
// #[derive(Clone, Debug, PartialEq)]
// pub struct ViewInfo {
//     pub id: String,
//     pub name: String,
// }

#[derive(Clone, PartialEq, Default)]
pub struct SpacesState {
    pub order: Vec<String>,
    pub details: HashMap<String, SpaceDetails>,
}

#[derive(Clone, PartialEq)]
pub struct SpaceDetails {
    pub object_id: String,
    pub target_space_id: String,
    pub name: String,
    pub icon_image: String,
    pub description: String,
}

#[derive(Clone, PartialEq, Default)]
pub struct SetsState {
    pub order: Vec<String>,
    pub details: HashMap<String, SetDetails>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SetDetails {
    pub object_id: String,
    pub name: String,
    pub layout: i32,
    pub set_of: Vec<String>,
}

#[derive(Clone, PartialEq, Default)]
pub struct ListObjectsState {
    pub order: Vec<String>,
    pub details: HashMap<String, ObjectDetails>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ObjectDetails {
    pub id: String,
    pub name: String,
    pub fields: std::collections::BTreeMap<String, prost_types::Value>,
}
use crate::protos::anytype_model::block::*;
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SetMetaState {
    pub id: String,
    pub name: String,
    pub set_of: Vec<String>,
    pub views: HashMap<String, content::dataview::View>,
    pub view_order: Vec<String>,
    pub active_view_id: String,
}
