#include <stdio.h>
#include <stdlib.h>

#define ECHO fwrite(yytext,yyleng,1,yyout)

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
    {-1, -1, 0},
    {1, 2, '0'},
    {1, 3, '1'},
    {1, 4, 'A'},
};
const int yy_edge_num = 3;

const int yy_vertexs_tag[] = {
    -1,
    -1,
    0,
    0,
    1,
};
const int yy_vertex_num = 4;

int yy_dfa[yy_vertex_num << 1][200];
void add_edge(int x, int y, char c) {
    yy_dfa[x][c] = y;
}

void yywork(int work) {
    switch (work) {
        case 0: {
            ECHO;
            break;
        }
        case 1: {
            break;
        }
        default: {
            break;
        }
    }
}

int yy_state;
void yy_match(char c) {
    int forward = yy_dfa[yy_state][c];
    if(forward == 0) {
        yywork(yy_vertexs_tag[c]);
    }
    yy_state = forward;
}

void yyinit() {
    if (yyin == NULL) yyin = stdin;
    if (yyout == NULL) yyout = stdin;

    yytext = malloc(sizeof(char) * 100);

    int i;
    for(i = 1; i <= yy_edge_num; i++) {
        add_edge(yy_edges[i].from, yy_edges[i].to, yy_edges[i].val);
    }
    yy_state = 1;
}

void yylex() {
    yyinit();

    while(1) {
        char c = fgetc(yyin);
        if (feof(yyin))  break;
        yytext[yyleng++] = c;
        yy_match(c);
    }

}