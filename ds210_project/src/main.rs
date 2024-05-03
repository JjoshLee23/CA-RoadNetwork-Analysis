mod graph;
use crate::graph::Graph;

mod path;
use crate::path::Path;

use std::fs::File;

use rand::seq::SliceRandom;
use std::io::{BufRead};
use std::io;
fn main() {
    
    let  mut vector_of_nodes=read_file("roadNet-CA (1).txt");
    let path=Path::initialization(vector_of_nodes.clone());
    loop{
        println!("1. Graph output\n2. Find Shortest Path for 2 Roads\n3. Find route\n4. End\nselect");
        let mut select = String::new();
        io::stdin().read_line(&mut select).expect("Fail to read line");
        let result: Result<i64, _> = select.trim().parse();
        match result {
            Ok(number) => {
                if number == 1{
                    vector_of_nodes.display_graph();
                }else if number == 2{
                    two_nodes(path.clone());
                }
               // }else if number == 3{
                    //select_three(paths.clone());
                else if number==4{
                    println!("—————Program End—————");
                    return;
                }
            }Err(e) => {
                println!("Error: {}", e);
            }
        }

    }
   fn two_nodes(mut path:Path){
    println!("Enter the start node: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number1: usize = input.trim().parse().expect("Please enter a valid number");
    println!("Enter the end node: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let number2: usize = input.trim().parse().expect("Please enter a valid number");
    let distance=path.calculate_distance(number1);
    let mut count=0;
    for i in 0..distance.len(){
        if i==number2 && distance[i]!=None {
            println!("The shortest distance from {:?} to {:?} is: {:?}",number1, number2,distance[i].unwrap());
        }
        else{
            count=count+1;
        }

    }
    if count==distance.len(){
        println!("There is no pathway from {:?} to {:?} ",number1,number2);
    }


   } 


    
    
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

       if i >= 100 {
           break;
        }
    }
    graph
}
       

