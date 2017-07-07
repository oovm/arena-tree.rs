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


// 调用示例

fn main() {
    let arena = Arc::new(Mutex::new(ArenaTree::new()));
    let root = ArcNode::new(arena.clone(), 0);

    let n1 = root.append_child(1);
    let n2 = root.append_child(2);
    let n3 = n1.append_child(3);

    root.delete_current();
}