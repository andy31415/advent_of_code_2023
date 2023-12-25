use std::collections::{HashSet, VecDeque};

use bimap::BiMap;
use itertools::Itertools;
use petgraph::{
    algo::{connected_components, min_spanning_tree, has_path_connecting},
    data::Element,
    dot::{Config, Dot},
    graph::NodeIndex,
    visit::IntoEdges, adj::EdgeIndex,
};

mod parse {
    pub fn input(s: &str) -> Vec<(&str, Vec<&str>)> {
        s.split("\n")
            .map(|l| {
                let (k, v) = l.split_once(':').expect("have valid assign");
                (k, v.trim().split(" ").map(|s| s.trim()).collect())
            })
            .collect()
    }
}

#[derive(Debug, Default)]
struct Input<'a> {
    graph: petgraph::graph::UnGraph<&'a str, ()>,
    node_map: BiMap<&'a str, NodeIndex>,
}

impl<'a> Input<'a> {
    fn from(input: &'a str) -> Self {
        let mut r = Self::default();

        for (a, v) in parse::input(input) {
            for b in v {
                r.add_edge(a, b);
            }
        }

        r
    }

    fn ensure_node(&mut self, a: &'a str) -> NodeIndex {
        match self.node_map.get_by_left(a) {
            Some(n) => *n,
            None => {
                let idx = self.graph.add_node(a);
                self.node_map.insert(a, idx);
                idx
            }
        }
    }

    fn add_edge(&mut self, a: &'a str, b: &'a str) {
        let a = self.ensure_node(a);
        let b = self.ensure_node(b);
        self.graph.add_edge(a, b, ());
    }
}

pub fn part1(input: &str) -> usize {
    let data = Input::from(input);

    eprintln!(
        "DATA with {} nodes, {} edges",
        data.graph.node_count(),
        data.graph.edge_count()
    );

    let mut g1 = data.graph.clone();

    /*
    for ix in g1.node_indices() {
        eprintln!("NODE {:?} has {} neighbours", data.node_map.get_by_right(&ix), g1.neighbors(ix).count());
    }
    println!("{:#?}", Dot::with_config(&g1, &[Config::EdgeNoLabel]));
    */
    

    let mut removed_edges = HashSet::new();

    while connected_components(&g1) == 1 {
        eprintln!("Removing ...");
        let edges = min_spanning_tree(&g1)
            .filter_map(|e| match e {
                Element::Edge {
                    source,
                    target,
                    weight,
                } => Some((NodeIndex::new(source), NodeIndex::new(target))),
                _ => None,
            })
            .collect::<Vec<_>>();

        for (a, b) in edges {
            removed_edges.insert((a,b));
            g1.remove_edge(g1.find_edge(a, b).expect("valid edge"));
        }
    }

    let choices = removed_edges.iter().filter(|(a,b)| 
        !has_path_connecting(&g1, *a, *b, None)
    ).collect::<Vec<_>>();
    
    for c in choices.iter().combinations(3) {
        g1 = data.graph.clone();
        let a = c.get(0).expect("3 items");
        let b = c.get(1).expect("3 items");
        let c = c.get(2).expect("3 items");
        
        g1.remove_edge(g1.find_edge(a.0, a.1).expect("valid edge 1"));
        g1.remove_edge(g1.find_edge(b.0, b.1).expect("valid edge 2"));
        g1.remove_edge(g1.find_edge(c.0, c.1).expect("valid edge 3"));
        

        if connected_components(&g1) == 2 {
            eprintln!("FOUND:");
            eprintln!("   {:?} - {:?}", data.node_map.get_by_right(&a.0), data.node_map.get_by_right(&a.1));
            eprintln!("   {:?} - {:?}", data.node_map.get_by_right(&b.0), data.node_map.get_by_right(&b.1));
            eprintln!("   {:?} - {:?}", data.node_map.get_by_right(&b.0), data.node_map.get_by_right(&c.1));
            break;
        }
    }

    // at this point g1 has the components ...
    let mut s1 = HashSet::new();
    let mut p = VecDeque::new();

    let start = data.node_map.iter().next().expect("has nodes").1;
    p.push_back(*start);

    while let Some(n) = p.pop_back() {
        if s1.contains(&n) {
            continue;
        }

        s1.insert(n);
        for o in g1.neighbors(n) {
            p.push_back(o);
        }
    }

    eprintln!("{} out of {}", s1.len(), data.node_map.len());
    s1.len()*(data.node_map.len() - s1.len())
}

pub fn part2(input: &str) -> usize {
    // TODO: implement
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../example.txt")), 54);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../example.txt")), 0);
    }
}
