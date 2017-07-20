use std::sync::{MutexGuard, PoisonError};

pub type Result<T> = std::result::Result<T, TreeError>;


#[derive(Copy, Clone, Debug)]
pub enum TreeError {
    /// Try to append sibling nodes to the root node
    RootSiblingOperation,

    MissingParentNode,

    MissingCurrentNode,

    MissingSibling,

    MissingChild,

    PoisonError,
}


impl<'i, A> From<PoisonError<MutexGuard<'i, A>>> for TreeError {
    fn from(error: PoisonError<MutexGuard<'i, A>>) -> Self {
        TreeError::PoisonError
    }
}