use crate::TreeError;

pub enum TraversalOrder {
    DepthFirst,
    BreadthFirst,
}

pub trait Node<T>
where
    Self: Sized,
{
    type Ancestors: Iterator<Item = Self>;
    type Siblings: Iterator<Item = Self>;
    type Children: Iterator<Item = Self>;
    type Descendants: Iterator<Item = Self>;

    /// Create a tree based on the given root node and capacity
    fn new(data: T, capacity: usize) -> Self;

    fn is_root(&self) -> bool {
        self.parent().is_none()
    }

    fn root(&self) -> Self;

    fn ancestor(&self, with_self: bool) -> Self::Ancestors;

    fn parent(&self) -> Option<Self>;

    fn is_alone(&self) -> bool {
        self.left_sibling().is_none() && self.right_sibling().is_none()
    }

    fn is_leftmost(&self) -> bool {
        self.left_sibling().is_none()
    }

    fn left_sibling(&self) -> Option<Self>;

    fn left_siblings(&self, include_self: bool) -> Self::Siblings;

    fn first_sibling(&self) -> Self;

    fn is_rightmost(&self) -> bool {
        self.right_sibling().is_none()
    }

    fn right_sibling(&self) -> Option<Self>;

    fn right_siblings(&self, include_self: bool) -> Self::Siblings;

    fn last_sibling(&self) -> Self;

    fn siblings(&self, reverse: bool) -> Self::Siblings;

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

    fn descendants(&self, reverse: bool) -> Self::Descendants;
}

pub trait MutateNode<T>: Node<T> {
    fn take(&self) -> Result<T, TreeError>
    where
        T: Default;

    fn swap(&self, data: &mut T);

    ///  Return new node
    fn insert_after(&self, data: T, after: &Self) -> Result<Self, TreeError>;

    /// Return new node
    fn insert_before(&self, data: T, before: &Self) -> Result<Self, TreeError>;

    /// Insert data to the left of the node, it does not need to be the first node
    ///
    /// Return new node
    fn insert_left(&self, data: T) -> Result<Self, TreeError> {
        match self.parent() {
            Some(s) => s.insert_before(data, self),
            None => Err(TreeError::RootSiblingOperation),
        }
    }

    fn insert_right(&self, data: T) -> Result<Self, TreeError> {
        match self.parent() {
            Some(s) => s.insert_after(data, self),
            None => Err(TreeError::RootSiblingOperation),
        }
    }

    ///  Return new node
    fn insert_child_left(&self, data: T) -> Self;

    ///  Return new node
    fn insert_child_right(&self, data: T) -> Self;

    /// Return parent and data
    fn delete_current(self, order: TraversalOrder);

    /// Delete all left
    fn delete_left(&self, count: usize);

    fn delete_right(&self, count: usize);

    /// Delete all sibling or lower-level nodes
    fn delete_siblings(&self, order: TraversalOrder);

    /// Delete all low-level nodes
    fn delete_children(&self, order: TraversalOrder);
}
