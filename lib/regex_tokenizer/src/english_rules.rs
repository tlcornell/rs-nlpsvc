/// english_rules.rs
///
/// `EnglishTokenizer` wraps a ThompsonInterpreter around a set of regex 
/// patterns for ordinary English token types. It also implements the 
/// `RegexTokenizer` trait, which in turn requires it to implement 
/// the `TokenReactor` and `TokenRecognizer` traits.

use nlpsvc_regex::reinterp::ThompsonInterpreter;
use nlpsvc_regex::reinterp::TokenRecognizer;
use nlpsvc_regex::reinterp::MatchRecord;
use regex_tokenizer::TokenReactor;
use annotated_document::AnnotatedDocument;
use annotated_document::AnnotationSet;
use regex_tokenizer::ThompsonProgramBuilder;
use regex_tokenizer::RegexTokenizer;


pub struct EnglishTokenizer {
    matcher: ThompsonInterpreter,
}

impl EnglishTokenizer {

    pub fn new() -> EnglishTokenizer {
        let mut english_patterns = ThompsonProgramBuilder::new()
           .add_rule(r"(?i)[a-z]+")           // [0] words
           .add_rule(r"[0-9,.]*[0-9]+")       // [1] numbers
           .add_rule(r"[.,?!]")               // [2] punctuation
           .build();
        EnglishTokenizer {
            matcher: ThompsonInterpreter::new(english_patterns),
        }
    }

    fn word_action(&mut self, begin: usize, end: usize, doc: &mut AnnotatedDocument) {
        println!("WORD [{}] at {}", &doc.get_text()[begin..end], begin);
        let token = doc.get_objects().node_builder()
                        .span(begin, end)
                        .sym_val("toktype", "WORD")
                        .build();
        doc.get_objects().append(token);
    }

    fn num_action(&mut self, begin:usize, end: usize, doc: &mut AnnotatedDocument) {
        println!("NUMBER [{}] at {}", &doc.get_text()[begin..end], begin);
    }

    fn punct_action(&mut self, begin: usize, end: usize, doc: &mut AnnotatedDocument) {
        println!("PUNCT [{}] at {}", &doc.get_text()[begin..end], begin);
    }
}

impl TokenRecognizer for EnglishTokenizer {
    fn next_token(&mut self, text: &str, pos: usize) -> Option<MatchRecord> {
        self.matcher.next_token(text, pos)
    }
}


impl TokenReactor for EnglishTokenizer {
    /// Append a token
    ///
    /// Append a token starting at `begin` with text `text`, that 
    /// matched rule #`rule_id`.
    fn append(&mut self, begin: usize, end:usize, rule_id: usize, doc: &mut AnnotatedDocument) {
        match rule_id {
            0 => { self.word_action(begin, end, doc); }
            1 => { self.num_action(begin, end, doc); }
            2 => { self.punct_action(begin, end, doc); }
            _ => { panic!("Unrecognized rule ID {} at pos {}", rule_id, begin); }
        }
    }

    /// Skip an unhandled character
    ///
    /// The character at `begin` is not the first character of any pattern
    /// that this tokenizer knows about. For symmetry with `append()`, 
    /// the text is passed in as a &str, but in general it should only be
    /// one character long.
    fn skip(&mut self, begin: usize, text: &str) {
        println!("No rule matched at pos {} ('{}')", begin, &text[0..1]);
    }
}

impl RegexTokenizer for EnglishTokenizer {}
