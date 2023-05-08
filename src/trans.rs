//处理转义字符
pub fn translate_ch(reg: &str) -> String {
    let mut ans = String::from("");
    let mut conv = false;

    for c in reg.to_string().as_bytes() {
        let ch = *c as char;
        match ch {
            '\\' => {conv = true;}
            _ => {
                if conv {
                    match ch {
                        't' => {ans.push('\t')}
                        'n' => {ans.push('\n')}
                        'r' => {ans.push('\r')}
                        's' => {ans.push(' ')}
                        '.' => {ans.push('.')}
                        _ => {panic!("Wrong escape character!")}
                    }
                    conv = false
                } else {
                    ans.push(ch);
                }
            }
        }
    }
    return ans;
}

pub fn translate_reg(reg: &str) -> String {
    let mut is_or = false;
    let mut con_start = '\0';
    let mut pre = '\0';
    let mut ans = String::from("");

    for c in translate_ch(reg).as_bytes() {
        let character = *c as char;
        match character {
            '[' => {
                is_or = true;
                ans.push('(');
            },
            ']' => {
                is_or = false;
                ans.pop(); //把最后push进去的|符号去掉
                ans.push(')');
            },
            '-' => {
                if (!is_or) || con_start != '\0'  {panic!("syntax error");}
                con_start = pre; //con_start表示的是-符号的上一个符号，同时con_start为不为0也表示目前有没有遇到-符号
            },
            _ => {
                if is_or {
                    if con_start != '\0' { //有连接符-的情况，走循环
                        ans.pop(); ans.pop();
                        for tmp in con_start..=character {
                            ans.push(tmp);
                            ans.push('|');
                        }
                        con_start = '\0';
                    } else {
                        ans.push(character);
                        ans.push('|'); //每添加一个字符都会加一个|符号，最后会多一个|符号出来
                    }
                } else {ans.push(character);}
            }
        }
        pre = character;
    }
    return String::from(ans);
}