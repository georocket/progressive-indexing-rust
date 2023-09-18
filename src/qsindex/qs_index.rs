#![allow(dead_code)]

use super::qs_node::QsNode;

pub struct IncrQsIndex
{
    pub nodes: Vec<QsNode<String>>,             // Maybe put nodes inside boxes? 
    pub root: Option<QsNode<String>>,
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
            nodes: vec![QsNode::new(0, None)], 
            root: Some(QsNode::new(0, None)), 
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
        return self.nodes.len();
    }

    pub fn get_current_node(&mut self) -> &mut QsNode<String>
    {
        return self.nodes.get_mut(self.curr_pivot).unwrap();
    }

    pub fn sorted_check(&mut self, node_idx: usize)
    {
        let left_node =self.nodes[node_idx].left;
        let right_node = self.nodes[node_idx].right;

        let left_sorted = match left_node {
            Some(id) => {
                let left = &mut self.nodes[id as usize];
                left.sorted
            },
            None => {
                true
            },
        };
        let right_sorted = match right_node {
            Some(id) => {
                let right = &mut self.nodes[id as usize];
                right.sorted
            },
            None => {
                true
            },
        };

        if right_sorted && left_sorted
        {
            let node = &mut self.nodes[node_idx];
            node.sorted = true;
            //node.left = None;
            //node.right = None;

            match node.parent {
                Some(parent) => {
                    if parent < 0
                    { 
                        println!("{:?}", self.nodes);
                        println!("Not good!");
                    }
                    self.sorted_check(parent as usize);
                },
                None => {},
            }
        }
    }

    pub fn print_index(&self)
    {
        match &self.data {
            Some(data) => println!("Index: {:?}", data),
            None => println!("Index empty!"),
        }
    }

    pub fn print_nodes(&self)
    {
        for n in &self.nodes
        {
            n.print_node();   
        }
    }

}