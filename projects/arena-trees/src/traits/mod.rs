use crate::TreeError;

pub enum TraversalOrder {
    DepthFirst,
    BreadthFirst,
}

pub struct DeleteNodes<N, T>
where
    N: TreeNode<T>,
{
    rest: Option<N>,
    data: Vec<T>,
}

pub trait TreeNode<T>
where
    Self: Sized,
{
    type Ancestors: Iterator<Item = Self>;
    type Siblings: Iterator<Item = Self>;
    type Children: Iterator<Item = Self>;
    type Descendants: Iterator<Item = Self>;

    /// Create a tree based on the given root node and capacity
    fn new(data: T, capacity: usize) -> Self;

    fn take(&self) -> Result<T, TreeError>
    where
        T: Default;

    fn swap(&self, data: &mut T);

    fn is_root(&self) -> bool {
        self.parent().is_none()
    }

    fn root(&self) -> Self;

    fn ancestor(&self, with_self: usize) -> Self::Ancestors;

    fn parent(&self) -> Option<Self>;

    fn is_alone(&self) -> bool {
        self.left().is_none() && self.right().is_none()
    }

    fn is_leftmost(&self) -> bool {
        self.left().is_none()
    }

    fn left(&self) -> Option<Self>;

    fn first_sibling(&self) -> Self;

    fn is_rightmost(&self) -> bool {
        self.right().is_none()
    }

    fn right(&self) -> Option<Self>;

    fn last_sibling(&self) -> Self;

    ///  Return new node
    fn insert_after(&self, data: T, after: &Self) -> Self;

    /// Return new node
    fn insert_before(&self, data: T, before: &Self) -> Self;

    /// Insert data to the left of the node, it does not need to be the first node
    ///
    /// Return new node
    fn insert_left(&self, data: T) -> Result<Self, TreeError> {
        match self.parent() {
            Some(s) => Ok(s.insert_before(data, self)),
            None => Err(TreeError::RootSiblingOperation),
        }
    }

    fn insert_right(&self, data: T) -> Result<Self, TreeError> {
        match self.parent() {
            Some(s) => Ok(s.insert_after(data, self)),
            None => Err(TreeError::RootSiblingOperation),
        }
    }
    fn has_child(&self) -> bool {
        self.first_child().is_some()
    }

    fn first_child(&self) -> Option<Self> {
        self.children(false).next()
    }

    fn last_child(&self) -> Option<Self> {
        self.children(true).next()
    }

    fn count_children(&self) -> usize {
        self.children(false).count()
    }

    fn children(&self, reverse: bool) -> Self::Children;

    ///  Return new node
    fn insert_child_left(&self, data: T) -> Self;

    ///  Return new node
    fn insert_child_right(&self, data: T) -> Self;

    fn descendants(&self, reverse: bool) -> Self::Descendants;

    /// Return parent and data
    fn delete_current(&self, order: TraversalOrder) -> DeleteNodes<Self, T>;

    /// Delete all left
    fn delete_left(&self, count: usize) -> DeleteNodes<Self, T>;

    fn delete_right(&self, count: usize) -> DeleteNodes<Self, T>;

    /// Delete all sibling or lower-level nodes
    fn delete_siblings(&self, order: TraversalOrder) -> DeleteNodes<Self, T>;

    /// Delete all low-level nodes
    fn delete_children(&self, order: TraversalOrder) -> DeleteNodes<Self, T>;
}
