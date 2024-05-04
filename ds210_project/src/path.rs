
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
    pub fn get_middle_edges(&mut self, start_node:usize,end_node:usize)->Vec<(usize,usize)>{
        let mut queue:VecDeque<usize>=VecDeque::new();
        queue.push_back(start_node);
        let new_edges_one=self.graph.sort_edges();
        while let Some(existing_node)=queue.pop_front(){
            if existing_node==end_node{
                 break;
             }
             let index: i32=self.graph.get_sorted_index(existing_node as i32);
             for &next_node in new_edges_one[index as usize].iter(){
                //let next_node_index=self.graph.get_sorted_index(next_node as i32);
                if !self.visited_nodes.contains_key(&(next_node as usize)){
                     self.visited_nodes.insert(next_node as usize, existing_node);
                     queue.push_back(next_node);
                 }
             }
             break;
         }
         let mut path=Vec::new();
         let mut current=end_node;
         while let Some(&prev_node)=self.visited_nodes.get(&current){
             path.push((prev_node,current));
             current=prev_node;
         }

         path.reverse();

         path

    }
}