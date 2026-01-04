%option noyywrap

%{
  int cc = 0, wc = 0, lc = 0;
%}

%%

[^ \t\n]+  {cc += yyleng; ++wc;}

[ \t]+     {cc += yyleng;}

\n         {++lc; ++cc;}

%%

int main() {
  read_file("test.txt");
  yylex();
  printf( "%8d %8d %8d\n", lc, wc, cc );
  return 0;
}
