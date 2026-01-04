%option noyywrap

%{
  #include <stdio.h>
  char **names = NULL;
  int current = 1;
%}

%%

%%

int main(int argc, char **argv) {
  if (argc < 2) {
    fprintf(stderr, "Usage: cat files....\n");
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
}

