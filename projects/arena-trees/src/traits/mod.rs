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
        self.parent().is_err()
    }

    fn root(&self) -> Self;

    fn ancestor(&self, with_self: usize) -> Self::Ancestors;

    fn parent(&self) -> Option<Self>;

    fn is_alone(&self) -> bool {
        self.left().is_err() && self.right().is_err()
    }

    fn is_leftmost(&self) -> bool {
        self.left().is_err()
    }

    fn left(&self) -> Result<Self, TreeError>;

    fn first_sibling(&self) -> Result<Self, TreeError>;

    fn is_rightmost(&self) -> bool {
        self.right().is_err()
    }

    fn right(&self) -> Result<Self, TreeError>;

    fn last_sibling(&self) -> Result<Self, TreeError>;

    ///  Return new node
    fn insert_after(&self, data: T, after: &Self) -> Self;

    /// Return new node
    fn insert_before(&self, data: T, before: &Self) -> Self;

    /// Insert data to the left of the node, it does not need to be the first node
    ///
    /// Return new node
    fn insert_left(&self, data: T) -> Result<Self, TreeError> {
        match self.parent() {
            Ok(s) => Ok(s.insert_before(data, self)),
            Err(_) => Err(TreeError::RootSiblingOperation),
        }
    }

    fn insert_right(&self, data: T) -> Result<Self, TreeError> {
        match self.parent() {
            Ok(s) => Ok(s.insert_after(data, self)),
            Err(_) => Err(TreeError::RootSiblingOperation),
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
