mod graph;
use crate::graph::Graph;

mod path;
use crate::path::Path;

use std::fs::File;
use std::io::prelude::*;
fn main() {
    
    let  mut vector_of_nodes=read_file("roadNet-CA (1).txt");
    vector_of_nodes.display_graph();
    print!("{:?}",vector_of_nodes.edge[1].iter());
    let path=Path::initialization(vector_of_nodes);
   // print!("{:?}",path.calculate_distance(1));


    
    
}
fn read_file(path: &str) -> Graph {
    let file = File::open(path).expect("Could not open file");
    let mut buf_reader = std::io::BufReader::new(file).lines();
    for _i in 0..1{
         buf_reader.next().expect("File is empty").unwrap();
         buf_reader.next().expect("File is empty").unwrap();
         buf_reader.next().expect("File is empty").unwrap();
         buf_reader.next().expect("File is empty").unwrap();

    }
    let mut graph=Graph::new_graph();
    let mut i=0;
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split_whitespace().collect();
        let x = v[0].parse::<i32>().unwrap();
        let y = v[1].parse::<i32>().unwrap();
        i=i+1;
        if i>2000{
            break;
        }
       
        graph.add_nodes(x as i32);
        graph.add_nodes(y as i32);

        graph.add_edge(y as i32,x);
        graph.add_edge(x as i32,y);
     }
     graph
       
    }
