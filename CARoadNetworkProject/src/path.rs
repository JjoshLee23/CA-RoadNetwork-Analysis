
use crate::graph::Graph;
use std::collections::VecDeque;
#[derive(Clone,Debug)]
pub struct Path{//creates a struct called "Path" which takes a graph instance and initializes a VecDeque
    pub queue:VecDeque<usize>,
    pub graph:Graph,
}
impl Path{
    pub fn initialization(graph: Graph)->Self{
        Self{
            queue: VecDeque::new(),
            graph,
        }
    }
    pub fn calculate_distance(&mut self, start_node:usize)->Vec<Option<u32>>{//calculate distance function which calcuates the shortest distance using bfs
        let mut distance:Vec<Option<u32>>=vec![None;self.graph.nodes.len() as usize];
        //distance variable used to store all of my distances as option enum
        let start_index=self.graph.get_sorted_index(start_node as i32);
        if start_index==-1{//checks if the starting node exists
            distance.push(None);
            return distance
        }
        distance[start_index as usize] = Some(0);//sets the distance from the node to itself as 0
        self.queue.push_back(start_node);//adds the starting node to the queue
        let new_edges_one=self.graph.sort_edges();
        while let Some(v) = self.queue.pop_front() { //if there is a node in the queue it analyzes it then removes it.
            let index=self.graph.get_sorted_index(v as i32);
            for u in new_edges_one[index as usize].iter() {//checks the node's neighbors
                let new_index=self.graph.get_sorted_index(*u as i32);
                if let None = distance[new_index as usize] { //updates the distance if it is none by adding the value by 1
                    distance[new_index as usize] = Some(distance[index as usize].unwrap() + 1);
                    self.queue.push_back(*u as usize);//adds the new neighbors nodes to the queue.
                }
            }
        }
        distance

    }
    pub fn maximum_distance_for_node(&mut self, node:usize)->u32{//calculates the maximum distance by using the calculate_distance() function.
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