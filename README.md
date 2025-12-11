# Lers
Rewrite Lex in Rust with extension.
Also this is my data structure course design.

## Usage
Run the command below to generate a `lex.yy.c` file.

`cargo run -- analyzer.l`

## Roadmap
- [x] tokenize the lex file
- [ ] generate a basic `lex.yy.c` file without specific rules
- [ ] apply rules
    - [ ] parse rule with regex crate
    - [ ] insert code
    - [ ] implement matching with Deterministic Finite Automate(DFA) and remove regex crate
- [ ] support `yy*` variables
more...
