mod input;
mod trans;
mod to_dfa;
mod charset;
mod minimize_dfa;
mod combine_dfa;

use input::replace_reg;

use crate::input::div_reg_action;
use crate::minimize_dfa::minimize_dfa;
use crate::input::init;
use crate::trans::translate_reg;
use crate::to_dfa::{convert, print_dfa};

fn main() {
    // let s = init("../minic.l");
    // let longtxt = replace_reg(&s[1], &div_reg_action(&s[2]));
    // for sent in longtxt {
    //     print!("-------------\n[[{}]] [[{}]]\n--------------\n", sent.reg, sent.action);
    // }
    let s = &translate_reg("[ac]+u[0-9]*");
    println!("{}", translate_reg(s));
    let dfa = convert(s);
    let min_dfa = minimize_dfa(&dfa);
    print_dfa(&min_dfa.graph);
    println!("{} {:?}", min_dfa.start, min_dfa.ends);
}
