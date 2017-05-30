//! # node_label.rs
//!
//! Representation for data stored in tree nodes (and maybe elsewhere)

use std::fmt;

/// For node data
///
/// This is the data that the tree container generic will be bound to.
///

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NodeLabel {
    span: Option<(usize, usize)>,
    attributes: HashMap<String, String>,
}

impl NodeLabel {
    pub fn new() -> NodeLabel {
        NodeLabel { span: None, attributes: HashMap::new(), }
    }

    pub fn set_span(&mut self, begin: usize, end: usize) {
        // TODO: Check for end < begin, etc.
        self.span = Some((begin, end));
    }

    pub fn set_sym_val(&mut self, attr: &str, val: &str) {
        self.attributes.insert(attr.to_string(), val.to_string());
    }

    pub fn get_sym_val(&self, attr: &str) -> &str {
        &self.attributes[attr]
    }
}

impl fmt::Display for NodeLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} ", self.span.unwrap().0, self.span.unwrap().1)
    }
}
