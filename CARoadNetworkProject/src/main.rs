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
    let mut vector_of_nodes=read_file("roadNet-CA (1).txt");//creates an instance of the graph
    
    let mut path=Path::initialization(vector_of_nodes.clone());//creates an instance of my path struct by cloning my instance of graph.
    
    loop{
        //this loop is for my user input (either 1,2,3,4,5). If the user inputs 5 the program will terminate.
        println!("1. Display of Roads and their Paths\n2. Find Shortest Path for 2 Roads\n3. Find Roads For Node\n4. Find max distance a road has\n5. Exit\nselect");
        //this code is for user input
        let mut select = String::new();
        io::stdin().read_line(&mut select).expect("Fail to read line");
        let result: Result<i64, _> = select.trim().parse();
        match result {//I use a match function to see which one of the user input corresponds to the number.
            Ok(number) => {//if it is a valid user input, it uses if statement
                //each if statement corresponds to a function that displays the graph, calculates the shortest distance or max distance.
                if number == 1{
                    vector_of_nodes.display_graph();
                }else if number == 2{
                    two_nodes(&mut path,&mut vector_of_nodes);
                }
               else if number == 3{
                    display_edge(&mut vector_of_nodes)
               }
               else if number==4{
                println!("Enter the start node: ");
                let mut input1 = String::new();
                io::stdin().read_line(&mut input1).expect("Failed to read line");
                let node: usize = input1.trim().parse().expect("Please enter a valid number");
                let max=path.maximum_distance_for_node(node);
                println!("The maximum distance road {:?} has to any road is {:?} ",node,max);
               }
                else if number==5{
                    println!("------Thanks for checking out CA Roads!!------");
                    return;
                }
                //prints an error if the input is invalid
            }Err(e) => {
                println!("Error: {}", e);
            }
        }
        }

    }
    //my function for outputting the shortest distance between 2 nodes
   fn two_nodes(path:&mut Path,graph:&mut Graph){
    let mut sorted_nodes=graph.nodes.clone();
    sorted_nodes.sort();
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
    for i in 0..distance.len(){//checks if the number inputted matches any of the nodes.
        if distance[i]!=None && graph.get_sorted_index(number2 as i32) as usize==i{
            println!("The shortest distance from {:?} to {:?} is: {:?}",number1, number2,distance[i].unwrap());
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
//my function for displaying the graph in general (nodes and the corresponding edges)
    fn display_edge(graph:&mut Graph){
    println!("Enter the start node: ");
    let mut _valid=false;
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let mut node1: i32 = input1.trim().parse().expect("Please enter a valid number");
    if graph.get_index(node1)!=-1 {//checks if the node inputted is valid
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


//this function reads the file and creates a graph instance
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

        //takes the first 100,000 nodes
       if i >= 100000 {
           break;
        }
    }
    graph
}

#[cfg(test)]

mod tests{
    use super::*;

     #[test]
      fn correct_graph(){//tests if the graph adds the correct edges to the nodes
          let mut graph=Graph::new_graph();
          graph.add_nodes(1);
          graph.add_nodes(2);
          graph.add_nodes(3);
          graph.add_nodes(4);
          graph.add_edge(10,1);
          graph.add_edge(9,1);
          graph.add_edge(5,2);
          graph.add_edge(6,3);
          assert_eq!(graph.edge[0],vec![10,9]);
          assert_eq!(graph.edge[1],vec![5]);
          assert_eq!(graph.edge[2],vec![6]);
          assert_eq!(graph.edge[3],vec![]);
          assert_eq!(graph.nodes,vec![1,2,3,4]);

      }
     #[test]
     fn shortest_distance(){
        let graph1=read_file("data.txt");
        let mut path=Path::initialization(graph1);
        let distance=path.calculate_distance(1);
        assert_eq!(distance[5].unwrap(),1);
        assert_eq!(distance[1].unwrap(),0);
        assert_eq!(distance[0].unwrap(),1);
        assert_eq!(distance[3].unwrap(),3);

     }

 }
       

