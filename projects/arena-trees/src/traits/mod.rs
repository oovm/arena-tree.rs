use crate::arc::ArcNode;

pub enum TreeError {
    /// Try to append sibling nodes to the root node
    RootSiblingNode,
}

pub trait Node<T>
where
    Self: Sized,
{
    type Ancestor: Iterator<Item = Self>;

    /// Create a tree based on the given root node and capacity
    fn new(data: T, capacity: usize) -> Self;

    fn is_root(&self) -> bool {
        self.parent().is_none()
    }

    fn root(&self) -> Self;

    fn ancestor(&self, with_self: usize) -> Self::Ancestor;

    fn parent(&self) -> Option<Self>;

    fn is_leftmost(&self) -> bool;

    fn is_rightmost(&self) -> bool;

    fn leftmost(&self) -> Self;

    fn rightmost(&self) -> Self;

    /// Insert data to the left of the node, it does not need to be the first node
    ///
    /// Return new node
    fn insert_left(&self, data: T) -> Result<Self, TreeError> {
        match self.parent() {
            Some(s) => Ok(s.insert_before(data, self)),
            None => Err(TreeError::RootSiblingNode),
        }
    }

    fn insert_right(&self, data: T) -> Result<Self, TreeError> {
        match self.parent() {
            Some(s) => Ok(s.insert_after(data, self)),
            None => Err(TreeError::RootSiblingNode),
        }
    }

    ///  Return new node
    fn insert_child_left(&self, data: &T) -> Self;

    ///  Return new node
    fn insert_child_right(&self, data: &T) -> Self;

    ///  Return new node
    fn insert_after(&self, data: T, after: &Self) -> Self;

    /// Return new node
    fn insert_before(&self, data: T, before: &Self) -> Self;

    /// Return parent and data
    fn delete_current(&self) -> (Option<Self>, T);

    /// Delete all left
    fn delete_left(&self, count: usize) -> Vec<T>;

    fn delete_right(&self, count: usize) -> Vec<T>;
    fn delete_children(&self) -> Option<ArcNode<T>>;
}
