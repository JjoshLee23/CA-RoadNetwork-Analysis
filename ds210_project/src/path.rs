
use crate::graph::Graph;

use std::collections::{HashMap, VecDeque};
#[derive(Clone,Debug)]

pub struct Path{
    pub queue:VecDeque<usize>,
    pub visited_nodes:HashMap<usize,usize>,
    pub graph:Graph,
}
impl Path{
    pub fn initialization(graph: Graph)->Self{
        Self{
            queue: VecDeque::new(),
            visited_nodes:HashMap::new(),
            graph,
        }

    }
    pub fn calculate_distance(&mut self, start_node:usize)->Vec<Option<u32>>{
        let mut numbered_nodes=vec![0;self.graph.max_node().unwrap() as usize];
        let mut corresponding_edges=vec![vec![0;self.graph.max_node().unwrap() as usize]];
        self.graph.nodes.sort();
        for &i in self.graph.nodes.iter(){
            let index=self.graph.get_index(i);
            numbered_nodes[(i-1) as usize]=i as usize;
            corresponding_edges[(i-1) as usize]=self.graph.edge[index as usize].clone();
        }
        let mut distance:Vec<Option<u32>>=vec![None;self.graph.max_node().unwrap() as usize];
        distance[start_node] = Some(0);
        self.queue.push_back(start_node);
        while let Some(v) = self.queue.pop_front() { 
            for &u in self.graph.edge[v].iter() {
                if let None = distance[u as usize] { 
                    distance[u as usize] = Some(distance[v].unwrap() + 1);
                    self.queue.push_back(u as usize);
                }
            }
        }
        distance

    }
}