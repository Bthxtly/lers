# Lers
Rewrite Lex in Rust with extension.
Also this is my data structure course design.

## Usage
Run the command below to generate a `lers.yy.c` file.

`cargo run -- analyzer.l`

## Feature
This project uses [my own regular expression engine](https://github.com/bthxtly/re).

## Example
- wc(word counter)
```sh
just wc # apply for test.txt
```

## Roadmap
- [x] tokenize the lex file
- [x] generate a basic `lers.yy.c` file without specific rules
- [ ] apply rules
    - [x] apply for simple rules (literal)
    - [x] build with Non-deterministic Finite Automate(NFA)
    - [ ] build with Deterministic Finite Automate(DFA)
    - [x] translate from regular expression to C code
- [ ] support multiple IO
    - [x] file
    - [ ] stdin
    - [ ] stdout
- [ ] support `yy*` variables
    - [x] yyin, yyout
    - [x] yytext, yyleng
    - [ ] more...
