use logos::Logos;

// simply put all codes into the target file
#[derive(Logos, Debug, PartialEq)]
pub enum AuxiliaryToken<'a> {
    #[regex(r"(?s).+", |lex|lex.slice(),  allow_greedy = true)]
    CCode(&'a str),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenize() {
        let aux = r#"
            /*** C Code section ***/

            int main(void)
            {
                /* Call the lexer, then quit. */
                yylex();
                return 0;
            }
            "#;
        let mut lex = AuxiliaryToken::lexer(aux);
        assert!(matches!(lex.next(), Some(Ok(AuxiliaryToken::CCode(_)))));
        assert_eq!(lex.next(), None);
    }
}
