mod ancestors;
mod children;
mod descendants;
mod siblings;

pub use self::{
    ancestors::Ancestors,
    children::{Children, Descendants},
    siblings::Siblings,
};

use crate::{
    traits::{DeleteNodes, MutateNode, QueryNode, TraversalOrder},
    trees::{NodeArena, NodeData, NodeLink},
    TreeError,
};
use std::{
    cell::RefCell,
    mem::{swap, take},
    rc::Rc,
};

#[derive(Debug)]
pub struct Node<T> {
    id: usize,
    arena: Rc<RefCell<NodeArena<T>>>,
}

impl<T> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self { id: self.id, arena: self.arena.clone() }
    }
}

impl<T> QueryNode<T> for Node<T> {
    type Ancestors = Ancestors<T>;
    type Siblings = Siblings<T>;
    type Children = Siblings<T>;
    type Descendants = Descendants<T>;

    fn new(data: T, capacity: usize) -> Self {
        let mut nodes = Vec::with_capacity(capacity);
        nodes.push(NodeData { link: NodeLink::default(), data });
        Self { id: 0, arena: Rc::new(RefCell::new(NodeArena { nodes, empty: vec![] })) }
    }

    fn is_root(&self) -> bool {
        self.id.eq(&0)
    }

    fn root(&self) -> Self {
        Self { id: 0, arena: self.arena.clone() }
    }

    fn ancestor(&self, with_self: bool) -> Self::Ancestors {
        Ancestors { current: if with_self { Some(self.clone()) } else { self.parent() } }
    }

    fn parent(&self) -> Option<Self> {
        if self.is_root() { None } else { Some(Self { id: self.unchecked_parent(), arena: self.arena.clone() }) }
    }

    fn left_sibling(&self) -> Option<Self> {
        let tree = self.arena.borrow_mut();
        let left = tree.get(self.id).link.left_sibling?;
        Some(Self { id: left, arena: self.arena.clone() })
    }

    fn left_siblings(&self, include_self: bool) -> Self::Siblings {
        if include_self {
            Siblings { current: Some(self.clone()), reversed: true }
        }
        else {
            Siblings { current: self.left_sibling(), reversed: true }
        }
    }

    fn first_sibling(&self) -> Self {
        if self.is_root() {
            return Self { id: 0, arena: self.arena.clone() };
        }
        else {
            let tree = self.arena.borrow_mut();
            unsafe {
                let parent = tree.get(self.id).link.parent.unwrap_unchecked();
                let left = tree.get(parent).link.first_child.unwrap_unchecked();
                Self { id: left, arena: self.arena.clone() }
            }
        }
    }

    fn right_sibling(&self) -> Option<Self> {
        let tree = self.arena.borrow_mut();
        let right = tree.get(self.id).link.right_sibling?;
        Some(Self { id: right, arena: self.arena.clone() })
    }

    fn right_siblings(&self, include_self: bool) -> Self::Siblings {
        if include_self {
            Siblings { current: Some(self.clone()), reversed: false }
        }
        else {
            Siblings { current: self.right_sibling(), reversed: false }
        }
    }

    fn last_sibling(&self) -> Self {
        if self.is_root() {
            return Self { id: 0, arena: self.arena.clone() };
        }
        else {
            let tree = self.arena.borrow_mut();
            unsafe {
                let parent = tree.get(self.id).link.parent.unwrap_unchecked();
                let right = tree.get(parent).link.last_child.unwrap_unchecked();
                Self { id: right, arena: self.arena.clone() }
            }
        }
    }

    fn siblings(&self, reverse: bool) -> Self::Siblings {
        if reverse {
            Siblings { current: Some(self.last_sibling()), reversed: true }
        }
        else {
            Siblings { current: Some(self.first_sibling()), reversed: false }
        }
    }

    fn children(&self, reverse: bool) -> Self::Children {
        if reverse {
            Siblings { current: self.last_child(), reversed: true }
        }
        else {
            Siblings { current: self.first_child(), reversed: false }
        }
    }

    fn descendants(&self, reverse: bool) -> Self::Descendants {
        todo!()
    }
}

impl<T> MutateNode<T> for Node<T> {
    fn take(&self) -> Result<T, TreeError>
    where
        T: Default,
    {
        let mut lock = self.arena.borrow_mut();
        let raw = lock.nodes.get_mut(self.id).unwrap();
        Ok(take(&mut raw.data))
    }
    fn swap(&self, data: &mut T) {
        let mut lock = self.arena.borrow_mut();
        let raw = lock.nodes.get_mut(self.id).unwrap();
        swap(&mut raw.data, data)
    }

