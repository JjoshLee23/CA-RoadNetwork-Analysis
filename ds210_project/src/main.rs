mod graph;
use crate::graph::Graph;

mod path;
use crate::path::Path;

use std::fs::File;

use std::collections::VecDeque;

use rand::seq::SliceRandom;
use std::io::BufRead;
use std::io;
fn main() {
    
    let mut vector_of_nodes=read_file("data.txt");
    
    let mut path=Path::initialization(vector_of_nodes.clone());
    loop{
        println!("1. Graph output\n2. Find Shortest Path for 2 Roads\n3. Find Edges For Node\n4. End\nselect");
        let mut select = String::new();
        io::stdin().read_line(&mut select).expect("Fail to read line");
        let result: Result<i64, _> = select.trim().parse();
        match result {
            Ok(number) => {
                if number == 1{
                    vector_of_nodes.display_graph();
                }else if number == 2{
                    two_nodes(&mut path,&mut vector_of_nodes);
                }
               else if number == 3{
                    display_edge(&mut vector_of_nodes)
               }
                else if number==4{
                    println!("—————End of Program—————");
                    return;
                }
            }Err(e) => {
                println!("Error: {}", e);
            }
        }
        }

    }
   fn two_nodes(path:&mut Path,graph:&mut Graph){
    println!("Enter the start node: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let number1: usize = input1.trim().parse().expect("Please enter a valid number");
    println!("Enter the end node: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let number2: usize = input2.trim().parse().expect("Please enter a valid number");
    let distance=path.calculate_distance(number1);
    let mut t_or_f=false;
    for i in 0..distance.len(){
        if distance[i]!=None && graph.get_sorted_index(number2 as i32) as usize==i{
            println!("The shortest distance from {:?} to {:?} is: {:?}",number1, number2,distance[i].unwrap());
            println!("The path you take is {:?} ",path.get_middle_edges(number1,number2));
            t_or_f=true;
            break;
        }
        else{
            t_or_f=false;
        }

    }
    if t_or_f==false{
        println!("There is no pathway from {:?} to {:?} ",number1,number2);
    }


   } 

   fn display_edge(graph:&mut Graph){
    println!("Enter the start node: ");
    let mut _valid=false;
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let mut node1: i32 = input1.trim().parse().expect("Please enter a valid number");
    if graph.get_index(node1)!=-1 {
        _valid=true;
    }
    else{
        _valid=false;
    }
    while _valid!=true{
        if graph.get_index(node1)!=-1{
            _valid=true;
            break;
        }
        if node1==-1{
            return;
        }
        else{
            println!("This node is not valid, please re-enter or enter -1 to exit.");
            input1 = String::new();
            io::stdin().read_line(&mut input1).expect("Failed to read line");
            node1= input1.trim().parse().expect("Please enter a valid number");
        }

    }
    let index=graph.get_index(node1);
    println!("The edges corresponding to this node: {:?} are {:?} ",node1,graph.edge[index as usize]);
   }


fn read_file(path: &str) -> Graph {
    let file = File::open(path).expect("Could not open file");
    let mut buf_reader = std::io::BufReader::new(file).lines();
    for _i in 0..4{
         buf_reader.next().expect("File is empty").unwrap();

    }
    let mut dataset: VecDeque<String> = buf_reader.map(|line| line.expect("Error reading line")).collect();

    // Shuffle the dataset
    let mut rng = rand::thread_rng();
    dataset.make_contiguous().shuffle(&mut rng);

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
       

