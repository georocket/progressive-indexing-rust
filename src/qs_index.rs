#![allow(dead_code)]


#[derive(Debug)]
pub struct QsNode 
{
    pub position: i64,
    pub parent: i64,
    pub sorted: bool,
    pub start: i64,
    pub end: i64,
    pub curr_start: i64,
    pub curr_end: i64,
    pub pivot: String,
    pub left: Option<i64>,
    pub right: Option<i64>,
    pub min: String,
    pub max: String,
    already_rebalanced: bool
}

impl QsNode 
{
    pub fn new(pos: i64, parent: i64) -> Self
    {
        Self {
            position: pos,
            parent,
            sorted: false,
            start: -1, 
            end: -1, 
            curr_start: -1, 
            curr_end: -1, 
            pivot: String::from("Empty"), 
            left: None, 
            right: None, 
            min: String::from("MIN"), 
            max: String::from("MAX"), 
            already_rebalanced: false 
        }
    }

    pub fn reset_curr_pointers(&mut self)
    {
        self.curr_start = self.start;
        self.curr_end = self.end - 1;
    }

    pub fn split(&mut self, index: &Vec<String>, pos: i64, parent: i64) -> (QsNode, QsNode)
    {
        self.left = Some(pos);
        self.right = Some(pos + 1);

        let mut left = QsNode::new(pos, parent);
        left.start = self.start;
        left.end = if index[self.curr_start as usize].as_str() < self.pivot.as_str() { self.curr_start + 1 } else { self.curr_start };
        left.curr_start = left.start;
        left.curr_end = left.end - 1;
        let idx = ((left.curr_start + left.curr_end)/2) as usize;
        left.pivot = index[idx].clone();
        left.min = self.min.clone();
        left.max = self.pivot.clone();

        let mut right = QsNode::new(pos + 1, parent);
        right.start = left.end;
        right.end = self.end;
        right.curr_start = right.start;
        right.curr_end = right.end - 1;
        right.pivot = index[((right.curr_start + right.curr_end)/2) as usize].clone();
        right.min = self.pivot.clone();
        right.max = self.max.clone();

        (left, right)
    }
}


pub struct IncrQsIndex
{
    pub nodes: Vec<QsNode>,             // Maybe put nodes inside boxes? 
    pub root: Box<Option<QsNode>>,
    pub index: Option<Vec<usize>>,
    pub data: Option<Vec<String>>,
    pub curr_pos: usize,
    pub curr_pivot: usize,
    pub first_run: bool
}

impl IncrQsIndex
{
    pub fn new() -> Self
    {
        Self { 
            nodes: vec![], 
            root: Box::new(Some(QsNode::new(-1, -1))), 
            index: None, 
            data: None, 
            curr_pos: 0, 
            curr_pivot: 0, 
            first_run: true
        }
    }

    fn parse_value(value: String) -> String 
    {
        value
    }

    pub fn init_index(&mut self, num_rows: usize)
    {
        self.index = Some(vec![0; num_rows]);
        self.data = Some(vec![String::from(""); num_rows]);
        let x = self.root.as_mut();
        match x {
            Some(a) => 
            {
                a.start = 0;
                a.end = num_rows as i64;
                a.curr_start = 0;
                a.curr_end = a.end - 1; 
            },
            None => {},
        }
        self.curr_pos = 0;
    }

    pub fn get_nodes_length(&self) -> usize
    {
        //let le = self.nodes.len();
        return 10;
    }

    pub fn get_current_node(&mut self) -> &mut QsNode
    {
        return self.nodes.get_mut(self.curr_pivot).unwrap();
    }

    pub fn print_index(&self)
    {
        match &self.data {
            Some(data) => println!("Index: {:?}", data),
            None => println!("Index empty!"),
        }
    }

}