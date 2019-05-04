This repo is the home of a programming language project. It is nowhere near complete and the generic name `Lang` is a working title.

## Building
To build, simply run `cargo build`. This builds a debug version of the interpreter within `target/debug`.
`Lang` requires a minumum version of `rustc` which is `1.34`

## The Language
The grammar for the language is described in `Lang.g4`. This is an [antlr](https://www.antlr.org/) file, but antlr is not used to generate a lexer nor parser.

A simple example:
```
struct MyStruct {}

trait SayHi {
    fn hi(name: String) -> bool;
}

impl SayHi for MyStruct {
    fn hi(name: String) -> bool {
        print "Hi from " + name + "!";
        return true;
    }
}

let struct_instance: MyStruct = MyStruct();
let return_value: bool = struct_instance.hi("Lang");
print return_value;
```
You may notice that the language has a striking similarity to Rust. This is not an accident.

# Motivation
This project initially started as an effort to work through Bill Nystrom's Crating Interpreters in Rust
with some quirks and eventually evolved into it's own project language.

I believe there exists a space for a language that is: typed, can infer typing, has a garbage collector,
explicit error propogation while also not patronizing the programmer.

# Status
The language and interpreter are nowhere near complete enough to be meaningfully useful.
