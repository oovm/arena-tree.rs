use crate::{
    traits::{DeleteNodes, TraversalOrder, TreeNode},
    TreeError,
};
use std::{
    mem::{swap, take},
    sync::{Arc, Mutex},
};
use std::sync::LockResult;

// need debug
pub struct Node<T> {
    id: usize,
    arena: Arc<Mutex<Tree<T>>>,
}

// need debug
pub struct Tree<T> {
    nodes: Vec<NodeData<T>>,
    empty: Vec<usize>,
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
}

// need debug
pub struct NodeData<T> {
    parent: Option<usize>,
    first_sibling: usize,
    left_sibling: Option<usize>,
    right_sibling: Option<usize>,
    last_sibling: usize,
    first_children: Option<usize>,
    last_children: Option<usize>,
    data: T,
}

pub struct Ancestors<T> {
    remember: Vec<Node<T>>,
    current: usize,
}

impl<T> Iterator for Ancestors<T> {
    type Item = Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<T> TreeNode<T> for Node<T> {
    type Ancestors = Ancestors<T>;
    type Siblings = Ancestors<T>;
    type Children = Ancestors<T>;
    type Descendants = Ancestors<T>;

    fn new(data: T, capacity: usize) -> Self {
        let mut nodes = Vec::with_capacity(capacity);
        nodes.push(NodeData {
            parent: None,
            first_sibling: 0,
            left_sibling: None,
            right_sibling: None,
            last_sibling: 0,
            first_children: None,
            last_children: None,
            data,
        });
        Self { id: 0, arena: Arc::new(Mutex::new(Tree { nodes, empty: vec![] })) }
    }

    fn take(&self) -> Result<T, TreeError>
    where
        T: Default,
    {
        let mut lock = self.arena.lock()?;
        let raw = lock.nodes.get_mut(self.id).unwrap();
        Ok(take(&mut raw.data))
    }
    fn swap(&self, data: &mut T) {
        let mut lock = self.arena.lock().unwrap();
        let raw = lock.nodes.get_mut(self.id).unwrap();
        swap(&mut raw.data, data)
    }

    fn is_root(&self) -> bool {
        self.id.eq(&0)
    }

    fn root(&self) -> Self {
        todo!()
    }

    fn ancestor(&self, with_self: usize) -> Self::Ancestors {
        todo!()
    }

    fn parent(&self) -> Result<Self, TreeError> {
        let mut lock = match self.arena.lock() {
            Ok(s) => {}
            Err(_) => {}
        };
        let raw = lock.get(self.id)?;
        match raw.parent {
            Some(s) => Ok(Self { id: s, arena: self.arena.clone() }),
            None => Err(TreeError::MissingParentNode),
        }
    }

    fn left(&self) -> Result<Self, TreeError> {
        let mut lock = self.arena.lock()?;
        let raw = lock.get(self.id)?;
        match raw.left_sibling {
            Some(s) => Ok(Self { id: s, arena: self.arena.clone() }),
            None => Err(TreeError::MissingParentNode),
        }
    }

    fn first_sibling(&self) -> Result<Self, TreeError> {
        let mut lock = self.arena.lock()?;
        let raw = lock.get(self.id)?;
        Ok(Self { id: raw.first_sibling, arena: self.arena.clone() })
    }

    fn right(&self) -> Result<Self, TreeError> {
        let mut lock = self.arena.lock()?;
        let raw = lock.get(self.id)?;
        match raw.right_sibling {
            Some(s) => Ok(Self { id: s, arena: self.arena.clone() }),
            None => Err(TreeError::MissingParentNode),
        }
    }

    fn last_sibling(&self) -> Result<Self, TreeError> {
        let mut lock = self.arena.lock()?;
        let raw = lock.get(self.id)?;
        Ok(Self { id: raw.last_sibling, arena: self.arena.clone() })
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
        todo!()
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
