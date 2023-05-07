use crate::to_dfa::{Graph, Dfa, get_all_vertex, add_edge, get_closure, get_move_closure, equal_vector};
use crate::charset::get_charset;
use std::cmp::{Ord, PartialOrd, Ordering, min};

/*
这是一个标准化的Vertex节点，用在最终合并之后的DFA里面
id表示节点的编号，end表示节点是哪个正则表达式的结束位置
如果end为-1，说明这个点不是任何一个正则表达式的结束位置
如果有多个正则表达式都可以在这个点结束，选择出现最早(编号最小)的那个表达式
 */
#[derive(Eq, PartialEq, Clone, Copy)]
pub struct Vertex {
    id: i32,
    end: i32
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

/*
输出一个Vertex标准化的DFA
 */
pub fn print_standard_dfa(dfa :&Dfa<Vertex, char>) {
    for (u, edges) in &dfa.graph {
        for e in edges {
            println!("{} {} {}", u.id, e.0.id, e.1);
        }
    }
    for u in &dfa.points {
        print!("[id: {}, end: {}]", u.id, u.end)
    }
}

/*
使用原先dfa上的点构造出一个标准的Vertex节点。
dfa是原图，v是原图上点的编号，i表示这是第i个图，offset表示原图到综合图的id偏移量
 */
fn construct_vertex(dfa: &Dfa<i32, char>, v: i32, i: usize, offset: i32) -> Vertex {
    return Vertex { id: v + offset, end: if dfa.ends.contains(&v) {i as i32} else {0} };
}

fn get_end(state: &Vec<Vertex>) -> i32 {
    let mut res = 100000;
    for v in state {
        if v.end != -1 {
            res = min(res, v.end);
        }
    }
    if res == 100000 {return -1;}
    return res;
}

/*
将合并出的NFA转为DFA，算法和之前的NFA转DFA一模一样（其实是复制的代码）
由于同样使用的是子集化算法，所以该函数里面调用的to_dfa模块里面的两个求闭包的函数
 */
fn to_final_dfa(nfa: &Graph<Vertex, char>, bgn: Vertex) -> Dfa<Vertex, char> {
    let mut dfa: Graph<Vertex, char> = Graph::new();
    let start_state = get_closure(nfa, bgn);
    let mut states = Vec::new();
    let mut vis = Vec::new();
    let mut tag: Vec<Vertex> = Vec::new();
    let mut points = 1;
    states.push(start_state);
    vis.push(false); tag.push(Vertex { id: points, end: -1 });

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
                states.push(new_state.clone());
                vis.push(false);
                points = points + 1;
                now = points;
                tag.push(Vertex { id: now, end: get_end(&new_state) });
            }

            //连边
            add_edge(&mut dfa, tag[state as usize], tag[now as usize - 1], ch);
        }
    }

    return Dfa { graph: dfa.clone(), points: get_all_vertex(&dfa), start: Vertex { id: points, end: -1 }, ends: Vec::new() };
}

/*
使用暴力方法合并多个DFA
原理是创建一个新的起点，然后从这个起点向所有DFA的起点连一条空串边，构造出一个NFA
然后对这个NFA做确定化，转成DFA即可
 */
pub fn combine(dfas: &Vec<Dfa<i32, char>>) -> Dfa<Vertex, char> {
    let mut res_graph = Graph::new();
    let start = Vertex{id: 1, end: -1};
    let mut offset = 1;
    for (i, dfa) in dfas.iter().enumerate() {
        let dfa_start = construct_vertex(dfa, dfa.start, i, offset);
        add_edge(&mut res_graph, start, dfa_start, '~');

        //枚举dfa里面所有的边，把它连到最终的图里面
        for u in &dfa.points {
            let from = construct_vertex(dfa, *u, i, offset);
            for e in dfa.graph.get(&u) {
                for v in e {
                    let to = construct_vertex(dfa, v.0, i, offset);
                    add_edge(&mut res_graph, from, to, v.1);
                }
            }
        }
        offset += dfa.points.len() as i32;
    }
    return to_final_dfa(&res_graph, start);
}