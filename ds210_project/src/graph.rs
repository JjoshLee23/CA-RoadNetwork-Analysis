#[derive(Debug)]
#[derive(Clone)]
pub struct Graph{//created a struct "Graph" that has 2 vectors for nodes and corresponding edges
    pub nodes:Vec<i32>,
    pub edge: Vec<Vec<usize>>,

}
impl Graph{
    pub fn new_graph()->Self{
        Self{
            nodes:Vec::new(),
            edge:Vec::new(),
        }

    }
    pub fn add_nodes(&mut self,point:i32){//add node function which adds nodes to the vector
        let mut count=0;
        for i in 0..self.nodes.len(){
            if self.nodes.get(i).unwrap()==&point{
                count=count+1;
            }
        }
        if count==0{
            self.nodes.push(point);
            self.edge.push(vec![]);
        }
    }
    pub fn add_edge(&mut self, edge:usize,point:i32){//add edge function which add edges to the vector 
        let mut count1=0;
        let index:i32=self.get_index(point);
        for j in 0..self.edge.get(index as usize).unwrap().len(){
            if self.edge[index as usize][j]==edge{
                count1=count1+1;
            }
        }
        if count1==0{
            self.edge[index as usize].push(edge);
        }
    }
    pub fn get_index(&self, value:i32)->i32{//function where I get the index of the unsorted nodes
        let mut index: i32=-1;
        for i in 0..self.num_nodes(){
            if self.nodes.get(i as usize).unwrap()==&value{
                index=i as i32
            }
        }
        index
    }
    pub fn get_sorted_index(&self, value:i32)->i32{//sorts the nodes by cloning the original nodes and find the index value
        let mut sorted_index:i32=-1;
        let mut sorted_nodes=self.nodes.clone();
        sorted_nodes.sort();
        for i in 0..sorted_nodes.len(){
            if sorted_nodes[i]==value{
                sorted_index=i as i32
            }
        }
        sorted_index
    }
    pub fn display_graph(&mut self){//used to display the entire graph
        for i in 0..self.nodes.len(){
            println!("Node: {:7?}   Edges: {:?}",self.nodes[i],self.edge[i]);
        }
    }
    pub fn num_nodes(&self)->i32{
        return self.nodes.len().try_into().unwrap();//gets the number of nodes
    }
    pub fn sort_edges(&mut self)->Vec<Vec<usize>>{//function where it sorts the edges based on the sorted vector.
        let unsorted_nodes=self.nodes.clone();
        let mut sorted_nodes=self.nodes.clone();
        sorted_nodes.sort();
        let mut new_edges:Vec<Vec<usize>>=vec![vec![];self.nodes.len()];
        for &i in unsorted_nodes.iter(){
            let index=self.get_index(i);
            for &j in sorted_nodes.iter(){
                if i==j{
                    let new_index=self.get_sorted_index(j);
                    new_edges[new_index as usize]=self.edge[index as usize].clone();
                }

            }
        }
        new_edges
    }

}
