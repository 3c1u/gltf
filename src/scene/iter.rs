use json;
use std::slice;

use {Document, Node};

/// An `Iterator` that visits the nodes in a scene.
#[derive(Clone, Debug)]
pub struct Nodes<'a> {
    /// The parent `Document` struct.
    pub(crate) document: &'a Document,

    /// The internal node index iterator.
    pub(crate) iter: slice::Iter<'a, json::Index<json::scene::Node>>,
}

/// An `Iterator` that visits the children of a node.
#[derive(Clone, Debug)]
pub struct Children<'a> {
    /// The parent `Document` struct.
    pub(crate) document: &'a Document,

    /// The internal node index iterator.
    pub(crate) iter: slice::Iter<'a, json::Index<json::scene::Node>>,
}

impl<'a> ExactSizeIterator for Nodes<'a> {}
impl<'a> Iterator for Nodes<'a> {
    type Item = Node<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|index| self.document.nodes().nth(index.value()).unwrap())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Children<'a> {}
impl<'a> Iterator for Children<'a> {
    type Item = Node<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|index| self.document.nodes().nth(index.value()).unwrap())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}
