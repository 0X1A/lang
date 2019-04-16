This repo is the home of a programming language project. It is nowhere near complete and the generic name `Lang` is a working title.

## Building
To build, simply run `cargo build`. This builds a debug version of the interpreter within `target/debug`.

## The Language
The grammar for the language is described in `Lang.g4`. This is an [antlr](https://www.antlr.org/) file, but antlr is not used to generate a lexer nor parser.