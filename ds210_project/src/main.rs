mod graph;
use crate::graph::Graph;


use std::fs::File;
use std::io::prelude::*;
fn main() {
    
    let  vector_of_nodes=read_file("Seoul_Edgelist.csv");
    print!("{:?}",vector_of_nodes);

    
    
}
fn read_file(path: &str) -> Graph {
    let file = File::open(path).expect("Could not open file");
    let mut buf_reader = std::io::BufReader::new(file).lines();
    for _i in 0..1{
         buf_reader.next().expect("File is empty").unwrap();

    }
    let mut graph=Graph::new_graph();
    let mut i=0;
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(',').collect();
        let x = v[2].parse::<i32>().unwrap();
        let y = v[3].parse::<i32>().unwrap();
        i=i+1;
        if i>1000{
            break;
        }
       
        graph.add_nodes(x as i32);

        graph.add_edge(y as i32,x);
     }
     graph
       
    }
