use super::*;

#[derive(Debug)]
pub struct Ancestors<T> {
    pub(crate) current: Option<Node<T>>,
}

impl<T> Iterator for Ancestors<T> {
    type Item = Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.current.take()?;
        self.current = out.parent();
        Some(out)
    }
}
