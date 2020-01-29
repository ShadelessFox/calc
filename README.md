## Simple calculator written in Rust

This project is just an experiment with goals to learn Rust language and research aspects of lexing and parsing data.
Sources has no dependencies and can be found in single file located at src/main.rs, it's not well-documented, but I think code is already pretty
simple to understand.

Calculator consist of three parts:
1) Lexer. Convert input text into set of tokens, emitting one token per call
2) Parser. Convert token stream into AST (Abstract Syntax Tree) for future evaluation
3) Evaluate. Recursively execute root node with its parameter(-s)

## Syntax
This calculator uses algebraic notation and has built-in constants and functions.

```
> 5 + 5
10
> 5 + 5 * 5
30
> 7 + 15 / 3
12
> 10 ^ 3
1000
> 123 % 3
3
```

Constants are implemented as functions with zero parameters.

```
> log 10 100
2
> pi
3.141592653589793
```

Functions are not greedy and only takes a single number or expression inside parentheses.

```
> cos pi * 2
-2
> cos (pi * 2)
1
```

## Building
1) Clone project using ```git clone``` or just download as zip.
2) Compile using ```cargo build --release``` or run directly ```cargo run```
3) Type some expressions to see if it works

## Licence
There's no licence, you are free to use parts of this code at your own.