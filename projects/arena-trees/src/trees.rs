use std::slice::SliceIndex;

// need debug
pub struct NodeArena<T> {
    nodes: Vec<NodeData<T>>,
    empty: Vec<usize>,
}

// need debug
pub struct NodeData<T> {
    link: NodeLink,
    data: T,
}

pub struct NodeLink {
    parent: Option<usize>,
    left_sibling: Option<usize>,
    right_sibling: Option<usize>,
    first_child: Option<usize>,
    last_child: Option<usize>,
}

impl<T> NodeArena<T> {
    pub fn get<I>(&self, index: I) -> Option<&I::Output>
    where
        I: SliceIndex<[NodeData<T>]>,
    {
        self.nodes.get(index)
    }
    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: SliceIndex<[NodeData<T>]>,
    {
        self.nodes.get_mut(index)
    }
}