    fn insert_after(&self, data: T, after: &Self) -> Result<Self, TreeError> {
        let mut tree = self.arena.borrow_mut();
        let parent_id = match tree.get(self.id).link.parent {
            Some(s) => s,
            None => Err(TreeError::RootSiblingOperation)?,
        };
        let left_id = after.id;
        let left_link = tree.get(left_id).link;
        let new_id = match left_link.right_sibling {
            Some(right_id) => {
                let new = tree.create(
                    data,
                    NodeLink {
                        parent: Some(parent_id),
                        left_sibling: Some(left_id),
                        right_sibling: Some(right_id),
                        first_child: None,
                        last_child: None,
                    },
                );
                tree.get_mut(left_id).link.right_sibling = Some(new);
                tree.get_mut(right_id).link.left_sibling = Some(new);
                new
            }
            None => {
                let new = tree.create(
                    data,
                    NodeLink {
                        parent: Some(parent_id),
                        left_sibling: Some(left_id),
                        right_sibling: None,
                        first_child: None,
                        last_child: None,
                    },
                );
                tree.get_mut(parent_id).link.last_child = Some(new);
                tree.get_mut(left_id).link.right_sibling = Some(new);
                new
            }
        };
        Ok(Self { id: new_id, arena: self.arena.clone() })
    }

    fn insert_before(&self, data: T, before: &Self) -> Result<Self, TreeError> {
        let mut tree = self.arena.borrow_mut();
        let parent_id = match tree.get(self.id).link.parent {
            Some(s) => s,
            None => Err(TreeError::RootSiblingOperation)?,
        };
        let right_id = before.id;
        let right_link = tree.get(right_id).link;
        let new_id = match right_link.left_sibling {
            Some(left_id) => {
                let new = tree.create(
                    data,
                    NodeLink {
                        parent: Some(parent_id),
                        left_sibling: Some(left_id),
                        right_sibling: Some(right_id),
                        first_child: None,
                        last_child: None,
                    },
                );
                tree.get_mut(right_id).link.left_sibling = Some(new);
                tree.get_mut(left_id).link.right_sibling = Some(new);
                new
            }
            None => {
                let new = tree.create(
                    data,
                    NodeLink {
                        parent: Some(parent_id),
                        left_sibling: None,
                        right_sibling: Some(right_id),
                        first_child: None,
                        last_child: None,
                    },
                );
                tree.get_mut(parent_id).link.first_child = Some(new);
                tree.get_mut(right_id).link.right_sibling = Some(new);
                new
            }
        };
        Ok(Self { id: new_id, arena: self.arena.clone() })
    }
    fn insert_child_left(&self, data: T) -> Self {
        let mut tree = self.arena.borrow_mut();
        let parent_id = self.id;
        let mut parent_link = tree.get(parent_id).link;
        let new_id = match parent_link.first_child {
            Some(right_id) => {
                let new = tree.create(
                    data,
                    NodeLink {
                        parent: Some(parent_id),
                        left_sibling: None,
                        right_sibling: Some(right_id),
                        first_child: None,
                        last_child: None,
                    },
                );
                tree.get_mut(parent_id).link.first_child = Some(new);
                tree.get_mut(right_id).link.right_sibling = Some(new);
                new
            }
            None => tree.create_child(data, parent_id),
        };
        Self { id: new_id, arena: self.arena.clone() }
    }

    fn insert_child_right(&self, data: T) -> Self {
        let mut tree = self.arena.borrow_mut();
        let parent_id = self.id;
        let mut parent_link = tree.get(parent_id).link;
        let new_id = match parent_link.last_child {
            Some(left_id) => {
                let new = tree.create(
                    data,
                    NodeLink {
                        parent: Some(parent_id),
                        left_sibling: Some(left_id),
                        right_sibling: None,
                        first_child: None,
                        last_child: None,
                    },
                );
                tree.get_mut(parent_id).link.last_child = Some(new);
                tree.get_mut(left_id).link.right_sibling = Some(new);
                new
            }
            None => tree.create_child(data, parent_id),
        };
        Self { id: new_id, arena: self.arena.clone() }
    }

    fn delete_current(self, order: TraversalOrder) -> DeleteNodes<Self, T> {
        todo!()
    }

    fn delete_left(&self, count: usize) -> DeleteNodes<Self, T> {
        todo!()
    }

    fn delete_right(&self, count: usize) -> DeleteNodes<Self, T> {
        todo!()
    }

    fn delete_siblings(&self, order: TraversalOrder) -> DeleteNodes<Self, T> {
        todo!()
    }

    fn delete_children(&self, order: TraversalOrder) -> DeleteNodes<Self, T> {
        todo!()
    }
}

impl<T> Node<T> {
    pub(crate) fn unchecked_parent(&self) -> usize {
        debug_assert_ne!(self.id, 0, "This is a root node!");
        let tree = self.arena.borrow_mut();
        let data = tree.get(self.id);
        unsafe { data.link.parent.unwrap_unchecked() }
    }
}

// test
#[test]
fn test_insert() {
    let root = Node::new("1", 0);
    let child_1 = root.insert_child_right("1_1");
    let child_2 = root.insert_child_right("1_2");
    // ....
}
