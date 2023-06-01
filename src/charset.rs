/*
字符集规定：
字符集中包含全部的字母、数字、运算符、括号，以及换行符、回车符、制表符三个特殊字符
但是由于某些符号在正则表达式里有特殊功能，所以用别的数字替代
具体来讲，加号用1替代，乘号用2替代，( ) [ ] { }分别是345678
*/
pub fn is_letter(ch: char) -> bool {
    (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || (ch >= '0' && ch <= '9') ||
    (ch == '\n') || (ch == '\t') || (ch == '\r') || (ch == ' ') || (ch == '.') || (ch == ';') ||
    (ch == 1 as char) || (ch == '-') || (ch == 2 as char) || (ch == '/') || (ch == '=') || (ch == ',') ||
    (ch == 3 as char) || (ch == 4 as char) || (ch == 5 as char) || (ch == 6 as char) || (ch == 7 as char) ||
    (ch == 8 as char)
}

pub fn get_charset() -> Vec<char> {
    let mut ans = Vec::new();
    for ch in 0u8..=128u8 {
        let c = ch as char;
        if is_letter(c) {ans.push(c);}
    }
    return ans
}

pub fn fix_str(ori: &str) -> String {
    let mut res = String::from("");
    for ch in ori.as_bytes() {
        match ch {
            b'+' => {res.push(1 as char);}
            b'*' => {res.push(2 as char);}
            b'(' => {res.push(3 as char);}
            b')' => {res.push(4 as char);}
            b'[' => {res.push(5 as char);}
            b']' => {res.push(6 as char);}
            b'{' => {res.push(7 as char);}
            b'}' => {res.push(8 as char);}
            _ => {res.push(*ch as char);}
        }
    }
    return res;
}

pub fn to_origin_ch(ch: char) -> char {
    match (ch as u8) {
        1 => {return '+';}
        2 => {return '*';}
        3 => {return '(';}
        4 => {return ')';}
        5 => {return '[';}
        6 => {return ']';}
        7 => {return '{';}
        8 => {return '}';}
        _ => {return ch;}
    }
}
