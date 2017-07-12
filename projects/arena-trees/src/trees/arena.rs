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
}

impl<T> NodeArena<T> {
    pub fn get<I>(&self, index: I) -> &I::Output
    where
        I: SliceIndex<[NodeData<T>]>,
    {
        if cfg!(debug_assertions) { self.nodes.get(index).unwrap() } else { unsafe { self.nodes.get_unchecked(index) } }
    }

    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: SliceIndex<[NodeData<T>]>,
    {
        self.nodes.get_mut(index)
    }
    pub unsafe fn get_unchecked_mut<I>(&mut self, index: I) -> &mut I::Output
    where
        I: SliceIndex<[NodeData<T>]>,
    {
        self.nodes.get_unchecked_mut(index)
    }
}
