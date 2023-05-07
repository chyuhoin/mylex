use crate::to_dfa::{Graph, Dfa, get_all_vertex, add_edge, nfa_to_dfa};
use std::cmp::{Ord, PartialOrd, Ordering};

/*
这是一个标准化的Vertex节点，用在最终合并之后的DFA里面
id表示节点的编号，end表示节点是哪个正则表达式的结束位置
如果end为0，说明这个点不是任何一个正则表达式的结束位置
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
使用原先dfa上的点构造出一个标准的Vertex节点。
dfa是原图，v是原图上点的编号，i表示这是第i个图，offset表示原图到综合图的id偏移量
 */
fn construct_vertex(dfa: &Dfa<i32, char>, v: i32, i: usize, offset: i32) -> Vertex {
    return Vertex { id: v + offset, end: if dfa.ends.contains(&v) {i as i32} else {0} };
}

pub fn combine(dfas: &Vec<Dfa<i32, char>>) -> Dfa<Vertex, char> {
    let mut res_graph = Graph::new();
    let start = Vertex{id: 1, end: 0};
    let ends = Vec::new();
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
    return Dfa { graph: res_graph.clone(), points: get_all_vertex(&res_graph), start, ends }
}