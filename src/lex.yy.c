#include <stdio.h>
#include <stdlib.h>

#define ECHO fwrite(yytext,yyleng,1,yyout)

#include <stdio.h>

FILE* yyin = NULL;
FILE* yyout = NULL;
char *yytext;
int yyleng = 0;
int yywarp();
void yylex();
