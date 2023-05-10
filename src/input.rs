use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use std::fs::File;

#[derive(Clone)]
pub struct Sentence {
    pub reg: String,
    pub action: String,
}

/*
从minic.l文件里读入数据，并且拆分到字符串数组里返回回去。
数组的第一项是原封不动的代码，第二项是正规定义，第三项是动作，第四项也是原封不动的代码
*/
pub fn init(path: &str) -> [String; 4] {
    let path = Path::new(path);

    let mut file = match File::open(&path) {
        Err(_) => panic!("unable to open file"),
        Ok(r) => r
    };

    let mut lex_content = String::new();

    if let Err(_) = file.read_to_string(&mut lex_content) {
        panic!("unable to read file");
    }

    let mut divided_lex = std::array::from_fn(|_| String::from(""));
    let mut source_start = 0usize;
    let mut source_end = 0usize;
    let mut division1 = 0usize;
    let mut division2 = 0usize;

    for i in 1..lex_content.len() {
        let substr = &lex_content[i - 1..i + 1];
        match substr {
            "%{" => source_start = i + 1,
            "%}" => source_end = i - 1,
            "%%" => {
                if division1 == 0 {division1 = i + 1;}
                else {division2 = i + 1;}
            },
            _ => {}
        }
    }

    divided_lex[0] = lex_content[source_start..source_end].to_string();
    divided_lex[1] = lex_content[source_end + 2..division1 - 2].to_string();
    divided_lex[2] = lex_content[division1..division2 - 2].to_string();
    divided_lex[3] = lex_content[division2..].to_string();

    return divided_lex;
}

/*
在动作表达式中划分出“正则表达式”和“语义动作”两部分
这两部分使用空格来进行区分
这种规定下，如果正则表达式中如果出现空格事情会比较难办，所以我规定正则表达式中不出现空格，应该使用\s来代替
*/
pub fn div_reg_action(longtext: &str) -> Vec<Sentence> {
    let mut sentences = Vec::new();
    let lines = longtext.split("\n");
    let mut in_braces = false;
    let mut current = Sentence{reg: String::from(""), action: String::from("")};

    for line in lines {
        if line.len() == 0 {continue;}
        if in_braces {
            current.action.push_str(line);
            if line.contains("}") {
                in_braces = false;
            }
        } else {
            let mut in_reg = true;
            for ch in line.as_bytes() {
                if (*ch == b' ' || *ch == 9) && in_reg {
                    in_reg = false; continue;
                }
                if in_reg {current.reg.push(*ch as char);}
                else {
                    if *ch == b'{' {in_braces = true;}
                    current.action.push(*ch as char);
                }
            }
        }

        if !in_braces {
            current.reg = String::from(current.reg.trim());
            current.action = String::from(current.action.trim());
            if current.reg.len() != 0 {sentences.push(current);}
            current = Sentence{reg: String::from(""), action: String::from("")};
        }
        
    }
    return sentences;
}

pub fn replace_reg(deftxt: &str, sents: &Vec<Sentence>) -> Vec<Sentence> {
    let lines: Vec<&str> = deftxt.lines().collect();
    let mut result = Vec::new();
    let mut mapping = HashMap::new();

    for line in lines {
        if line.len() == 0 {continue;}
        let words: Vec<&str> = line.split_ascii_whitespace().collect();
        let (define, reg) = (words[0], words[1]);
        mapping.insert(format!("{{{}}}", define), reg);
    }

    let mut defines: Vec<(&String, String)> = Vec::new();

    for (def, reg) in &mapping {
        let mut new_reg = reg.clone().to_string();
        for (k, v) in &defines {
            new_reg = new_reg.replace(&k[..], v);
        }
        defines.push((def, new_reg));
    }

    for sent in sents {
        let (mut reg, act) = (sent.reg.clone(), sent.action.clone());
        for (k, v) in &defines {
            reg = reg.replace(&k[..], v);
        }
        result.push(Sentence { reg, action: act });
    }

    return result;
}
