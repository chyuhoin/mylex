use std::{collections::{BTreeMap, HashMap, BTreeSet}};
use crate::charset::get_charset;
use crate::charset::is_letter;

//使用BTreeMap作为存图的数据结构，V表示图中的点，借助Vec可以遍历所有从点V出发的边
pub type Graph<V, E> = BTreeMap<V, Vec<(V, E)>>;

pub struct Dfa<V, E> {
    pub graph: Graph<V, E>,
    pub points: BTreeSet<V>,
    pub start: V,
    pub ends: Vec<V>
}

//在u和v之间插入一条权值为c的边
pub fn add_edge<V: Ord + Copy, E: Ord>(graph: &mut Graph<V, E>, u: V, v: V, c: E) {
    graph.entry(u).or_insert_with(Vec::new).push((v, c));
}

//获得dfa里面的所有点集
pub fn get_all_vertex<V: Ord + Copy>(dfa: &Graph<V, char>) -> BTreeSet<V>{
    let mut set = BTreeSet::new();
    for (v, edges) in dfa {
        set.insert(*v);
        for (p, _) in edges {
            set.insert(*p);
        }
    }
    return set;
}

/*
最开始先要给正规表达式加连接符，因为本来正规表达式的连接符是隐含的
比如abc其实应该是a 连接 b 连接 c
本系统用`符号来表示连接符
有几种情况要加连接符：*和字母之间，*和(之间等等，具体看代码吧我懒得说了
*/
fn insert_linker(origin: &str) -> String {
    let mut ans = String::new();
    let mut pre: char;
    let mut now = '#';
    for i in origin.chars() {
        pre = now;
        now = i;
        if (pre == '*' || pre == '+' || pre == '?') && is_letter(now) {ans.push('`');}
        if (pre == '*' || pre == '+' || pre == '?') && now == '(' {ans.push('`');}
        if pre == ')' && is_letter(now) {ans.push('`');}
        if pre == ')' && now == '(' {ans.push('`');}
        if is_letter(pre) && is_letter(now) {ans.push('`');}
        if is_letter(pre) && now == '(' {ans.push('`');}
        ans.push(now);
    }
    return ans;
}

/*
中缀正规表达式转后缀正规表达式，这个操作可以去掉所有的括号，方便我们处理
具体流程和大一那会学的完全一样，开个栈，然后扫描的时候按照优先级弹栈就行了
*/
fn infix_to_suffix(origin: &str) -> String {
    let mut stk = Vec::new();
    let origin = &insert_linker(&origin);
    let mut ans = String::new();
    stk.push('$');//用一个$来当做开始符
    for ch in origin.chars() {
        match ch {
            '(' => {stk.push(ch);}
            ')' => { //遇到后括号就一直弹栈，直到遇到前括号
                let mut top = match stk.last() {
                    Some(value) => *value,
                    _ => break,
                };
                while top != '(' {
                    ans.push(top);
                    stk.pop();
                    top =  match stk.last() {
                        Some(value) => *value,
                        _ => break,
                    };
                }
                stk.pop();
            }
            '|' => { //二目运算符中，或符号的优先级高于连接符
                let mut top = match stk.last() {
                    Some(value) => *value,
                    _ => break,
                };
                while top != '(' && top != '$' {
                    ans.push(top);
                    stk.pop();
                    top =  match stk.last() {
                        Some(value) => *value,
                        _ => break,
                    };
                }
                stk.push(ch);
            }
            '`' => {
                let mut top = match stk.last() {
                    Some(value) => *value,
                    _ => break,
                };
                while top != '(' && top != '$' && top != '|' {
                    ans.push(top);
                    stk.pop();
                    top =  match stk.last() {
                        Some(value) => *value,
                        _ => break,
                    };
                }
                stk.push(ch);
            }
            _ => {ans.push(ch);}//遇到普通字符和单目运算符都可以直接压栈
        }
    }
    while stk[stk.len() - 1] != '$' {
        ans.push(stk[stk.len() - 1]);
        stk.pop();
    }
    return ans;
}

