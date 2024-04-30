#[derive(Debug)]

pub struct Graph{
    pub node:Vec<i32>,
    pub edge: Vec<Vec<i32>>,

}

impl Graph{
    pub fn new_graph()->Self{
        Self{
            node:Vec::new(),
            edge:Vec::new(),
        }

    }
    pub fn add_nodes(&mut self,point:i32){
        let mut count=0;
        for i in 0..self.node.len(){
            if self.node.get(i).unwrap()==&point{
                count=count+1;
            }
        }
        if count==0{
            self.node.push(point);
            self.edge.push(vec![]);
        }
    }
    pub fn add_edge(&mut self, edge:i32,point:i32){
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
        for i in 0..self.node.len(){
            if self.node.get(i).unwrap()==&value{
                index=i as i32
            }
        }
        index
    }
    pub fn print_graph(&self){
        for i in 0..self.node.len(){
            print!("Node {:?} Edges: {:?}",self.node[i],self.edge[i]);
        }
    }
}