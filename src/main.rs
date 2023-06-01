mod input;
mod trans;
mod to_dfa;
mod charset;
mod minimize_dfa;
mod combine_dfa;
mod output;

use input::replace_reg;

use crate::charset::fix_str;
use crate::combine_dfa::{combine, print_standard_dfa};
use crate::input::div_reg_action;
use crate::minimize_dfa::minimize_dfa;
use crate::input::init;
use crate::output::*;
use crate::trans::translate_reg;
use crate::to_dfa::{convert, print_dfa};

fn main() {
    let s = init("../minic.l");
    print_part_a(&s[0]);
    let longtxt = replace_reg(&s[1], &div_reg_action(&s[2]));
    let mut dfas = Vec::new();
    for sent in &longtxt {
        let mut pattern = sent.reg.clone();
        if sent.reg.starts_with(".") {continue;}
        else if sent.reg.starts_with("\"") {
            let tmp_strs: Vec<&str> = sent.reg.split("\"").filter(|s| !s.is_empty()).collect();
            pattern = fix_str(tmp_strs[0])
        }
        else {pattern = translate_reg(&pattern)}
        // print!("{:?}\n", &pattern);
        let tmp_dfa = minimize_dfa(&convert(&pattern));
        dfas.push(tmp_dfa);
    }
    let final_dfa = combine(&dfas);
    print_part_b(&final_dfa);
    print_part_c(&final_dfa);
    print_part_d(&longtxt);
    print_part_e(&s[3])
}
