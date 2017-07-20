mod ancestors;
mod children;
mod descendants;
mod siblings;

pub use self::ancestors::SyntaxAncestors;

use crate::{
    traits::{MutateNode, Node},
    trees::{TextArena, TextNode},
};
use std::{cell::RefCell, rc::Rc};

#[derive(Clone, Debug)]
pub struct SyntaxNode {
    id: usize,
    arena: Rc<RefCell<TextArena>>,
}

impl Node<TextNode> for SyntaxNode {
    type Ancestors = SyntaxAncestors;
    type Siblings = SyntaxAncestors;
    type Children = SyntaxAncestors;
    type Descendants = SyntaxAncestors;

    fn new(data: TextNode, capacity: usize) -> Self {
        todo!()
    }

    fn is_root(&self) -> bool {
        self.id.eq(&0)
    }

    fn root(&self) -> Self {
        todo!()
    }

    fn ancestor(&self, with_self: bool) -> Self::Ancestors {
        todo!()
    }

    fn parent(&self) -> Option<Self> {
        todo!()
    }

    fn left_sibling(&self) -> Option<Self> {
        todo!()
    }

    fn left_siblings(&self, include_self: bool) -> Self::Siblings {
        todo!()
    }

    fn first_sibling(&self) -> Self {
        todo!()
    }

    fn right_sibling(&self) -> Option<Self> {
        todo!()
    }

    fn right_siblings(&self, include_self: bool) -> Self::Siblings {
        todo!()
    }

    fn last_sibling(&self) -> Self {
        todo!()
    }

    fn siblings(&self, reverse: bool) -> Self::Siblings {
        todo!()
    }

    fn children(&self, reverse: bool) -> Self::Children {
        todo!()
    }

    fn descendants(&self, reverse: bool) -> Self::Descendants {
        todo!()
    }
}
