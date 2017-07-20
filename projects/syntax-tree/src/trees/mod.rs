use std::{ops::Range, slice::SliceIndex};

pub type NodeID = u32;
pub type Offset = u32;

mod arena;
mod links;

#[derive(Clone, Debug)]
pub struct TextArena {
    pub text: String,
    pub nodes: Vec<TextNode>,
    pub empty: Vec<NodeID>,
}

#[derive(Clone, Debug)]
pub struct TextNode {
    pub data: NodeData,
    pub link: NodeLink,
}

#[derive(Clone, Debug)]
pub struct NodeData {
    pub rule: u32,
    pub tag: String,
    pub head: Range<Offset>,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct NodeLink {
    /// The ID of the parent node, or `None` if this is the root node.
    pub parent: Option<NodeID>,
    /// The ID of the left sibling node, or `None` if this node has no left sibling.
    pub left: Option<NodeID>,
    /// The ID of the right sibling node, or `None` if this node has no right sibling.
    pub right: Option<NodeID>,
    /// The ID of the first child node, or `None` if this node has no children.
    pub first_child: Option<NodeID>,
    /// The ID of the last child node, or `None` if this node has no children.
    pub last_child: Option<NodeID>,
}
