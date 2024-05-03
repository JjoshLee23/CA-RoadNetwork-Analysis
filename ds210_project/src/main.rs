mod graph;
use crate::graph::Graph;

mod path;
use crate::path::Path;

use std::fs::File;
use std::io::prelude::*;

use rand::seq::SliceRandom;
fn main() {
    
    let  mut vector_of_nodes=read_file("roadNet-CA (1).txt");
    //vector_of_nodes.display_graph();
    //print!("{:?}",vector_of_nodes.num_nodes());
    //vector_of_nodes.nodes.sort();
    let mut path=Path::initialization(vector_of_nodes);


    
    
}
fn read_file(path: &str) -> Graph {
    let file = File::open(path).expect("Could not open file");
    let mut buf_reader = std::io::BufReader::new(file).lines();
    for _i in 0..4{
         buf_reader.next().expect("File is empty").unwrap();

    }
    let mut dataset: Vec<String> = buf_reader.map(|line| line.expect("Error reading line")).collect();

    // Shuffle the dataset
    let mut rng = rand::thread_rng();
    dataset.shuffle(&mut rng);

    let mut graph = Graph::new_graph();
    let mut i = 0;
    for line in dataset.iter(){
        let v: Vec<&str> = line.trim().split_whitespace().collect();
        let x = v[0].parse::<i32>().unwrap();
        let y = v[1].parse::<i32>().unwrap();
        i += 1;
       
        graph.add_nodes(x as i32);
        graph.add_nodes(y as i32);

        graph.add_edge(y as usize, x);
        graph.add_edge(x as usize, y);

        if i >= 1000 {
            break;
        }
    }
    graph
}
       

