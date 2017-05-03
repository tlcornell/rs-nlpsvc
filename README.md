# An NLP Sandbox in Rust

This repository is intended to hold various components, written in Rust, 
that do various Natural Language Processing tasks. 
Holding it all together will be a web service for submitting analysis tasks,
which will analyze the submitted document and put the results somewhere.
In the meantime, each back end component is expected to come with a 
command line wrapper program that can be used independently, given suitable
input.

For starters I only intend to include support for:

1. Regular expression based tokenizers
2. Efficient dictionaries
3. Pattern matching on trees and graphs for parsing

As I get more comfortable with machine learning algorithms, I expect to try to
implement some of those as well, but first things first!

The main thing holding it all together will be some sort of (hopefully flexible)
Annotated Document interface, so that all components will speak a common language
of operations they are able to perform and queries they are able to make.

