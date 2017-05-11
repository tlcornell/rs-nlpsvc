use std::collections::HashMap;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}


pub struct AnnotatedDocument {
    doc_string: String,
    annotations: AnnotationSet,
}

impl AnnotatedDocument {

    pub fn new(text: &str) -> AnnotatedDocument {
        AnnotatedDocument {
            doc_string: String::from(text),
            annotations: AnnotationSet::new(),
       }
    }
    pub fn get_text(&self) -> &str {
        &self.doc_string
    }

    pub fn get_objects(&mut self) -> &mut AnnotationSet {
        &mut self.annotations
    }
}


pub struct AnnotationSet {
    objects: NodeArena,
}

impl AnnotationSet {

    pub fn new() -> AnnotationSet {
        AnnotationSet { objects: NodeArena::new(), }
    }

    pub fn node_builder(&self) -> NodeBuilder {
        NodeBuilder::new()
    }

    pub fn append(&mut self, node: Node) {
        unimplemented!();
    }

    pub fn leaf_nodes(&self) -> LeafIter {
        LeafIter::new(self)
    }

    pub fn get_first_leaf(&self) -> Option<&Node> {
        unimplemented!();
    }
}

pub struct LeafIter<'a> {
    node_source: &'a NodeArena,
    next_item: Option<&'a Node>,
}

impl<'a> LeafIter<'a> {
    pub fn new(doc: &'a AnnotationSet) -> LeafIter<'a> {
        LeafIter {
            node_source: &doc.objects,
            next_item: doc.get_first_leaf(),
        }
    }
}

impl<'a> Iterator for LeafIter<'a> {
    type Item = &'a Node;
    fn next(&mut self) -> Option<Self::Item> {
        let retval = self.next_item;
        self.next_item = match self.next_item {
            None => None,
            Some(nd) => nd.get_next_leaf()
        };
        retval
    }
}


pub struct NodeBuilder;

impl NodeBuilder {

    pub fn new() -> NodeBuilder {
        NodeBuilder {}
    }

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

    pub fn get_next_leaf(&self) -> Option<&Node> {
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