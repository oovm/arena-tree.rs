mod children;

pub use self::children::{Ancestors, Children, Descendants, Siblings};

use crate::{
    traits::{DeleteNodes, TraversalOrder, TreeNode},
    TreeError,
};
use std::{
    cell::RefCell,
    mem::{swap, take},
    rc::Rc,
    sync::{Arc, Mutex},
};

// need debug
pub struct Node<T> {
    id: usize,
    arena: Rc<RefCell<Tree<T>>>,
}

// need debug
pub struct Tree<T> {
    nodes: Vec<NodeData<T>>,
    empty: Vec<usize>,
}

// need debug
pub struct NodeData<T> {
    parent: Option<usize>,
    left_sibling: Option<usize>,
    right_sibling: Option<usize>,
    first_child: Option<usize>,
    last_child: Option<usize>,
    data: T,
}


impl<T> Tree<T> {
    pub fn get(&self, id: usize) -> Result<&NodeData<T>, TreeError> {
        match self.nodes.get(id) {
            Some(s) => Ok(s),
            None => Err(TreeError::MissingCurrentNode),
        }
    }
    pub fn get_mut(&mut self, id: usize) -> Result<&mut NodeData<T>, TreeError> {
        match self.nodes.get_mut(id) {
            Some(s) => Ok(s),
            None => Err(TreeError::MissingCurrentNode),
        }
    }
    pub fn create_new(
        &mut self,
        data: T,
        parent: Option<usize>,
        left: Option<usize>,
        right: Option<usize>,
        first: Option<usize>,
        last: Option<usize>,
    ) -> usize {
        match self.empty.pop() {
            Some(old) => unsafe {
                let node = self.nodes.get_unchecked_mut(old);
                node.parent = parent;
                node.left_sibling = left;
                node.right_sibling = right;
                node.first_child = first;
                node.last_child = last;
                node.data = data;
                old
            },
            None => {
                self.nodes.push(NodeData {
                    parent,
                    left_sibling: left,
                    right_sibling: None,
                    first_child: None,
                    last_child: None,
                    data,
                });
                self.nodes.len()
            }
        }
    }
}



impl<T> NodeData<T> {
    pub fn parent(&self) -> Result<usize, TreeError> {
        match self.parent {
            Some(s) => Ok(s),
            None => Err(TreeError::MissingParentNode),
        }
    }
}

impl<T> TreeNode<T> for Node<T> {
    type Ancestors = Ancestors<T>;
    type Siblings = Siblings<T>;
    type Children = Children<T>;
    type Descendants = Descendants<T>;

    fn new(data: T, capacity: usize) -> Self {
        let mut nodes = Vec::with_capacity(capacity);
        nodes.push(NodeData {
            parent: None,
            left_sibling: None,
            right_sibling: None,
            first_child: None,
            last_child: None,
            data,
        });
        Self { id: 0, arena: Rc::new(RefCell::new(Tree { nodes, empty: vec![] })) }
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
        todo!()
    }

    fn left(&self) -> Option<Self> {
        todo!()
    }

    fn first_sibling(&self) -> Result<Self, TreeError> {
        todo!()
    }

    fn right(&self) -> Option<Self> {
        todo!()
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
        match self.last_child() {
            Some(s) => self.insert_after(data, &s),
            None => {
                let mut arena = self.arena.borrow_mut();
                Self { id: arena.create_new(data, Some(self.id), None, None, None, None), arena: self.arena.clone() }
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
