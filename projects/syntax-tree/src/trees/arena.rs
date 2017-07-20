use super::*;

impl<T> NodeArena<T> {
    pub fn create(&mut self, data: T, link: NodeLink) -> NodeID {
        match self.empty.pop() {
            Some(old) => unsafe {
                let node = self.nodes.get_unchecked_mut(old);
                node.link = link;
                node.data = data;
                old
            },
            None => {
                self.nodes.push(NodeData { link, data });
                self.nodes.len()
            }
        }
    }
    pub fn create_child(&mut self, data: T, parent: NodeID) -> NodeID {
        let new = self.create(
            data,
            NodeLink { parent: Some(parent), left_sibling: None, right_sibling: None, first_child: None, last_child: None },
        );
        debug_assert!(self.is_leaf(parent), "This node already has child nodes!");
        self.get_mut(parent).link.first_child = Some(new);
        self.get_mut(parent).link.last_child = Some(new);
        new
    }
}

impl<T> NodeArena<T> {
    pub fn is_leaf(&self, node: NodeID) -> bool {
        let link = self.get(node).link;
        link.first_child.is_none() && link.last_child.is_none()
    }
    pub fn contains(&self, node: NodeID) -> bool {
        if self.empty.contains(&node) {
            return false;
        }
        self.nodes.get(node).is_some()
    }
    pub fn get(&self, index: usize) -> &NodeData<T> {
        if cfg!(debug_assertions) {
            if self.empty.contains(&index) {
                panic!("this node had been delete!")
            }
            match self.nodes.get(index) {
                Some(s) => s,
                None => panic!("this node does not exists!"),
            }
        }
        else {
            unsafe { self.nodes.get_unchecked(index) }
        }
    }

    pub fn get_mut<I>(&mut self, index: I) -> &mut I::Output
    where
        I: SliceIndex<[NodeData<T>]>,
    {
        if cfg!(debug_assertions) { self.nodes.get_mut(index).unwrap() } else { unsafe { self.nodes.get_unchecked_mut(index) } }
    }
}
