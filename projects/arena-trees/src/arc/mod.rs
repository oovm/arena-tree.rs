use crate::traits::{DeleteNodes, TraversalOrder, TreeNode};
use std::sync::{Arc, Mutex};

// need debug
pub struct Node<T> {
    id: usize,
    arena: Arc<Mutex<ArenaTree<T>>>,
}

// need debug
pub struct ArenaTree<T> {
    nodes: Vec<ArenaData<T>>,
    empty: Vec<usize>,
}

// need debug
pub struct ArenaData<T> {
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
        nodes.push(ArenaData {
            parent: None,
            first_sibling: 0,
            left_sibling: None,
            right_sibling: None,
            last_sibling: 0,
            first_children: None,
            last_children: None,
            data,
        });
        Self { id: 0, arena: Arc::new(Mutex::new(ArenaTree { nodes, empty: vec![] })) }
    }

    fn is_root(&self) -> bool {
        self.id.eq(&0)
    }
    fn take(&self)
    where
        T: Default,
    {
        todo!()
    }

    fn swap(&self, data: T) {
        todo!()
    }

    fn root(&self) -> Self {
        todo!()
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

    fn first_sibling(&self) -> Self {
        todo!()
    }

    fn right(&self) -> Option<Self> {
        todo!()
    }

    fn last_sibling(&self) -> Self {
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
    let root = Node::new("1");
    let child_1 = root.insert_right_child("1_1");
    // ....
}
