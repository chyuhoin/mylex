use std::fs::{OpenOptions, File};
use std::io::prelude::*;
use crate::charset::to_origin_ch;
use crate::input::Sentence;
use crate::to_dfa::Dfa;
use crate::combine_dfa::Vertex;

fn get_output_file() -> File{
    let file = OpenOptions::new()
        .append(true)
        .open("lex.yy.c")
        .unwrap();
    return file;
}

/*
输出partA是头文件、宏定义和函数定义等信息，原封不动复制即可
 */
pub fn print_part_a(text: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .open("lex.yy.c")
        .unwrap();
    let shader = format!(r#"
#include <stdio.h>
#include <stdlib.h>

#define ECHO fwrite(yytext,yyleng,1,yyout)
{}
FILE* yyin = NULL;
FILE* yyout = NULL;
char *yytext;
int yyleng = 0;
int yywarp();
void yylex();
    "#, text);
    if let Err(e) = writeln!(file, "{}\n", shader.trim()) {
        println!("cannot write: {}", e);
    }
}

/*
输出partB是边信息，枚举dfa里面的所有边并打印到目标文件
 */
pub fn print_part_b(dfa :&Dfa<Vertex, char>) {
    let mut edge_string = String::from("{-1, -1, 0},\n");
    let mut edge_num = 0;
    for u in &dfa.points {
        if let Some(e) = dfa.graph.get(&u) {
            for v in e {
                let one_edge = format!("    {{{}, {}, {:?}}},\n", u.id, v.0.id, to_origin_ch(v.1));
                edge_string.push_str(&one_edge);
                edge_num += 1;
            }
        }
    }

    let mut file = get_output_file();
    let shader = format!(r#"
typedef struct {{
    int from;
    int to;
    char val;
}} YYEdge;
const YYEdge yy_edges[] = {{
    {}
}};
const int yy_edge_num = {};
    "#, edge_string.trim(), edge_num);
    if let Err(e) = writeln!(file, "{}\n", shader.trim()) {
        println!("cannot write: {}", e);
    }
}

/*
输出partC是点信息，枚举dfa里面的所有点的end信息并打印到目标文件
 */
pub fn print_part_c(dfa :&Dfa<Vertex, char>) {
    let mut point_string = String::from("-1,\n");
    let mut point_num = 0;
    for u in &dfa.points {
        let one_point = format!("    {},\n", u.end);
        point_string.push_str(&one_point);
        point_num += 1;
    }

    let mut file = get_output_file();
    let shader = format!(r#"
const int yy_vertexs_tag[] = {{
    {}
}};
const int yy_vertex_num = {};

int yy_dfa[{} << 1][200];
void add_edge(int x, int y, char c) {{
    yy_dfa[x][c] = y;
}}

    "#, point_string.trim(), point_num, point_num);
    if let Err(e) = writeln!(file, "{}\n", shader.trim()) {
        println!("cannot write: {}", e);
    }
}

pub fn print_part_d(sents: &Vec<Sentence>) {
    let mut action_string = String::from("");
    for (i, sent) in sents.iter().enumerate() {
        if sent.action == "." {
            let one_action = format!("        default: {{{} break;}}\n", sent.action);
        action_string.push_str(&one_action);
        } else {
            let one_action = format!("        case {}: {{{} break;}}\n", i, sent.action);
            action_string.push_str(&one_action);
        }
    }

    let mut file = get_output_file();
    let shader = format!(r#"
void yywork(int work) {{
    switch (work) {{
        {}
    }}
}}

int yy_state;
void yy_match(char c) {{
    int forward = yy_dfa[yy_state][c];
    if(forward == 0) {{
        yywork(yy_vertexs_tag[yy_state]);
        forward = yy_dfa[1][c];
        yyleng = 0;
    }}
    yy_state = forward;
}}
    "#, action_string.trim());
    if let Err(e) = writeln!(file, "{}\n", shader.trim()) {
        println!("cannot write: {}", e);
    }
}

pub fn print_part_e(text: &str) {
    let mut file = get_output_file();
    let shader = format!(r#"
void yyinit() {{
    if (yyin == NULL) yyin = stdin;
    if (yyout == NULL) yyout = stdout;

    yytext = malloc(sizeof(char) * 100);

    int i;
    for(i = 1; i <= yy_edge_num; i++) {{
        add_edge(yy_edges[i].from, yy_edges[i].to, yy_edges[i].val);
    }}
    yy_state = 1;
}}

void yylex() {{
    yyinit();

    while(1) {{
        char c = fgetc(yyin);
        if (feof(yyin))  break;
        yy_match(c);
        yytext[yyleng++] = c;
    }}

}}
{}
    "#, text);
    if let Err(e) = writeln!(file, "{}\n", shader.trim()) {
        println!("cannot write: {}", e);
    }
}