/*
将已经转换为后缀的正规表达式转为NFA
基本原理：栈里的每一个元素都是一个独立的NFA，在各个NFA之间连边形成最终的NFA
扫描整个后缀表达式：
遇到字母就构造一个只有两个点一条边的NFA压进栈里，比如遇到字母a，就把 1--[a]-->2压栈
遇到单目运算符就把栈顶的NFA取出来
如果是*，就在这个NFA的起点往终点连一条空串边，再从终点往起点连空串边
如果是+，就只从终点往起点连一条空串边
如果是?，就只从起点往终点连一条空串边
遇到二目运算符就从栈里面弹出两个NFA，分别记为a和b
如果是连接符，就用一条空串边把a和b连起来
如果是|符号，就新开一个起点和一个终点，然后新起点向ab起点各连一条边，ab终点向新终点各连一条边
*/
fn build_nfa(nfa: &mut Graph<i32, char>, reg: &str, bgn_p: &mut i32, end_p: &mut i32) {
    let mut stk: Vec<(i32, i32)> = Vec::new();
    *bgn_p = (*end_p) + 1i32;
    for ch in reg.chars() {
        match ch {
            '*' => {
                let (bg, ed) = stk[stk.len() - 1];
                add_edge(nfa, bg, ed, '~');
                add_edge(nfa, ed, bg, '~');
            }
            '+' => {
                let (bg, ed) = stk[stk.len() - 1];
                add_edge(nfa, ed, bg, '~');
            }
            '?' => {
                let (bg, ed) = stk[stk.len() - 1];
                add_edge(nfa, bg, ed, '~');
            }
            '|' => {
                let (bg1, ed1) = stk[stk.len() - 1];
                stk.pop();
                let (bg2, ed2) = stk[stk.len() - 1];
                stk.pop();
                let bg = *end_p + 1;
                let ed = *end_p + 2;
                add_edge(nfa, bg, bg1, '~');
                add_edge(nfa, bg, bg2, '~');
                add_edge(nfa, ed1, ed, '~');
                add_edge(nfa, ed2, ed, '~');
                *end_p += 2;
                if *bgn_p == bg1 || *bgn_p == bg2 {*bgn_p = bg;}
                stk.push((bg, ed));
            }
            '`' => {
                let (bg1, ed1) = stk[stk.len() - 1];
                stk.pop();
                let (bg2, ed2) = stk[stk.len() - 1];
                stk.pop();
                add_edge(nfa, ed2, bg1, '~');
                stk.push((bg2, ed1));
            }
            _ => {
                let bg = *end_p + 1;
                let ed = *end_p + 2;
                *end_p += 2;
                add_edge(nfa, bg, ed, ch);
                stk.push((bg, ed));
            }
        }
    }
}

fn print_nfa(nfa :&Graph<i32, char>) {
    for (u, edges) in nfa {
        for e in edges {
            println!("{} ---{}---> {}", u, e.1, e.0);
        }
    }
}

/*
获得从x点出发的闭包
方法是广度优先搜索，把NFA看作有向图（好像不用看作，NFA本来就是有向图），然后以x为起点做BFS
BFS的过程中只允许经过空串边
BFS的过程中到达的所有点组成的集合就是从x出发的闭包
*/
fn get_closure(nfa :&Graph<i32, char>, x: i32) -> Vec<i32> {
    let mut q = Vec::new(); //用来做BFS的队列
    let (mut hen, mut tai) = (0, 0); //队列的头尾
    let mut ans = Vec::new(); //闭包集合
    let mut vis = HashMap::new();

    //BFS的具体过程
    q.push(x); vis.insert(x, 1); ans.push(x);
    while hen <= tai {
        let u = q[hen];
        hen = hen + 1;
        let edges = match nfa.get(&u) {
            Some(value) => value,
            None => {continue;}
        };
        for edge in edges {
            let (v, c) = *edge;
            if c != '~' {continue;}
            match vis.get(&v) {
                Some(_) => {continue;},
                None => {},
            }
            q.push(v);
            ans.push(v);
            vis.insert(v, 1);
            tai = tai + 1;
        }
    }

    //为了方便比较两个闭包是否相同，还要排个序
    ans.sort();
    return ans;
}

