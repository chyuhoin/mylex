use crate::to_dfa::{Graph, Dfa, get_all_vertex};

pub struct Vertex {
    id: i32,
    end: i32
}

pub fn combine(dfas: &Vec<Dfa<i32, char>>) -> Dfa<i32, char> {
    let res_graph = Graph::new();
    for (i, dfa) in dfas.iter().enumerate() {
        let mut offset = 1;
    }
    return Dfa { graph: res_graph.clone(), points: get_all_vertex(&res_graph), start: (), ends: () }
}