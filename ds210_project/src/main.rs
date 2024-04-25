mod graph;
use crate::graph::Graph;


use std::fs::File;
use std::io::prelude::*;
fn main() {
    
    let  vector_of_nodes=read_file("data.txt");
    print!("{:?}",vector_of_nodes);
}
fn read_file(path: &str) -> Graph {
    let file = File::open(path).expect("Could not open file");
    let mut buf_reader = std::io::BufReader::new(file).lines();
    let first_line = buf_reader.next().expect("File is empty").unwrap();
    let _num_vertices = first_line.trim().parse::<usize>().expect("Invalid number of vertices");
    let mut graph=Graph::new_graph();

    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let x = v[0].parse::<i32>().unwrap();
        let y = v[1].parse::<i32>().unwrap();
       
        graph.add_nodes(x as i32);
        graph.add_nodes(y as i32)
     }
     graph
       
    }
