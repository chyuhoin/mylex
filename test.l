%{
   #include<string.h>

   int statue;
   char name[20];
   
   int val_int=0;
   double val_double=0;
%}

VAR   [a-zA-Z]+[a-zA-Z0-9]*
KEY   int|double
VALUE [1-9]+[0-9]*|[0-9]+\.[0-9]+

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
				printf("<%s,int,%d>\n",name,val_int);
			else if (statue==2)
				printf("<%s,double,%lf>\n",name,val_double);
		}
\n		printf("\n");
.

%%
int main(int argc, char **argv)
{
  yylex();
  yywrap();
}
int yywrap()
{
	return 1;
}