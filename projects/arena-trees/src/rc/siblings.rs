use super::*;

#[derive(Debug)]
pub struct Siblings<T> {
    remember: Vec<Node<T>>,
    current: usize,
}
impl<T> Iterator for Siblings<T> {
    type Item = Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}