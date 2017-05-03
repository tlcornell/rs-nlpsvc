// Basic document test

extern crate annotated_document;
extern crate rs_regex;

use annotated_document::AnnotatedDocument;

#[test]
fn low_level() {
    let mut doc = AnnotatedDocument::new("Hello world");
    let mut node1 = doc.node_builder()
        .string_val("lemma", "hello")
        .span(0, 5)
        .build();
    let mut node2 = doc.node_builder()
        .span(7, 11)
        .string_val("lemma", "world")
        .follows(&mut node1)
        .build();
}

#[test]
fn tokenization_test() {
    let mut doc = AnnotatedDocument::new("Hello world");
    let mut node1 = doc.node_builder()
        .span(0, 5)
        .sym_val("toktype", "WORD")
        .build();
    let mut _node2 = doc.node_builder()
        .span(7, 11)
        .sym_val("toktype", "WORD")
        .follows(&mut node1)
        .build();

    for t in doc.leaf_nodes() {
        println!("{}: {}", t.word(), t.get_value("lemma"));
    }
}




