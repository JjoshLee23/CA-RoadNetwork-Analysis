
use crate::graph::Graph;

use std::collections::{HashMap, VecDeque};

#[derive(Debug)]

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
    pub fn calculate_distance(& mut self, start_node:usize)->Vec<Option<u32>>{
        let mut distance:Vec<Option<u32>>=vec![None;self.graph.num_nodes() as usize];
        let start: i32 = start_node as i32;
        let index=self.graph.get_index(start);
        distance[index as usize] = Some(0);
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