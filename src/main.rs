mod input;
mod trans;
mod to_dfa;
mod charset;
mod minimize_dfa;
mod combine_dfa;

use input::replace_reg;

use crate::combine_dfa::{combine, print_standard_dfa};
use crate::input::div_reg_action;
use crate::minimize_dfa::minimize_dfa;
use crate::input::init;
use crate::trans::translate_reg;
use crate::to_dfa::{convert, print_dfa};

fn main() {
    let s = init("test.l");
    print!("{}", s[2]);
    let longtxt = replace_reg(&s[1], &div_reg_action(&s[2]));
    let mut dfas = Vec::new();
    for sent in longtxt {
        print!("-------------\n[[{}]] [[{}]]\n--------------\n", sent.reg, sent.action);
        if sent.reg.starts_with("\"") || sent.reg.starts_with(".") {continue;}
        let tmp_dfa = minimize_dfa(&convert(&translate_reg(&sent.reg)));
        dfas.push(tmp_dfa);
    }
    let final_dfa = combine(&dfas);
    print_standard_dfa(&final_dfa);
    // let dfas = vec![
    //     &convert(&translate_reg("[a-c]+"))),
    //     minimize_dfa(&convert(&translate_reg("s[1-3]*"))),
    //     minimize_dfa(&convert(&translate_reg("o[z9]?")))
    // ];
    // let final_dfa = combine(&dfas);
    // print_standard_dfa(&final_dfa);
}