/*
获得从状态state出发，经过字符ch之后会到达的新状态
方法是先看从状态state里的所有点出发，经过一次字符ch的边，获得一个点集
然后从这个点集里的每个点出发，做和上面一样的BFS
最终得到一个闭包，这个闭包就是新状态
*/
fn get_move_closure(nfa: &Graph<i32, char>, state: &Vec<i32>, ch: char) -> Vec<i32> {
    let mut q = Vec::new();
    let (mut hen, mut tai) = (0, 0);
    let mut ans = Vec::new();
    let mut vis = HashMap::new();

    //第一步：获得x出发经过一次ch边到达的点集
    for u in state {
        let edges = match nfa.get(u) {
            Some(value) => value,
            None => {continue;}
        };
        for edge in edges {
            let (v, c) = *edge;
            if c != ch {continue;}
            q.push(v);
            ans.push(v);
            vis.insert(v, 1);
            tai = tai + 1;
        }
    }

    //第二步：从点集出发，只经过空串边，获得整个闭包
    while hen < tai {
        let u = q[hen];
        hen = hen + 1;
        let edges = match nfa.get(&u) {
            Some(value) => value,
            None => {continue;}
        };
        for edge in edges {
            let (v, c) = *edge;
            if c != '~' {continue;}
            match vis.get(&v) {
                Some(_) => {continue;},
                None => {},
            }
            q.push(v);
            ans.push(v);
            vis.insert(v, 1);
            tai = tai + 1;
        }
    }

    //为了方便比较两个闭包是否相同，还要排个序
    ans.sort();
    return ans;
}

pub fn equal_vector(v1: &Vec<i32>, v2: &Vec<i32>) -> bool {
    if v1.len() != v2.len() {return false;}
    for i in 0..v1.len() {
        if v1[i] != v2[i] {
            return false;
        }
    }
    return true;
}

/*
NFA转DFA
使用子集化算法，NFA里面的每个闭包都可能是DFA里面的一个状态
DFA的起点就是NFA的起点所在的闭包，编号设为1
接下来的操作有点类似最短路dijsktra算法：
1.每次寻找一个还没扩展过的状态state
2.将state依次用每一个字符进行扩展，扩展到一个新的状态new_state
3.看new_state是不是从来没出现过的状态，如果是的话就给他映射到DFA里面作为一个新的节点
4.把state闭包对应的DFA节点和new_state闭包对应的DFA节点连边
等到所有点都扩展过了，算法就结束。
不过这个算法的时间复杂度有点感人哈~毕竟最坏的情况下，时间复杂度相当于子集个数，是指数级的
*/
pub fn nfa_to_dfa(nfa: &Graph<i32, char>,bgn: i32, end: i32, ends: &mut Vec<i32>) -> Graph<i32, char> {
    let mut dfa = Graph::new();
    let start_state = get_closure(nfa, bgn);
    let mut states = Vec::new();
    let mut vis = Vec::new();
    let mut tag = Vec::new();
    let mut points = 1;
    states.push(start_state);
    vis.push(false); tag.push(points);

    loop {
        //第一步：寻找一个还没出现过的状态state
        let mut state: i32 = -1;
        for i in 0..states.len() {
            if vis[i] == false {
                state = i as i32;
                break;
            }
        }
        if state == -1 {break;}
        vis[state as usize] = true;

        //第二步：枚举字符，对state进行扩展
        for ch in get_charset() {
            let new_state = get_move_closure(nfa, &states[state as usize], ch);
            if new_state.len() == 0 {continue;}

            //判断一下new_state是不是没出现过的状态
            let mut now: i32 = -1;
            for i in 0..states.len() {
                if equal_vector(&states[i], &new_state) {
                    now = i as i32 + 1; //DFA的点集是从1开始，但是Vec的下标是从0开始，所以要+1
                    break;
                }
            }

            //如果new_state确实是没出现过的状态，那么就在DFA里面新创一个状态给它
            if now == -1 {
                states.push(new_state);
                vis.push(false);
                points = points + 1;
                now = points;
                tag.push(now);
                for poi in &states[now as usize - 1] {
                    if *poi == end {
                        ends.push(now);
                        break;
                    }
                }
            }

            //连边
            add_edge(&mut dfa, tag[state as usize], tag[now as usize - 1], ch);
        }
    }

    return dfa;
}

pub fn print_dfa(dfa :&Graph<i32, char>) {
    for (u, edges) in dfa {
        for e in edges {
            println!("{} {} {}", u, e.0, e.1);
        }
    }
}

pub fn convert(reg: &str) -> Dfa<i32, char> {
    let mut nfa: Graph<i32, char> = Graph::new();
    let mut bgn = 0;
    let mut end = 0;

    let suffix = infix_to_suffix(&reg);
    println!("{}", suffix);
    
    build_nfa(&mut nfa, &suffix, &mut bgn, &mut end);
    print_nfa(&nfa);
    println!("bgn = {}  end = {}", bgn, end);

    let mut ends = Vec::new();
    let dfa = nfa_to_dfa(&nfa, bgn, end, &mut ends);
    print_dfa(&dfa);
    print!("{:?}", ends);
    return Dfa { graph: dfa.clone(), points: get_all_vertex(&dfa), start: 1, ends }
}
