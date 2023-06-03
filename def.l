%{
   #include<string.h>

   int statue;
   char name[20];
   
   int val_int=0;
   double val_double=0;
%}

ALPHA [a-zA-Z]
VAR   {ALPHA}+[a-zA-Z0-9_]*
KEY   int|double
DIGIT [0-9]
VALUE [1-9]+{DIGIT}*|{DIGIT}+\.{DIGIT}+

%%
{KEY}   { 
			if (yytext[0]=='i') 
				statue=1; 
			else 
				statue=2;
		}
{VAR}   strncpy(name,yytext,yyleng);
{VALUE} {
			if (statue==1)
				sscanf(yytext,"%d",&val_int);
			else if (statue==2)
				sscanf(yytext,"%lf",&val_double);
		}
;		{
			if (statue==1)
				printf("<%s,int,%d>",name,val_int);
			else if (statue==2)
				printf("<%s,double,%lf>",name,val_double);
		}
\n		printf("\n");
.

%%
int main(int argc, char **argv)
{
  yyin = fopen("test.txt", "r");
  yylex();
  yywrap();
}
int yywrap()
{
	return 1;
}