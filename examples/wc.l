%option noyywrap

%{
  #include <stdio.h>
  char **names = NULL;
  int current = 1;
  /* newline, word, byte counts */
  int lc = 0, wc = 0, cc = 0;
%}

%%

[^ \t\n]+  {cc += yyleng; ++wc;}

[ \t]+     {cc += yyleng;}

\n         {++lc; ++cc;}

%%

int main(int argc, char **argv) {
  if (argc < 2) {
    fprintf(stderr, "Usage: wc files....\n");
    exit(1);
  }
  names = argv;

    printf( "  line  word  byte  file\n");
  do {
    yyin = fopen(names[current], "r");
    if (yyin == NULL) {
      fprintf(stderr, "cat: unable to open %s\n", names[current]);
    }

    yylex();
    printf( "%6d%6d%6d  %s\n", lc, wc, cc, names[current]);
  } while (++current < argc);
  return 0;
}
