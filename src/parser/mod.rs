mod rule_parser;

use crate::lexer::{AuxiliaryToken, DeclarationToken, Lexer, RuleToken, Token};
use rule_parser::RuleParser;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    rule_parser: RuleParser<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Parser {
            lexer: Lexer::new(source),
            rule_parser: RuleParser::default(),
        }
    }

    pub fn gen_code(&mut self) -> String {
        let mut target_code = String::new();
        target_code.push_str("/* DECLARATION START */\n");

        while let Some(Ok(token)) = &self.lexer.next() {
            match token {
                Token::Decl(decl) => match decl {
                    DeclarationToken::OptionStart => {} // do nothing for now
                    DeclarationToken::Option(_) => {}   // do nothing for now
                    DeclarationToken::CCode(code) => {
                        target_code.push_str(code);
                    }
                },
                Token::RuleStart => {
                    target_code.push('\n');
                    target_code.push_str("\n/* RULE START */\n");
                    target_code.push_str(&self.gen_rule_code());
                    target_code.push_str("\n/* AUXILIARY START */\n");
                }
                Token::Auxi(aux) => match aux {
                    AuxiliaryToken::CCode(code) => {
                        target_code.push_str(code);
                    }
                },
                Token::Rule(_) => unreachable!(),
                Token::RuleEnd => unreachable!(),
            }
        }

        target_code
    }

    fn gen_rule_code(&mut self) -> String {
        while let Some(Ok(Token::Rule(rule))) = &self.lexer.next() {
            match rule {
                RuleToken::Pattern(pattern) => self.rule_parser.add_pattern(pattern),
                RuleToken::Action(action) => self.rule_parser.add_action(action),
            }
        }
        self.rule_parser.gen_code()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let source = r#"
%option noyywrap
/* comment */
%{
    c code block
%}

%%

pattern1    { action1(); }
pattern2    { action2(); }
pattern3    { action3(); }

%%

/* auxiliary code */
void helper() {}"#;
        let mut parser = Parser::new(source);
        let target_code = r#"/* DECLARATION START */
    c code block

/* RULE START */

#include <stdio.h>
#include <stdlib.h>

typedef unsigned long IdxType;
char *g_buffer;
IdxType g_buflen;
IdxType g_bufidx;

char* yytext = "YYTEXT"; /* TODO: implement yytext properly */

void read_file(const char *filename) {
  FILE *fp = fopen(filename, "r");
  if (fp == NULL) {
    perror("Failed to open file");
  }

  fseek(fp, 0, SEEK_END);
  g_buflen = ftell(fp);
  rewind(fp);

  g_buffer = malloc(g_buflen + 1);
  if (!g_buffer) {
    perror("Failed to allocate memory for buffer");
  }

  fread(g_buffer, 1, g_buflen, fp);
  g_buffer[g_buflen] = '\0'; // Null-terminate the string
  fclose(fp);
}

#define g_pattern_count 3
char *g_patterns[] = {
  "pattern1",
  "pattern2",
  "pattern3",
};

void action(int pattern_index) {
  if (pattern_index == 0) {
{ action1(); }
  }
  if (pattern_index == 1) {
{ action2(); }
  }
  if (pattern_index == 2) {
{ action3(); }
  }
}

void match() {
  while (g_bufidx < g_buflen) {
    for (int i = 0; i < g_pattern_count; i++) {
      const char *pat = g_patterns[i];
      if (g_buffer[g_bufidx] == pat[0]) {
        IdxType current_idx = g_bufidx;
        while (g_buffer[current_idx] == pat[current_idx - g_bufidx] &&
               pat[current_idx - g_bufidx] != '\0') {
          current_idx++;
        }
        if (pat[current_idx - g_bufidx] == '\0') {
          action(i);
        }
      }
    }
    ++g_bufidx;
  }
}

int yylex() {
  match();
  return 0;
}

/* AUXILIARY START */


/* auxiliary code */
void helper() {}"#;
        assert_eq!(parser.gen_code(), target_code);
    }
}
