use std::slice::SliceIndex;

type NodeID = usize;

mod links;
mod arena;

#[derive(Debug)]
pub struct NodeArena<T> {
    pub(crate) nodes: Vec<NodeData<T>>,
    pub(crate) empty: Vec<NodeID>,
}

#[derive(Debug)]
pub struct NodeData<T> {
    pub link: NodeLink,
    pub data: T,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct NodeLink {
    /// The ID of the parent node, or `None` if this is the root node.
    pub parent: Option<NodeID>,
    /// The ID of the left sibling node, or `None` if this node has no left sibling.
    pub left_sibling: Option<NodeID>,
    /// The ID of the right sibling node, or `None` if this node has no right sibling.
    pub right_sibling: Option<NodeID>,
    /// The ID of the first child node, or `None` if this node has no children.
    pub first_child: Option<NodeID>,
    /// The ID of the last child node, or `None` if this node has no children.
    pub last_child: Option<NodeID>,
}

