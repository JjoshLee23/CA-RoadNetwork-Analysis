
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
        let mut distance:Vec<Option<u32>>=vec![None;self.graph.nodes.len() as usize];
        let start_index=self.graph.get_sorted_index(start_node as i32);
        if start_index==-1{
            distance.push(None);
            return distance
        }
        distance[start_index as usize] = Some(0);
        self.queue.push_back(start_node);
        let new_edges_one=self.graph.sort_edges();
        while let Some(v) = self.queue.pop_front() { 
            let index=self.graph.get_sorted_index(v as i32);
            for u in new_edges_one[index as usize].iter() {
                let new_index=self.graph.get_sorted_index(*u as i32);
                if let None = distance[new_index as usize] { 
                    distance[new_index as usize] = Some(distance[index as usize].unwrap() + 1);
                    self.queue.push_back(*u as usize);
                }
            }
        }
        distance

    }
    pub fn maximum_distance_for_node(&mut self, node:usize)->u32{
        let distances=self.calculate_distance(node);
        let mut max=0;
        for i in distances.iter(){
            if *i!=None{
                if i.unwrap()>max{
                    max=i.unwrap()
                }
            }
        }
        max
    }
}