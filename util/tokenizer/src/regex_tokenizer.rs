extern crate nlpsvc_regex;
extern crate annotated_document;

use nlpsvc_regex::reinterp::ThompsonInterpreter;
use nlpsvc_regex::reinterp::TokenizerAction;
use nlpsvc_regex::retrans::RegexTranslator;
use nlpsvc_regex::reparse;

/**
 * The interpreter is actually a Thompson VM partially applied to a
 * given program. That is, the regex program is like "virtual firmware"
 * in the machine. Short version: it looks like a generic interpreter, 
 * but it is not; the program it interprets is fixed during construction.
 */
pub struct RegexTokenizer {

    interpreter: ThompsonInterpreter,

}

impl RegexTokenizer {

    pub fn run(&mut self, text: &str) {
        self.interpreter.apply(text);
    }

}


pub struct TokenizerBuilder {
    compiler: RegexTranslator,
    rule_nbr: usize,
    actions: Vec<TokenizerAction>,
}

impl TokenizerBuilder {

    pub fn new() -> TokenizerBuilder {
        TokenizerBuilder {
            compiler: RegexTranslator::new(),
            rule_nbr: 0,
            actions: vec![],
        }
    }

    /**
     * This should compile the pattern and add to the current program.
     */
    pub fn add_rule(
        mut self, 
        pattern: &str, 
        action: TokenizerAction,
    ) -> TokenizerBuilder {
        let tree = reparse::parse(pattern);
        println!("{}", tree);
        self.compiler.compile(&tree, self.rule_nbr);
        // TODO: The action should be added to an action list here.
        self.actions.push(action);

        self.rule_nbr += 1;
        self
    }

    pub fn build(mut self) -> RegexTokenizer {
        self.compiler.finish();       // ground instruction labels
        self.compiler.print_prog();
        RegexTokenizer {
            interpreter: ThompsonInterpreter::new(self.compiler.prog,
                                                  self.actions),
        }
    }

}


