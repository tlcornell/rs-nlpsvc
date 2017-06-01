extern crate indextree;

use std::collections::HashMap;


pub mod tree_sequence;
mod node_label;

pub use node_label::NodeLabel;

use tree_sequence::TreeSequence;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}


pub struct AnnotatedDocument {
    doc_string: String,
    tree_sequence: TreeSequence,
}

impl AnnotatedDocument {

    pub fn new(text: &str) -> AnnotatedDocument {
        AnnotatedDocument {
            doc_string: String::from(text),
            tree_sequence: TreeSequence::new(),
       }
    }
    pub fn get_text(&self) -> &str {
        &self.doc_string
    }
    pub fn get_trees_mut(&mut self) -> &mut TreeSequence {
        &mut self.tree_sequence
    }
    pub fn get_trees(&self) -> &TreeSequence {
        &self.tree_sequence
    }
}

