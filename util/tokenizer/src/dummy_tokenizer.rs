/// dummy_tokenizer.rs

use nlpsvc_regex::reinterp::TokenSink;

fn stub_word_action (tok: &str) {
    println!("WORD [{}]", tok);
}

fn stub_num_action(tok: &str) {
    println!("NUMBER [{}]", tok);
}

fn stub_punct_action(tok: &str) {
    println!("PUNCT [{}]", tok);
}

pub struct DummyTokenizerRules;

impl TokenSink for DummyTokenizerRules {
    /// Append a token
    ///
    /// Append a token starting at `begin` with text `text`, that 
    /// matched rule #`rule_id`.
    fn append(&mut self, begin: usize, text: &str, rule_id: usize) {
        match rule_id {
            0 => { stub_word_action(text); }
            1 => { stub_num_action(text); }
            2 => { stub_punct_action(text); }
            _ => { panic!("Unrecognized rule ID"); }
        }
    }

    /// Skip an unhandled character
    ///
    /// The character at `begin` is not the first character of any pattern
    /// that this tokenizer knows about. For symmetry with `append()`, 
    /// the text is passed in as a &str, but in general it should only be
    /// one character long.
    fn skip(&mut self, begin: usize, text: &str) {
        println!("No rule matched at pos {}", begin);
    }

}

