mod children;

pub use self::children::{Ancestors, Children, Descendants, Siblings};

use crate::{
    traits::{DeleteNodes, TraversalOrder, TreeNode},
    trees::{NodeArena, NodeData, NodeLink},
    TreeError,
};
use std::{
    cell::RefCell,
    mem::{swap, take},
    rc::Rc,
    sync::{Arc, Mutex},
};

#[derive(Debug)]
pub struct Node<T> {
    id: usize,
    arena: Rc<RefCell<NodeArena<T>>>,
}

impl<T> TreeNode<T> for Node<T> {
    type Ancestors = Ancestors<T>;
    type Siblings = Siblings<T>;
    type Children = Children<T>;
    type Descendants = Descendants<T>;

    fn new(data: T, capacity: usize) -> Self {
        let mut nodes = Vec::with_capacity(capacity);
        nodes.push(NodeData { link: NodeLink::default(), data });
        Self { id: 0, arena: Rc::new(RefCell::new(NodeArena { nodes, empty: vec![] })) }
    }

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

    fn is_root(&self) -> bool {
        self.id.eq(&0)
    }

    fn root(&self) -> Self {
        Self { id: 0, arena: self.arena.clone() }
    }

    fn ancestor(&self, with_self: usize) -> Self::Ancestors {
        todo!()
    }

    fn parent(&self) -> Option<Self> {
        let tree = self.arena.borrow_mut();
        let parent = tree.get(self.id).link.parent?;
        Some(Self { id: parent, arena: self.arena.clone()})
    }

    fn left(&self) -> Option<Self> {
        let tree = self.arena.borrow_mut();
        let left = tree.get(self.id).link.left_sibling?;
        Some(Self { id: left, arena: self.arena.clone()})
    }

    fn first_sibling(&self) -> Result<Self, TreeError> {
        todo!()
    }

    fn right(&self) -> Option<Self> {
        let tree = self.arena.borrow_mut();
        let right = tree.get(self.id).link.right_sibling?;
        Some(Self { id: right, arena: self.arena.clone()})
    }

    fn last_sibling(&self) -> Result<Self, TreeError> {
        todo!()
    }

    fn insert_after(&self, data: T, after: &Self) -> Self {
        todo!()
    }

    fn insert_before(&self, data: T, before: &Self) -> Self {
        todo!()
    }

    fn children(&self, reverse: bool) -> Self::Children {
        todo!()
    }

    fn insert_child_left(&self, data: T) -> Self {
        todo!()
    }

    fn insert_child_right(&self, data: T) -> Self {
        self.parent()

        match self.last_child() {
            Some(s) => self.insert_after(data, &s),
            None => {
                let mut arena = self.arena.borrow_mut();
                Self {
                    id: arena.create(
                        data,
                        NodeLink { parent: None, left_sibling: None, right_sibling: None, first_child: None, last_child: None },
                    ),
                    arena: self.arena.clone(),
                }
            }
        }
    }

    fn descendants(&self, reverse: bool) -> Self::Descendants {
        todo!()
    }

    fn delete_current(&self, order: TraversalOrder) -> DeleteNodes<Self, T> {
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

// test
#[test]
fn test_insert() {
    let root = Node::new("1", 0);
    let child_1 = root.insert_child_right("1_1");
    let child_2 = root.insert_child_right("1_2");
    // ....
}
