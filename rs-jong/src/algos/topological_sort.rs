use std::collections::{HashMap, HashSet};

pub fn backwards<T>(graph: &HashMap<T, Vec<T>>) -> HashMap<T, HashSet<&T>>
where T: Eq + std::hash::Hash + Clone, {
    let mut back: HashMap<T, HashSet<&T>> = HashMap::new();
    for (key_from, value) in graph.into_iter() {
        for key_to in value {
            if !back.contains_key(&key_to) {
                let mut hs = HashSet::new();
                hs.insert(key_from);
                back.insert(key_to.clone(), hs);
            } else {
                back.get_mut(key_to).unwrap().insert(key_from);
            }
        }
    }
    /* let mut res: HashMap<T, Vec<&T>> = HashMap::new();
    for (k, v) in back.into_iter() {
        res.insert(k, v.into_iter().collect());
    }
    res */
    back
}

use std::error::Error;
#[derive(Debug)]
pub enum FailSort {
    Cycle,
    Construct,
}

pub fn topological_sort<'a, T>(next_nodes: &'a HashMap<T, Vec<T>>) -> Result<Vec<T>, FailSort>
where T: Eq + std::hash::Hash + Clone, {
    let res;
    {
        let graph: Graph<&'a T, ()> = into_graph(next_nodes).ok_or(FailSort::Construct)?;
        let sorted = petgraph::algo::toposort(&graph, None).or(Err(FailSort::Cycle))?;
        res = sorted
            .iter()
            .map(|x| (*(graph.node_weight(*x).unwrap())).clone())
            .collect();
    }
    Ok(res)
}
use petgraph::algo;
use petgraph::dot::Dot;
use petgraph::Graph;
pub fn into_graph<'a, T>(next_nodes: &'a HashMap<T, Vec<T>>) -> Option<Graph<&'a T, ()>>
where T: Eq + std::hash::Hash + Clone, {
    let mut graph = petgraph::Graph::new();
    let mut node = HashMap::new();
    for k in next_nodes.keys().into_iter() {
        node.insert(k, graph.add_node(k));
    }
    for (k, vs) in next_nodes.into_iter() {
        for v in vs {
            graph.add_edge(node.get(k)?.clone(), node.get(v)?.clone(), ());
        }
    }
    Some(graph)
}
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_topo() {}
}
