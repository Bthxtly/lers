%option noyywrap

%{
  #include <stdio.h>
  char **names = NULL;
  int current = 1;
%}

number    [0-9]+
id        [A-Za-z]+
%%

{number}  { printf("<NUMBER, %s >\n", yytext); }
{id}      { printf("<ID, %s >\n", yytext); }
BEGIN     { printf("<BEGIN, BEGIN>\n"); }
END       { printf("<END, END>\n"); }
+         { printf("<ADD, + >\n"); }
-         { printf("<SUB, - >\n"); }
*         { printf("<MUL, * >\n"); }
/         { printf("<DIV, / >\n"); }
:=        { printf("<ASSIGN, := >\n"); }
;         { printf("<SEMICOLON, ; >\n"); }
.|\n      { /* ignore whitespace */ }

%%

int main(int argc, char **argv) {
  if (argc < 2) {
    fprintf(stderr, "Usage: pascal files....\n");
    exit(1);
  }
  names = argv;

  do {
    yyin = fopen(names[current], "r");
    if (yyin == NULL) {
      fprintf(stderr, "cat: unable to open %s\n", names[current]);
    }

    yylex();
  } while (++current < argc);
  return 0;
}
