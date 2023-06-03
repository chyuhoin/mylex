%{
   int key=0;
   int mark=0;
%}

ALPHA [a-zA-Z]
MARK  {ALPHA}+[a-zA-Z0-9]*
KEY   if|else|int|double|return

%%
{KEY}					printf("KEY ");key++;
{MARK}					printf("MARK "); mark++;
\n						printf("\n");
.

%%
int main(int argc, char **argv)
{
  yyin = fopen(argv[1], "r");
  yylex();
  printf("\nMARK=%d  KEY=%d",mark,key);
  yywrap();
}
int yywrap()
{
	return 1;
}