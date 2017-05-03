# Applying a battery of regexes

Either there is a vector of Thompson VMs, 
or there is one VM with a battery of regex programs,
or there is one regex program, with a collection of start states
and a map from Match lines to rule numbers, or equivalent.

Right now, I favor the latter. 
Which means that it is the *compiler* that mainly has to change.
The interpreter has to change mainly in that there is a different structure coming in, and also in the way it interprets Match states. Also intitialization is different.

If we include the parser in the compiler, then:

1. The parser takes a vector of regexes and returns a vector of Terms.
2. The translator takes a vector of Terms and returns a Program.

Or we take each regex through to a program, appending them as we go.
Note that that means that we would need to change (e.g.) `jmp 107` to `jmp +15`.
That is, we need some kind of relative addressing.

[
	Split(+1, +size(trans(subs[0])) + 1),
	L1:
] ++ translate(subs[0]) ++ [
	Jump(+size(trans(subs[1]))),
	L2:
] ++ translate(subs[1]) ++ [
	L3:
]

For each regex in the input:

1. Translate to a Term
2. Encode the Term as a program, *given the current program as a base*.

That is, every step takes a Term and the current state of the Program, and returns a new Program, extended by the code generated from the Term.


## Lifetimes and Borrowing

So if we represent "characters" as UTF-8 strings (often just 1 byte: think of the overhead!), and store them into character range data objects as `&str` slices, then we have to deal with passing lifetime parameters all over the place. They quickly pollute everything, since CharRangeData is used in both the AST and the Program. 

But if we use Strings instead, then we end up with heap allocations even for single byte characters. And these are presumably the most common, with 2-byte sequences being next. It seems like a horrible waste. UTF-8 to Unicode conversion really doesn't seem so bad in comparison.

One possible solution might be to try to bring the regex string closer to where we are borrowing from it. 

* Maybe have a single String that will be the alphabet 
  (the parts we actually mention in the regex). 
  Then hold copies of that close to the AST and to the Program. 
* A problem with that is that there is no good way to tell if a "character" 
  is already in the alphabet.
* Otherwise, we could try something like making both the tree 
  and the program code be members of a single structure 
  that also includes the original regex list.

But in all these cases we cannot avoid polluting the Program, for example, with lifetime parameters.

To avoid that, we would have to move the data objects out of the instructions themselves, but how? What would be the data package of a CharClass instruction then? 
The best I can come up with is an integer index into a vector of data objects.
But really that ends up just being a way of doing pointers under the radar, and basically hacking the borrow checker.

## Making "UTF-8 Characters" Cloneable

The basic idea is to have a 4-byte array for holding the character bytes.

* Memory-wise, that's the same size as a character, even for 1-byte sequences.
* But we save the bit-twiddling of decoding and encoding.
* Or do we? How do we know what size the sequence is (i.e., how many live 
  bytes in the array) without examining the high bits of the leader?

The length test can be done with a sequence of if-thens on just the leader byte.

How often does it have to be done? 

* If we can be sure that the dead bytes are all zero, then it is safe 
  to compare these arrays without worrying about length. Otherwise, 
  the anser is: every time we compare them.
* If they are safe to compare without close examination, then I think we 
  rarely need to know what bytes they actuall contain. 
* Also, we can treat them as zero-terminated strings (unless they are 4 bytes).

It would be ideal if we could just shove the arrays in registers and compare them using integer comparison, instead of looping through the bytes. But if we are talking almost all 1- and 2-byte sequences, then I guess it amounts to the same thing. 
Especially if the compiler is clever enough to unroll the loops enough.

So, *cloning*. Basically what we need is a 



## Propriety of CharClass Instructions

If we were trying to compile down to a DFA with an alphabet of *bytes*, then what does a character class expression even mean? 
It basically represents a collection of whole paths, matching byte *sequences* in a single instruction.

To fit within a byte-by-byte matching regime, wouldn't you have to compile them to DFAs, and then patch them in as *sub-automata*?

Maybe this is backwards. 

1. `char(c)` instructions also consume all the bytes of a utf8 byte sequence
2. At any point in the text, if each thread is sitting on a char or class 
   instruction, each thread will either consume the whole current character 
   byte-sequence, or it will terminate.

So it's really the notion of pre-compiled character classes that is the odd thing here. 

* It must be the case that any such pre-compiled byte-DFA matches a single 
  character (or at most one). Character-by-character scanning of the text 
  remains an invariant.
* The pre-compiled DFA idea remains basically an optimization. 
  We can treat character classes as composite instructions -- sub-programs, 
  essentially -- with each interval or single character or named character class 
  counting as an instruction in the sub-program.

So maybe we don't have a CharClass instruction? Maybe we break it up into a disjunction of atomic predicates?
But then we have to deal with *negation* head-on, and do a whole conversion to 
NNF or something similar.
So by "disjunction" here, I don't really mean "regular alternation" but rather some top-level CharClass instruction that wraps a whole sequence of sub-instructions.

That means that the grammar of a CharClass instruction is a little different.

```
<CharClass> ::= "["  "^"?  <CharPredicate>+  "]"
<CharPredicate> ::= <CharAtom>
                  | <CharRange>
                  | <NamedCharClass>
<CharRange> ::= <CharAtom>  "-"  <CharAtom>
```

