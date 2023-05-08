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

typedef struct {
    int from;
    int to;
    char val;
} YYEdge;
const YYEdge yy_edges[] = {
    {-1, -1, 0},
    {1, 2, '0'},
    {1, 3, '1'},
    {1, 3, '2'},
    {1, 3, '3'},
    {1, 3, '4'},
    {1, 3, '5'},
    {1, 3, '6'},
    {1, 3, '7'},
    {1, 3, '8'},
    {1, 3, '9'},
    {1, 4, 'A'},
    {1, 4, 'B'},
    {1, 4, 'C'},
    {1, 4, 'D'},
    {1, 4, 'E'},
    {1, 4, 'F'},
    {1, 4, 'G'},
    {1, 4, 'H'},
    {1, 4, 'I'},
    {1, 4, 'J'},
    {1, 4, 'K'},
    {1, 4, 'L'},
    {1, 4, 'M'},
    {1, 4, 'N'},
    {1, 4, 'O'},
    {1, 4, 'P'},
    {1, 4, 'Q'},
    {1, 4, 'R'},
    {1, 4, 'S'},
    {1, 4, 'T'},
    {1, 4, 'U'},
    {1, 4, 'V'},
    {1, 4, 'W'},
    {1, 4, 'X'},
    {1, 4, 'Y'},
    {1, 4, 'Z'},
    {1, 4, 'a'},
    {1, 4, 'b'},
    {1, 4, 'c'},
    {1, 4, 'd'},
    {1, 4, 'e'},
    {1, 4, 'f'},
    {1, 4, 'g'},
    {1, 4, 'h'},
    {1, 4, 'i'},
    {1, 4, 'j'},
    {1, 4, 'k'},
    {1, 4, 'l'},
    {1, 4, 'm'},
    {1, 4, 'n'},
    {1, 4, 'o'},
    {1, 4, 'p'},
    {1, 4, 'q'},
    {1, 4, 'r'},
    {1, 4, 's'},
    {1, 4, 't'},
    {1, 4, 'u'},
    {1, 4, 'v'},
    {1, 4, 'w'},
    {1, 4, 'x'},
    {1, 4, 'y'},
    {1, 4, 'z'},
    {3, 3, '0'},
    {3, 3, '1'},
    {3, 3, '2'},
    {3, 3, '3'},
    {3, 3, '4'},
    {3, 3, '5'},
    {3, 3, '6'},
    {3, 3, '7'},
    {3, 3, '8'},
    {3, 3, '9'},
    {4, 4, 'A'},
    {4, 4, 'B'},
    {4, 4, 'C'},
    {4, 4, 'D'},
    {4, 4, 'E'},
    {4, 4, 'F'},
    {4, 4, 'G'},
    {4, 4, 'H'},
    {4, 4, 'I'},
    {4, 4, 'J'},
    {4, 4, 'K'},
    {4, 4, 'L'},
    {4, 4, 'M'},
    {4, 4, 'N'},
    {4, 4, 'O'},
    {4, 4, 'P'},
    {4, 4, 'Q'},
    {4, 4, 'R'},
    {4, 4, 'S'},
    {4, 4, 'T'},
    {4, 4, 'U'},
    {4, 4, 'V'},
    {4, 4, 'W'},
    {4, 4, 'X'},
    {4, 4, 'Y'},
    {4, 4, 'Z'},
    {4, 4, 'a'},
    {4, 4, 'b'},
    {4, 4, 'c'},
    {4, 4, 'd'},
    {4, 4, 'e'},
    {4, 4, 'f'},
    {4, 4, 'g'},
    {4, 4, 'h'},
    {4, 4, 'i'},
    {4, 4, 'j'},
    {4, 4, 'k'},
    {4, 4, 'l'},
    {4, 4, 'm'},
    {4, 4, 'n'},
    {4, 4, 'o'},
    {4, 4, 'p'},
    {4, 4, 'q'},
    {4, 4, 'r'},
    {4, 4, 's'},
    {4, 4, 't'},
    {4, 4, 'u'},
    {4, 4, 'v'},
    {4, 4, 'w'},
    {4, 4, 'x'},
    {4, 4, 'y'},
    {4, 4, 'z'},
};
const int yy_edge_num = 124;

const int yy_vertexs_tag[] = {
    -1,
    -1,
    0,
    0,
    1,
};
const int yy_vertex_num = 4;

int yy_dfa[4 << 1][200];
void add_edge(int x, int y, char c) {
    yy_dfa[x][c] = y;
}

void yywork(int work) {
    switch (work) {
        case 0: {ECHO; break;}
        case 1: { break;}
        case 2: { break;}
    }
}

int yy_state;
void yy_match(char c) {
    int forward = yy_dfa[yy_state][c];
    if(forward == 0) {
        yywork(yy_vertexs_tag[yy_state]);
        forward = yy_dfa[1][c];
        yyleng = 0;
    }
    yy_state = forward;
}

void yyinit() {
    if (yyin == NULL) yyin = stdin;
    if (yyout == NULL) yyout = stdout;

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
        yy_match(c);
        yytext[yyleng++] = c;
    }

}


int main(int argc, char **argv)
{
  yylex();
  yywrap();
}
int yywrap()
{
	return 1;
}

