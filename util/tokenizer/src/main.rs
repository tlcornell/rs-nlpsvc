////////////////////////////////////////////////////////////////////////////
//
// package: regex_tokenizer
//
// main.rs 

extern crate getopts;
extern crate nlpsvc_regex;

use getopts::Options;
use std::env;
use std::process;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

mod regex_tokenizer;
use regex_tokenizer::TokenizerBuilder;

use nlpsvc_regex::reinterp::TokenSink;


struct AppConfig {
    text_file: Option<String>,
}

impl AppConfig {
    fn new() -> AppConfig {
        AppConfig { 
            text_file: None,
        }
    }
}

fn configure() -> AppConfig {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this message and exit");
    opts.optopt("f", "file", "match text from file", "NAME");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&args[0], &opts);
    }

    let mut cfg: AppConfig = AppConfig::new();
    cfg.text_file = matches.opt_str("f");

    cfg
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("\nUsage: {} [options]", program);
    print!("{}", opts.usage(&brief));
    println!("\nIf no file is given, input will be read from stdin.");
    process::exit(1);
}



struct TextSource {
    text: String,
}

impl TextSource {
    pub fn new(cfg: &AppConfig) -> TextSource {
        // Get the text to match against (from file or stdin)
        let mut txt = String::new();
        match cfg.text_file {
            None => {
                let stdin = io::stdin();
                stdin.lock().read_to_string(&mut txt).unwrap();
            },
            Some(ref fname) => {
                let fpath = Path::new(&fname);
                let mut f = File::open(fpath).expect("Could not open file");
                f.read_to_string(&mut txt).expect("Could not read file");
            }
        }  

        TextSource { text: txt }      
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }
}



fn word_action (tok: &str) {
    println!("WORD [{}]", tok);
}

fn num_action(tok: &str) {
    println!("NUMBER [{}]", tok);
}

fn punct_action(tok: &str) {
    println!("PUNCT [{}]", tok);
}

struct EnglishTokenSink;

impl TokenSink for EnglishTokenSink {
    /// Append a token
    ///
    /// Append a token starting at `begin` with text `text`, that 
    /// matched rule #`rule_id`.
    fn append(&mut self, begin: usize, text: &str, rule_id: usize) {
        match rule_id {
            0 => { word_action(text); }
            1 => { num_action(text); }
            2 => { punct_action(text); }
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

fn main() {
    let cfg = configure();
    let text_src = TextSource::new(&cfg);

    let mut english_tokenizer = TokenizerBuilder::new()
        .add_rule(r"(?i)[a-z]+")           // [0] words
        .add_rule(r"[0-9,.]*[0-9]+")       // [1] numbers
        .add_rule(r"[.,?!]")               // [2] punctuation
        .build();

    println!("\n{}", text_src.get_text());
    let mut sink = EnglishTokenSink {};
    english_tokenizer.run(text_src.get_text(), &mut sink);
}
