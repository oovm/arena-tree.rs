use super::*;

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

pub struct Children<T> {
    remember: Vec<Node<T>>,
    current: usize,
}
impl<T> Iterator for Children<T> {
    type Item = Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
pub struct Descendants<T> {
    remember: Vec<Node<T>>,
    current: usize,
}

impl<T> Iterator for Descendants<T> {
    type Item = Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
