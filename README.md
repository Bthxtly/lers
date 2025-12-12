# Lers
Rewrite Lex in Rust with extension.
Also this is my data structure course design.

## Usage
Run the command below to generate a `lex.yy.c` file.

`cargo run -- analyzer.l`

## Roadmap
- [x] tokenize the lex file
- [x] generate a basic `lex.yy.c` file without specific rules
- [ ] apply rules
    - [ ] apply for simple rules (literal)
    - [ ] build a regular expression engine with Deterministic Finite Automate(DFA)
    - [ ] translate from regular expression to C code
- [ ] support `yy*` variables
more...
