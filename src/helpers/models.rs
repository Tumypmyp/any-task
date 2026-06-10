use dioxus::prelude::*;
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
pub struct TileTree(pub HashMap<NodeId, Node>);
impl TileTree {
    fn generate_2_ids(&self) -> (NodeId, NodeId) {
        let max_val = self.0.keys().map(|node_id| node_id.0).max().unwrap_or(0);
        (NodeId(max_val + 1), NodeId(max_val + 2))
    }
    pub fn add_right(&mut self, parent_id: NodeId) -> () {
        let (id_first, id_second) = self.generate_2_ids();
        let val = self
            .0
            .get(&parent_id)
            .expect("parent node dissapered from tree")
            .clone();
        self.0.insert(
            id_second,
            Node::Pane {
                relation_key: RelationKey::default(),
            },
        );
        self.0.insert(id_first, val);
        self.0.insert(
            parent_id,
            Node::Split {
                direction: SplitDirection::Row,
                ratio: 0.5,
                first: id_first,
                second: id_second,
            },
        );
    }
    pub fn add_up(&mut self, parent_id: NodeId) -> () {
        let (id_first, id_second) = self.generate_2_ids();
        let val = self
            .0
            .get(&parent_id)
            .expect("parent node dissapered from tree")
            .clone();
        self.0.insert(
            id_first,
            Node::Pane {
                relation_key: RelationKey::default(),
            },
        );
        self.0.insert(id_second, val);
        self.0.insert(
            parent_id,
            Node::Split {
                direction: SplitDirection::Column,
                ratio: 0.5,
                first: id_first,
                second: id_second,
            },
        );
    }
    pub fn add_down(&mut self, parent_id: NodeId) -> () {
        let (id_first, id_second) = self.generate_2_ids();
        let val = self
            .0
            .get(&parent_id)
            .expect("parent node dissapered from tree")
            .clone();

        self.0.insert(
            id_second,
            Node::Pane {
                relation_key: RelationKey::default(),
            },
        );
        self.0.insert(id_first, val);
        self.0.insert(
            parent_id,
            Node::Split {
                direction: SplitDirection::Column,
                ratio: 0.5,
                first: id_first,
                second: id_second,
            },
        );
    }
    pub fn add_left(&mut self, parent_id: NodeId) -> () {
        let (id_first, id_second) = self.generate_2_ids();
        let val = self
            .0
            .get(&parent_id)
            .expect("parent node dissapered from tree")
            .clone();

        self.0.insert(
            id_first,
            Node::Pane {
                relation_key: RelationKey::default(),
            },
        );
        self.0.insert(id_second, val);
        self.0.insert(
            parent_id,
            Node::Split {
                direction: SplitDirection::Row,
                ratio: 0.5,
                first: id_first,
                second: id_second,
            },
        );
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Node {
    Split {
        direction: SplitDirection,
        ratio: f32,
        first: NodeId,
        second: NodeId,
    },
    Pane {
        relation_key: RelationKey,
    },
}

#[derive(Copy, PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct NodeId(pub u32);

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum SplitDirection {
    Row,
    Column,
}

// #[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize)]
// pub struct PropertyID(pub String);
// impl PropertyID {
//     pub fn as_str(&self) -> &str {
//         &self.0
//     }
// }
#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub enum DateTimeFormat {
    #[default]
    DateTime,
    Date,
    Time,
}
pub const NAME_RELATION_KEY: &str = "name";
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelationInfo {
    pub key: RelationKey,
    pub name: String,
    // pub optional: OptionalInfo,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum OptionalInfo {
    Date,
    Checkbox,
    #[default]
    Other,
}
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneralPropertySettings {
    pub width: f64,
    pub height: f64,
}
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateSettings {
    pub general: GeneralPropertySettings,
    pub date_format: DateTimeFormat,
}
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CheckboxSettings {
    pub size: f64,
}
impl Default for CheckboxSettings {
    fn default() -> Self {
        Self { size: 40.0 }
    }
}
impl Default for DateSettings {
    fn default() -> Self {
        Self {
            date_format: DateTimeFormat::Date,
            general: GeneralPropertySettings {
                width: 70.0,
                height: 50.0,
            },
        }
    }
}
impl Default for GeneralPropertySettings {
    fn default() -> Self {
        Self {
            width: 60.0,
            height: 40.0,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PropertySettings {
    General(GeneralPropertySettings),
    Date(DateSettings),
    Checkbox(CheckboxSettings),
}
impl Default for PropertySettings {
    fn default() -> Self {
        Self::General(GeneralPropertySettings::default())
    }
}
pub const NAME_PROPERTY_SETTINGS: PropertySettings =
    PropertySettings::General(GeneralPropertySettings {
        width: 100.0,
        height: 40.0,
    });
impl PropertySettings {
    pub fn height(&self) -> f64 {
        match self {
            Self::General(s) => s.height,
            Self::Date(s) => s.general.height,
            Self::Checkbox(s) => s.size,
        }
    }
    pub fn width(&self) -> f64 {
        match self {
            Self::General(s) => s.width,
            Self::Date(s) => s.general.width,
            Self::Checkbox(s) => s.size,
        }
    }
    pub fn set_height(&mut self, val: f64) {
        match self {
            Self::General(s) => s.height = val,
            Self::Date(s) => s.general.height = val,
            _ => {}
        }
    }
    pub fn set_width(&mut self, val: f64) {
        match self {
            Self::General(s) => s.width = val,
            Self::Date(s) => s.general.width = val,
            _ => {}
        }
    }
}
pub trait PropertyRenderer {
    fn render(
        &self,
        space_id: String,
        object_id: String,
        info: RelationInfo,
        settings: PropertySettings,
    ) -> Element;
}
#[derive(Clone, Debug, PartialEq)]
pub struct ViewInfo {
    pub id: String,
    pub name: String,
}
