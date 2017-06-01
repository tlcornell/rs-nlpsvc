// Basic document test

extern crate annotated_document;

use annotated_document::AnnotatedDocument;
use annotated_document::NodeLabel;

#[test]
fn push_tokens_and_traverse() {
    // Fake tokenizer
    let mut doc = AnnotatedDocument::new("01 Hello!");
    let mut lbl0 = NodeLabel::new();
    lbl0.set_span(0, 2)
        .set_sym_val("toktype", "NUMBER");
    doc.get_trees_mut().push_back(lbl0);
    let mut lbl1 = NodeLabel::new();
    lbl1.set_span(3, 8)
        .set_sym_val("toktype", "WORD");
    doc.get_trees_mut().push_back(lbl1);
    let mut lbl2 = NodeLabel::new();
    lbl2.set_span(8, 9)
        .set_sym_val("toktype", "PUNCT");
    doc.get_trees_mut().push_back(lbl2);

    // Traverse (and print)
    let mut cursor = doc.get_trees().first();
    while cursor.is_valid() {
        {
            let label = cursor.get().unwrap();
            let span = label.get_span().unwrap();
            println!("({}, {}) {}", span.0, span.1,
                              &doc.get_text()[span.0..span.1]);
        }
        cursor.next();
    }
}
