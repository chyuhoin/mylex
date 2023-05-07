use std::{collections::{BTreeMap, VecDeque, HashMap}};
use crate::{charset::get_charset, to_dfa::add_edge, to_dfa::get_all_vertex};
use crate::to_dfa::{Graph, Dfa};

fn get_one_step(dfa: &Graph<i32, char>, state: &Vec<i32>, ch: char) -> Vec<i32> {
    let mut ans = Vec::new();

    //获得state出发经过一次ch边到达的点集，如果到不了任何点，就算一个-1
    for u in state {
        let edges = match dfa.get(u) {
            Some(value) => value,
            None => {ans.push(-1); continue;}
        };
        let mut can_access = false;
        for edge in edges {
            let (v, c) = *edge;
            if c != ch {continue;}
            ans.push(v);
            can_access = true;
        }
        if !can_access {ans.push(-1);}
    }
    return ans;
}

//判断v1里面的值是不是全部为-1
fn is_all_none(v1: &Vec<i32>) -> bool {
    for i in v1 {
        if *i != -1 {return false;}
    }
    return true;
}

//检查state里面的点分别是属于哪个集合里面的
fn get_which_set(state :&Vec<i32>, mapping: &BTreeMap<i32, Vec<i32>>) -> Vec<i32> {
    let mut ans = Vec::new();
    for point in state {
        if *point == -1 {
            ans.push(-1);
            continue;
        }
        for (k, v) in mapping {
            if v.contains(point) {
                ans.push(*k);
                break;
            }
        }
    }
    return ans;
}

//把v1里的元素按照v2里的值进行分裂，获得一大堆Vec
fn split(v1: &Vec<i32>, v2: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = Vec::new();
    let mut vis = HashMap::new();
    for i in 0..v1.len() {
        let (k, v) = (v1[i], v2[i]);
        if let Some(index) = vis.get(&v) {
            let ind = *index as i32;
            ans[ind as usize].push(k);
        } else {
            ans.push(vec![k]);
            vis.insert(v, ans.len() - 1);
        }
    }
    return ans;
}

/*
DFA最小化，使用一种分裂集合的方法
一开始有两个集合start_state和end_state，分别是非终点集合和终点集合
然后依次考虑每个集合S，如果S中的点经过字符x之后，去向了不同的集合（或者有些点能走字符x，有些点不能走字符x），那么说明集合S应该被分裂
把S分裂S1、S2、...、Sn之后要保证，Si中所有点经过字符x都到达了同一个点
 */
pub fn minimize_dfa(dfa: &Dfa<i32, char>) -> Dfa<i32, char> {
    let mut min_dfa = Graph::new();
    let mut start_state = Vec::new();
    let mut end_state = Vec::new();
    let mut mapping = BTreeMap::new();
    let mut min_start = 1;
    let mut min_ends = Vec::new();

    for k in &dfa.points {
        if dfa.ends.contains(k) {end_state.push(*k)}
        else {start_state.push(*k)}
    }
    mapping.insert(1, start_state);
    mapping.insert(2, end_state);

    //还是使用类似BFS的思路，维护一个队列，从队首取出一个集合，看能不能分裂，能分裂就把分裂之后的集合们入队
    let mut q = VecDeque::new();
    q.push_back(1); q.push_back(2);
    let mut tot = 2;
    while !q.is_empty() {
        //取出队首元素并查到对应的集合
        let now = match q.front() {
            Some(x) => *x,
            None => {break;}
        };
        let now_state = match mapping.get(&now) {
            Some(x) => x,
            None => {println!("Something Wrong!!"); break;}
        };

        //枚举字符集看能不能分裂
        for ch in get_charset() {
            let new_state = get_one_step(&dfa.graph, now_state, ch);
            if is_all_none(&new_state) {continue;}
            let belonging = get_which_set(&new_state, &mapping);//belonging表示new_state里面每个元素所属的集合
            let splited = split(&now_state, &belonging);//按照所属集合进行分裂
            if splited.len() != 1 {//如果分裂出来不止一个，证明是可分裂的
                for small_set in splited {
                    //把分裂之后的集合们依次编号并入队
                    tot += 1;
                    q.push_back(tot);
                    mapping.insert(tot, small_set);
                }
                mapping.remove(&now);//当前集合已经被分裂，所以要删掉
                break;
            }
        }
        q.pop_front();
    }

    //经过上面的处理，集合已经被正确分裂，但是集合们的编号不一定是从1开始的，需要重新做一遍映射
    let mut new_map = BTreeMap::new();
    let mut points = 0;
    for (_, v) in &mapping {
        points = points + 1;
        let new_v = v.clone();
        //顺便求一下新的start和ends
        if v.contains(&1) {
            min_start = points;
        } else if v.into_iter().all(|x| dfa.ends.contains(&x)) {
            min_ends.push(points);
        }
        new_map.insert(points, new_v);
    }

    //根据划分进行连边
    for (now, state) in &new_map {
        for ch in get_charset() {
            let new_state = get_one_step(&dfa.graph, &state, ch);
            if is_all_none(&new_state) {continue;}
            let belonging = get_which_set(&new_state, &new_map);
            add_edge(&mut min_dfa, *now, belonging[0], ch);
        }
    }

    return Dfa { graph: min_dfa.clone(), points: get_all_vertex(&min_dfa), start: min_start, ends: min_ends }
}