If we focus on translating `<CharPredicate>` as single instructions, 
then maybe the resulting program looks like:

```
L0: pred a-z / goto L1
    pred [:digit:] / goto L1
    char 😀 / goto L1
    fail
L1: 
```

A negated character class might be implemented as:

```
L0: pred a-z / goto L1
    pred [:digit:] / goto L1
    char 😀 / goto L1
    jump L2
L1: fail
L2: 
```

Note that without a `goto` field in Instructions, this becomes much gnarlier,
with lots of `split` instructions, and so lots of threads (though most branches will fail quickly).

If we don't split CharClass into a sequence of instructions, then instead the argument to the single CharClass instruction becomes much more intricate.

## CharClass "Microcode"

Basically, a character class has a Boolean flag for if it's negated,
and a list of primitive *character predicates*. There are (so far) three kinds:

1. A single character. This behaves just like a `char` instruction.
2. A character range, represented by its two endpoints.
3. A named character class.
4. There's also a `fail` instruction, for implementing negation.

A single character predicate can be simulated by a character range with equal endpoints, and a named class could conceivably be compiled out into a sequence of primitive ranges. But for now, I am voting to just have 3 different instructions.

All three instructions require `goto` fields.

Named classes have to be drawn from some kind of *library* of predicates. 
So we need some way to dispatch the class name to a particular function implementing the predicate. 
And the compiler has to know what the valid class names are.

Unfortunately, the above simple translation scheme does not work with the standard approach to `goto`. Currently even trivial `char` instructions have a `goto` field, that simply points to the next instruction.
So there is really nothing in the interpreter that would implement the two-way "true-`goto`" and "false-`goto`" that we need here.

That just means that we can't use a straight-up `char` instruction for single character predicates.

## Threads

Alternatively, we could do a `split` and try all the predicates each in their own thread. 

```
L0: split L1, L2
L1: range a-z goto L
L2: split L3, L4
L3: pred [:digit:] goto L
L4: char 😀 goto L
L:
```

Not sure how we would do negation in such a case, and it adds a lot of extra states to the thread space.

## CharClass Sub-Interpreter

A relatively straightforward solution is to just have a single CharClass instruction, and have an interpreter that loops through all the individual predicates.
If any predicate succeeds, then we jump to the `goto` location.
Otherwise the thread dies.

And we reverse the logic for negated character classes.



# Goto

The problem is the same for back-filling label slots in `jmp` or `split` instructions. 

```
translate(e1|e2) -->
		split L1, L2
	L1: translate(e1, L3)
	L2: translate(e2, L3)
	L3:
```

We can't know what L3 is until we have translated both e1 and e2. 
Ideally, we could embed some sort of pointer in the translations, 
and then just by assigning L3 to it at the end, properly fill in and close off the program fragment.

That is, *program fragments have slots* that need to be filled by whoever is composing in that fragment.

Passing L3 as the second argument to `translate()` is basically saying: construct a fragment with a slot and make damn sure to call it "L3".

One way to embed a bound slot in a term is to make it a lambda term.
So then `translate()` would return a function, which we would then apply to L3 once we knew what it was.

```
	let t_e1 = translate(e1);
	...
	let l3 = prog.len();
	t_e1(l3);
```

The expectation is that `t_e1()` has grabbed the `prog` variable from the environment. But it would be a reference, right? 

Otherwise, we have to go in and physically replace a placeholder value with the concrete label. That seems like what we'll have to do.

1. So assign a dummy value to the `goto` field of the relevant instruction.
2. Return a list of all instructions associated with that value?

Basically a map from labels to program lines that need to be filled in.

Consider the above example of `translate(e1|e2)`. Note that it is missing the label argument that we use when we call it recursively. We have to assume that we are calling it with something already there, and that we have to pass that something down to the recursive calls.

The recursive translation bottoms out at instructions like `char a` that have a `goto` field. That field can only have one element in it. So what happens to all the labels passed in from higher calls to `translate()`?


## Two-Stage Compilation

So it looks like first we have to compile to a pre-program that has instructions with *symbolic addresses* for `goto` and `split`.
Then we run through that pre-program and replace the symbolic addresses with concrete `usize` addresses.

That seems to mean we would have two sets of instructions, depending on whether they took strings or integers. 
```
enum Label {
	Temporary(String),
	Final(usize),
}
```
Something like that.

Ultimately, every instruction will be located at a symbolically labeled address.
At that point we can iterate through the program and compute the mapping from symbols to `usize`.
Though we may also be able to construct that mapping on the fly as a side-effect of pushing an instruction onto the program.

Unfortunately, this means that we can only handle Instruction objects via mapping. There will always be two cases to consider, at least in the opinion of the Rust compiler.
So it might be better to have two different kinds of instructions.

It is also possible that we could use *integers for our symbols*. 
The map in question then just becomes a permutation, for what that's worth.
The key point is that allows us to have a single type of instruction. 
We just vary the semantics of the `usize` data items.

* In the proto-program, the number *n* represents the *n*th label symbol.
* In the final program, the number *n* represents the *n*th program line.

