/*
字符集规定：
字符集中包含全部大小写字母和阿拉伯数字，以及空格\n\t\r。
*/
pub fn is_letter(ch: char) -> bool {
    (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || (ch >= '0' && ch <= '9') ||
    (ch == '\n') || (ch == '\t') || (ch == '\r')
}

pub fn get_charset() -> Vec<char> {
    let mut ans = Vec::new();
    for ch in 0u8..=200u8 {
        let c = ch as char;
        if is_letter(c) {ans.push(c);}
    }
    return ans
}

