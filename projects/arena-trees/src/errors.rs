
pub type Result<T> = std::result::Result<T, TreeError>;

pub enum TreeError {
    /// Try to append sibling nodes to the root node
    RootSiblingNode,
}


