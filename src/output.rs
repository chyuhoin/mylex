use std::fs::{OpenOptions, File};
use std::io::prelude::*;
use crate::to_dfa::Dfa;
use crate::combine_dfa::Vertex;

fn get_output_file() -> File{
    let file = OpenOptions::new()
        .append(true)
        .open("lex.yy.c")
        .unwrap();
    return file;
}

pub fn print_partA(text: &str) {
    let mut file = get_output_file();
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
    if let Err(e) = writeln!(file, "{}", shader.trim()) {
        println!("cannot write: {}", e);
    }
}

pub fn print_partB(dfa :&Dfa<Vertex, char>) {
    let mut edge_string = String::from("{-1, -1, 0},\n");
    for u in &dfa.points {
        if let Some(e) = dfa.graph.get(&u) {
            for v in e {
                let one_edge = format!("{{{}, {}, {:?}}},\n", u.id, v.0.id, v.1);
                edge_string.push_str(&one_edge);
            }
        }
    }
}