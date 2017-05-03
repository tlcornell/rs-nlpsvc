use std::collections::HashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}


pub struct AnnotatedDocument {
    doc_string: String,
    metadata: Annotation,
    objects: NodeArena,
    values: ValueTables,
}

impl AnnotatedDocument {

    pub fn new(text: &str) -> AnnotatedDocument {
        AnnotatedDocument {
            doc_string: String::from(text),
            metadata: Annotation::new(),
            objects: NodeArena::new(),
            values: ValueTables::new(),
        }
    }

    pub fn node_builder(&self) -> NodeBuilder {
        NodeBuilder {}
    }

    pub fn leaf_nodes(&self) -> LeafIter {
        LeafIter {
            node_source: &self.objects,
        }
    }

}

pub struct LeafIter<'a> {
    node_source: &'a NodeArena,
}

impl<'a> Iterator for LeafIter<'a> {
    type Item = &'a Node;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}


pub struct NodeBuilder;

impl NodeBuilder {

    pub fn build(self) -> Node {
        Node {}
    }

    pub fn span(mut self, begin: usize, end: usize) -> NodeBuilder {
        self
    }

    pub fn string_val(mut self, key: &str, val: &str) -> NodeBuilder {
        self
    }

    pub fn sym_val(mut self, key: &str, val: &str) -> NodeBuilder {
        self
    }

    pub fn follows(mut self, pred: &mut Node) -> NodeBuilder {
        self
    }


}

pub struct Node;

impl Node {

    /// Return the string spanned by our begin/end offsets
    pub fn word(&self) -> String {
        // Need a reference back to the containing AnnotatedDocument.
        // That's gonna hurt...
        unimplemented!()
    }

    pub fn get_value(&self, key: &str) -> String {
        unimplemented!()
    }
}



struct Annotation {
    data: HashMap<String, u64>,
}

impl Annotation {

    fn new() -> Annotation {
        Annotation {
            data: HashMap::new(),
        }
    }

}

struct NodeArena;

impl NodeArena {

    fn new() -> NodeArena {
        NodeArena {}
    }

}

struct ValueTables;

impl ValueTables {

    fn new() -> ValueTables {
        ValueTables {}
    }

}