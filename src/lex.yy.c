#include <stdio.h>
#include <stdlib.h>

FILE* yyin = NULL;
FILE* yyout = NULL;
char *yytext;
int yyleng = 0;
int yywarp();
void yylex();

typedef struct {
    int from;
    int to;
    char val;
} YYEdge;
const YYEdge yy_edges[] = {
    {1, 2, 'a'},
    {2, 3, 'b'},
};
const int yy_edge_num = 2;

typedef struct {
    int work;
} YYVertex;
const YYVertex yy_vertexs[] = {
    {-1},
    {0},
    {2},
    {3},
};
const int yy_vertex_num = 3;

void yywork(int work) {
    switch (work) {
        case 1: {
            break;
        }
        case 2: {
            break;
        }
        case 3: {
            break;
        }
        default: {
            break;
        }
    }
}

void yyinit() {
    if (yyin == NULL) yyin = stdin;
    if (yyout == NULL) yyout = stdin;

    yytext = malloc(sizeof(char) * 100);
}

void yylex() {
    yyinit();

    while(1) {
        char c = fgetc(yyin);
        if (feof(yyin))  break;
        yytext[yyleng++] = c;
    }

}