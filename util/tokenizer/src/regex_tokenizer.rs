extern crate nlpsvc_regex;
extern crate annotated_document;

use nlpsvc_regex::reinterp::ThompsonInterpreter;
use nlpsvc_regex::reinterp::TokenSink;
use nlpsvc_regex::retrans::RegexTranslator;
use nlpsvc_regex::reparse;


/// A regular expression based pattern-action tokenizer
///
/// The `interpreter` is actually a Thompson VM partially applied to a
/// given program. That is, the regex program is like "virtual firmware"
/// in the machine. Short version: it looks like a generic interpreter, 
/// but it is not; the program it interprets is fixed during construction.
pub struct RegexTokenizer {

    interpreter: ThompsonInterpreter,

}

impl RegexTokenizer {

    pub fn run(&mut self, text: &str, sink: &mut TokenSink) {
        self.interpreter.apply(text, sink);
    }

}


pub struct TokenizerBuilder {
    compiler: RegexTranslator,
    rule_nbr: usize,
}

impl TokenizerBuilder {

    pub fn new() -> TokenizerBuilder {
        TokenizerBuilder {
            compiler: RegexTranslator::new(),
            rule_nbr: 0,
        }
    }

    /**
     * This should compile the pattern and add to the current program.
     */
    pub fn add_rule(mut self, pattern: &str) -> TokenizerBuilder {
        let tree = reparse::parse(pattern);
        println!("{}", tree);
        self.compiler.compile(&tree, self.rule_nbr);

        self.rule_nbr += 1;
        self
    }

    pub fn build(mut self) -> RegexTokenizer {
        self.compiler.finish();       // ground instruction labels
        self.compiler.print_prog();
        RegexTokenizer {
            interpreter: ThompsonInterpreter::new(self.compiler.prog),
        }
    }

}


