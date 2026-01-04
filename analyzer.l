%option noyywrap
/*** Definition section ***/

%{
  /* C code to be copied verbatim */
  #include <stdio.h>
%}

digit   [0-9]
number  {digit}+

%%
    /*** Rules section ***/

    /* [0-9]+ matches a string of one or more digits */
{number}  {
              /* yytext is a string containing the matched text. */
              printf("Saw an integer: %s\n", yytext);
          }

lorem     { printf("ipsum\n"); }
fo*       { printf("%s has %d o's\n", yytext, yyleng-1); }
\n        { /* do nothing */ }
.         { printf("Unknow character: %s\n", yytext); }

%%
/*** C Code section ***/

int main(void)
{
  /* Call the lexer, then quit. */
  read_file("test.txt");
  yylex();
  return 0;
}
