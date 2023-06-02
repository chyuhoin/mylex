%{
    #include "syntax.tab.h"
    #define debug 0
    int line = 1;
    #define YY_USER_ACTION \
        yylloc.first_line = yylloc.last_line = line; 
%}
ws [\s\t]+
letter [a-zA-Z]
relop >|<|>=|<=|==|!=
type int|float
digit [0-9]
id {letter}({letter}|{digit})*
tenNumber ([0])|([1-9]{digit}*)
floatNumber {digit}*(\.{digit}+)

%%
{ws} {}/*no action*/
\n { line ++; } /* a new line */
";" { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng);yylval.tokenStruct = newNode("SEMI", NONVALTOKEN, line); if(debug) printf("<semi, line%d>\n", line);  return SEMI; } /*semi*/
"," { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng);yylval.tokenStruct = newNode("COMMA", NONVALTOKEN, line); if(debug) printf("<comma, line%d>\n", line); return COMMA; } /*comma*/
"=" { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng);yylval.tokenStruct = newNode("ASSIGNOP", NONVALTOKEN, line); if(debug) printf("<assignop, line%d>\n", line); return ASSIGNOP; } /*assign op*/
{relop} { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng);  yylval.tokenStruct = newNode("RELOP", NONVALTOKEN, line); if(debug) printf("<relop, line%d>\n", line); return RELOP; } /*rel op*/
"+" { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng);yylval.tokenStruct = newNode("PLUS", NONVALTOKEN,  line); if(debug) printf("<plus, line%d>\n", line); return PLUS; } /*plus*/
"-" { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng);yylval.tokenStruct = newNode("MINUS", NONVALTOKEN, line); if(debug) printf("<minus, line%d>\n", line); return MINUS; } /*minus*/
"*" { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng);yylval.tokenStruct = newNode("STAR", NONVALTOKEN, line); if(debug) printf("<star, line%d>\n", line); return STAR; } /*star*/
"/" { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng);yylval.tokenStruct = newNode("DIV", NONVALTOKEN, line); if(debug) printf("<div, line%d>\n", line); return DIV; } /*div*/
"." { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng);yylval.tokenStruct = newNode("DOT", NONVALTOKEN, line); if(debug) printf("<dot, line%d>\n", line); return DOT; } /*dot*/
"!" { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng);yylval.tokenStruct = newNode("NOT", NONVALTOKEN, line); if(debug) printf("<not, line%d>\n", line); return NOT; } /*not*/
{type} { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng); yylval.tokenStruct = newNode("TYPE", VALTOKEN, line); yylval.tokenStruct->idValue = op; if(debug) printf("<type, line%d, %s>\n", line, yytext); return TYPE; } /*type*/
"(" { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng);yylval.tokenStruct = newNode("LP", NONVALTOKEN, line); if(debug) printf("<LP, line%d>\n", line); return LP; } /*LP*/
")" { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng);yylval.tokenStruct = newNode("RP", NONVALTOKEN, line);if(debug)  printf("<RP, line%d>\n", line); return RP; } /*RP*/
"[" { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng);yylval.tokenStruct = newNode("LB", NONVALTOKEN, line);if(debug)  printf("<LB, line%d>\n", line); return LB; } /*LB*/
"]" { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng);yylval.tokenStruct = newNode("RB", NONVALTOKEN, line);if(debug)  printf("<RB, line%d>\n", line); return RB; } /*RB*/
"{" { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng);yylval.tokenStruct = newNode("LC", NONVALTOKEN, line);if(debug)  printf("<LC, line%d>\n", line); return LC; } /*LC*/
"}" { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng);yylval.tokenStruct = newNode("RC", NONVALTOKEN, line);if(debug)  printf("<RC, line%d>\n", line); return RC; } /*RC*/
"struct" { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng);yylval.tokenStruct = newNode("STRUCT", NONVALTOKEN, line); if(debug) printf("<struct, line%d>\n", line); return STRUCT; } /*struct*/
"return" { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng);yylval.tokenStruct = newNode("RETURN", NONVALTOKEN, line); if(debug) printf("<return, line%d>\n", line); return RETURN; } /*return*/
"if" {char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng); yylval.tokenStruct = newNode("IF", NONVALTOKEN,  line); if(debug) printf("<if, line%d>\n", line); return IF; } /*if*/
"else" { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng);yylval.tokenStruct = newNode("ELSE", NONVALTOKEN, line); if(debug) printf("<else, line%d>\n", line); return ELSE; } /*else*/
"while" { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng);yylval.tokenStruct = newNode("WHILE", NONVALTOKEN, line); if(debug) printf("<while, line%d>\n", line); return WHILE; } /*while*/
{id} { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng); yylval.tokenStruct = newNode("id", VALTOKEN, line); yylval.tokenStruct->idValue = op;if(debug)  printf("<id, line%d>\n", line); return ID; } /*id*/
{tenNumber} { char *op=malloc(sizeof(char)*yyleng);  strncpy(op, yytext, yyleng); yylval.tokenStruct = newNode("int", VALTOKEN, line); yylval.tokenStruct->intValue = atoi(op);if(debug) printf("<tennumber, line%d>\n", line); return INT; } /*ten number*/
{floatNumber} { char *op=malloc(sizeof(char)*yyleng); strncpy(op, yytext, yyleng);yylval.tokenStruct = newNode("float", VALTOKEN, line); yylval.tokenStruct->floatValue = atof(op);if(debug) printf("<floatnumber, line%d>\n", line); return FLOAT; } /*float number*/
. { printf("Error type A at line %d\n", line); } /*error*/

%%
int yywrap()
{  
    return 1;  
}