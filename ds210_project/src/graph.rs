#[derive(Debug)]
#[derive(Clone)]
pub struct Graph{
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
    pub fn add_nodes(&mut self,point:i32){
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
    pub fn add_edge(&mut self, edge:usize,point:i32){
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
    pub fn get_index(&mut self, value:i32)->i32{
        let mut index: i32=-1;
        for i in 0..self.num_nodes(){
            if self.nodes.get(i as usize).unwrap()==&value{
                index=i as i32
            }
        }
        index
    }
    pub fn display_graph(&mut self){
        for i in 0..self.nodes.len(){
            println!("Node: {:7?}   Edges: {:?}",self.nodes[i],self.edge[i]);
        }
    }
    pub fn num_nodes(&mut self)->i32{
        return self.nodes.len().try_into().unwrap();
    }
    pub fn max_node(&self) -> Option<i32> {
            self.nodes.iter().cloned().max()
        }
}
