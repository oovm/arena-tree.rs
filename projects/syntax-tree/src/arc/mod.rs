mod children;

pub use self::children::{Ancestors, Children, Descendants, Siblings};

use crate::{
    traits::{DeleteNodes, TraversalOrder},
    TreeError,
};
use std::{
    mem::{swap, take},
    sync::{Arc, Mutex},
};
use crate::traits::QueryNode;

// need debug
#[derive(Debug)]
pub struct Node<T> {
    id: usize,
    arena: Arc<Mutex<Tree<T>>>,
}

// need debug
#[derive(Debug)]
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
    pub fn create_new(&mut self, data: T, parent: Option<usize>, left: Option<usize>) -> usize {
        match self.empty.pop() {
            Some(old) => unsafe {
                let node = self.nodes.get_unchecked_mut(old);
                node.parent = parent;
                node.left_sibling = left;
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

#[derive(Debug)]
pub struct NodeData<T> {
    parent: Option<usize>,
    left_sibling: Option<usize>,
    right_sibling: Option<usize>,
    first_child: Option<usize>,
    last_child: Option<usize>,
    data: T,
}

impl<T> NodeData<T> {
    pub fn parent(&self) -> Result<usize, TreeError> {
        match self.parent {
            Some(s) => Ok(s),
            None => Err(TreeError::MissingParentNode),
        }
    }
}

impl<T> QueryNode<T> for Node<T> {
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
        Self { id: 0, arena: Arc::new(Mutex::new(Tree { nodes, empty: vec![] })) }
    }


    fn is_root(&self) -> bool {
        self.id.eq(&0)
    }

    fn root(&self) -> Self {
        Self { id: 0, arena: self.arena.clone() }
    }

    fn ancestor(&self, with_self: bool) -> Self::Ancestors {
        todo!()
    }

    fn parent(&self) -> Option<Self> {
        let id = match self.arena.lock() {
            Ok(s) => s.nodes.get(self.id)?.parent?,
            Err(_) => None?,
        };
        Some(Self { id, arena: self.arena.clone() })
    }

    fn left_sibling(&self) -> Option<Self> {
        let id = match self.arena.lock() {
            Ok(s) => s.nodes.get(self.id)?.left_sibling?,
            Err(_) => None?,
        };
        Some(Self { id, arena: self.arena.clone() })
    }

    fn left_siblings(&self, include_self: bool) -> Self::Siblings {
        todo!()
    }

    fn first_sibling(&self) -> Self {
        todo!()
    }

    fn right_sibling(&self) -> Option<Self> {
        let id = match self.arena.lock() {
            Ok(s) => s.nodes.get(self.id)?.right_sibling?,
            Err(_) => None?,
        };
        Some(Self { id, arena: self.arena.clone() })
    }

    fn right_siblings(&self, include_self: bool) -> Self::Siblings {
        todo!()
    }

    fn last_sibling(&self) -> Self {
            todo!()
    }

    fn siblings(&self, reverse: bool) -> Self::Siblings {
        todo!()
    }


    fn children(&self, reverse: bool) -> Self::Children {
        todo!()
    }


    fn descendants(&self, reverse: bool) -> Self::Descendants {
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
