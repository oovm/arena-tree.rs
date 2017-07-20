use super::*;

#[derive(Debug)]
pub struct SyntaxAncestors {
    pub(crate) current: Option<SyntaxNode>,
}

impl Iterator for SyntaxAncestors {
    type Item = SyntaxNode;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.current.take()?;
        self.current = out.parent();
        Some(out)
    }
}
