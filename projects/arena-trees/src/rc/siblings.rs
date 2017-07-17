use super::*;

#[derive(Debug)]
pub struct Siblings<T> {
    pub(crate) current: Option<Node<T>>,
    pub(crate) reversed: bool,
}

impl<T> Iterator for Siblings<T> {
    type Item = Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.current.take()?;
        self.current = if self.reversed { out.left_sibling() } else { out.right_sibling() };
        Some(out)
    }
}
