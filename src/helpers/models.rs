use crate::protos::anytype_model::*;
use anyhow::*;
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
    fn next_id(&self) -> NodeId {
        let max_val = self
            .nodes
            .keys()
            .map(|node_id| node_id.0)
            .max()
            .unwrap_or(0);
        NodeId(max_val + 1)
    }
    fn generate_2_ids(&self) -> (NodeId, NodeId) {
        let max_val = self
            .nodes
            .keys()
            .map(|node_id| node_id.0)
            .max()
            .unwrap_or(0);
        (NodeId(max_val + 1), NodeId(max_val + 2))
    }
    pub fn clean(&mut self) -> () {
        self.root = NodeId(0);
        self.nodes.clear();
        self.nodes.insert(
            NodeId(0),
            Node::Pane {
                parent: None,
                relation_key: RelationKey("name".to_string()),
            },
        );
    }
    pub fn remove_node(&mut self, node_id: NodeId) -> Result<()> {
        let parent_id = self
            .nodes
            .get(&node_id)
            .context("node not found")?
            .parent()
            .context("node is the root (has no parent)")?;
        self.nodes.remove(&node_id);
        let (first, second, grand_parent) = match self.nodes.remove(&parent_id) {
            Some(Node::Split {
                first,
                second,
                parent,
                ..
            }) => (first, second, parent),
            _ => anyhow::bail!("parent node {parent_id:?} is not a Split node"),
        };

        let sibling_id = if node_id == first { second } else { first };

        if let Some(sibling) = self.nodes.get_mut(&sibling_id) {
            sibling.set_parent(grand_parent);
        }

        match grand_parent {
            None => self.root = sibling_id,
            Some(gp_id) => {
                if let Some(Node::Split { first, second, .. }) = self.nodes.get_mut(&gp_id) {
                    if *first == parent_id {
                        *first = sibling_id;
                    } else if *second == parent_id {
                        *second = sibling_id;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn add_pane_at(&mut self, node_id: NodeId, zone: DropZone, key: RelationKey) {
        let direction = match zone {
            DropZone::Top | DropZone::Bottom => SplitDirection::Column,
            DropZone::Left | DropZone::Right => SplitDirection::Row,
        };
        let first_is_new = matches!(zone, DropZone::Top | DropZone::Left);

        let new_pane_id = self.next_id();
        self.nodes.insert(
            new_pane_id,
            Node::Pane {
                parent: None, // will be set below
                relation_key: key,
            },
        );

        let new_split_id = self.next_id();
        let (first, second) = if first_is_new {
            (new_pane_id, node_id)
        } else {
            (node_id, new_pane_id)
        };

        // find parent of node_id
        let parent = self.nodes.get(&node_id).and_then(|n| match n {
            Node::Pane { parent, .. } | Node::Split { parent, .. } => *parent,
        });

        self.nodes.insert(
            new_split_id,
            Node::Split {
                parent,
                direction,
                ratio: 0.5,
                first,
                second,
            },
        );

        // update children's parent pointer
        if let Some(n) = self.nodes.get_mut(&new_pane_id) {
            if let Node::Pane { parent: p, .. } = n {
                *p = Some(new_split_id);
            }
        }
        if let Some(n) = self.nodes.get_mut(&node_id) {
            match n {
                Node::Pane { parent: p, .. } | Node::Split { parent: p, .. } => {
                    *p = Some(new_split_id)
                }
            }
        }

        // update grandparent or root
        match parent {
            Some(gp_id) => {
                if let Some(Node::Split {
                    first: f,
                    second: s,
                    ..
                }) = self.nodes.get_mut(&gp_id)
                {
                    if *f == node_id {
                        *f = new_split_id;
                    } else if *s == node_id {
                        *s = new_split_id;
                    }
                }
            }
            None => {
                self.root = new_split_id;
            }
        }
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

#[derive(Clone, PartialEq)]
pub struct SpaceDetails {
    pub object_id: String,
    pub target_space_id: String,
    pub name: String,
    pub icon_image: String,
    pub description: String,
}

#[derive(Clone, PartialEq, Debug, Default)]
pub struct NewPaneDrag {
    pub is_dragging: bool,
    pub hover_node: Option<NodeId>,
    pub drop_zone: Option<DropZone>,
    pub dragging_node: Option<NodeId>,
    pub dragging_key: Option<RelationKey>,
    pub hover_delete: bool,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum DropZone {
    Top,
    Bottom,
    Left,
    Right,
}
