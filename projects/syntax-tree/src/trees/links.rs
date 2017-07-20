use super::*;


impl NodeLink {
    /// Creates a new `Node` with the given parent node ID.
    ///
    /// # Arguments
    /// * `parent` - The ID of the parent node. If `None`, this node is the root node.
    ///
    /// # Returns
    /// A new `Node` instance with the given parent node ID, and default values for the other fields.
    pub fn new(parent: NodeID) -> Self {
        Self {
            parent: Some(parent),
            left: None,
            right: None,
            first_child: None,
            last_child: None,
        }
    }

    /// Creates a new `Node` with the given parent node ID and left sibling node ID.
    ///
    /// # Arguments
    /// * `parent` - The ID of the parent node.
    /// * `left` - The ID of the left sibling node.
    ///
    /// # Returns
    /// A new `Node` instance with the given parent and left sibling node IDs, and default values for the other fields.
    pub fn with_sibling_left(self, sibling: NodeID) -> Self {
        Self {
            left: Some(sibling),
            ..self
        }
    }

    /// Creates a new `Node` with the given parent node ID and without a left sibling.
    ///
    /// # Arguments
    /// * `parent` - The ID of the parent node.
    ///
    /// # Returns
    /// A new `Node` instance with the given parent node ID and no left sibling, and default values for the other fields.
    pub fn without_sibling_left(self) -> Self {
        Self {
            left: None,
            ..self
        }
    }

    /// Creates a new `Node` with the given parent node ID and right sibling node ID.
    ///
    /// # Arguments
    /// * `parent` - The ID of the parent node.
    /// * `right` - The ID of the right sibling node.
    ///
    /// # Returns
    /// A new `Node` instance with the given parent and right sibling node IDs, and default values for the other fields.
    pub fn with_sibling_right(self, sibling: NodeID) -> Self {
        Self {
            right: Some(sibling),
            ..self
        }
    }

    /// Creates a new `Node` with the given parent node ID and without a right sibling.
    ///
    /// # Arguments
    /// * `parent` - The ID of the parent node.
    ///
    /// # Returns
    /// A new `Node` instance with the given parent node ID and no right sibling, and default values for the other fields.
    pub fn without_sibling_right(self) -> Self {
        Self {
            right: None,
            ..self
        }
    }

    /// Creates a new `Node` with the given parent node ID and first child node ID.
    ///
    /// # Arguments
    /// * `parent` - The ID of the parent node.
    /// * `first_child` - The ID of the first child node.
    ///
    /// # Returns
    /// A new `Node` instance with the given parent and first child node IDs, and default values for the other fields.
    pub fn with_child_first(self, child: NodeID) -> Self {
        Self {
            first_child: Some(child),
            ..self
        }
    }

    /// Creates a new `Node` with the given parent node ID and without a first child.
    ///
    /// # Arguments
    /// * `parent` - The ID of the parent node.
    ///
    /// # Returns
    /// A new `Node` instance with the given parent node ID and no first child, and default values for the other fields.
    pub fn without_child_first(self) -> Self {
        Self {
            first_child: None,
            ..self
        }
    }

    /// Creates a new `Node` with the given parent node ID and last child node ID.
    ///
    /// # Arguments
    /// * `parent` - The ID of the parent node.
    /// * `last_child` - The ID of the last child node.
    ///
    /// # Returns
    /// A new `Node` instance with the given parent and last child node IDs, and default values for the other fields.
    pub fn with_child_last(self, child: NodeID) -> Self {
        Self {
            last_child: Some(child),
            ..self
        }
    }

    /// Creates a new `Node` with the given parent node ID and without a last child.
    ///
    /// # Arguments
    /// * `parent` - The ID of the parent node.
    ///
    /// # Returns
    /// A new `Node` instance with the given parent node ID and no last child, and default values for the other fields.
    pub fn without_child_last(self) -> Self {
        Self {
            last_child: None,
            ..self
        }
    }
}