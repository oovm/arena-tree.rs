use std::sync::{Arc, Mutex};

pub struct ArcNode<T> {
    id: usize,
    arena: Arc<Mutex<ArenaTree<T>>>,

}

pub struct ArenaTree<T> {
    root: usize,
    nodes: Vec<ArenaData<T>>,
    empty: Vec<usize>,
}

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

pub trait Node<T> {
    /// Create a tree based on the given root node and capacity
    fn new(data: T, capacity: usize) -> Self;

    /// Insert data to the left of the node, it does not need to be the first node
    ///
    /// Return new node
    fn insert_left(&self, data: T) -> Self;

    fn insert_left_items<I>(&self, data: I) where I: IntoIterator<Item=T>;


    /// Add new node to left, return new sibling node
    fn prepend_one(&self, data: T) -> Self;

    fn prepend<I>(&self, data: I) where I: IntoIterator<Item=T>;
    /// Add Return new node
    fn append_child(&self, data: T) -> Self;

    /// Return new node
    fn prepend_child(&self, child_id: usize) {}

    /// Return parent
    fn delete_current(&self) -> Option<ArcNode<T>>;
    fn delete_left(&self) -> Vec<T>;
    fn delete_right(&self) -> Vec<T>;
    fn delete_children(&self) -> Option<ArcNode<T>>;
}


// 调用示例

fn main() {
    let arena = Arc::new(Mutex::new(ArenaTree::new()));
    let root = ArcNode::new(arena.clone(), 0);

    let n1 = root.append_child(1);
    let n2 = root.append_child(2);
    let n3 = n1.append_child(3);

    root.delete_current();
}