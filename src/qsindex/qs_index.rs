#![allow(dead_code)]

use super::qs_node::QsNode;

/// 
/// Struct representing the index for the incremental quicksort (inspired by Pedro Holanda)
/// with respect to an attribute (of type String)
pub struct IncrQsIndex
{
    pub nodes: Vec<QsNode<String>>,
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
            index: None, 
            data: None, 
            curr_pos: 0, 
            curr_pivot: 0, 
            first_run: true
        }
    }

    /// 
    /// Method to translate String (Key) found in raw file into Index data format
    /// 
    /// * `value` - String found in raw file to be translated
    fn parse_value(value: String) -> String 
    {
        value
    }

    ///
    /// Initialized index by creating the vectors and setting the state parameters
    /// 
    /// * `num_rows` - Maximum number of rows required for the index (number of objects in raw file)
    pub fn init_index(&mut self, num_rows: usize)
    {
        self.index = Some(vec![0; num_rows]);
        self.data = Some(vec![String::from(""); num_rows]);
        let a = &mut self.nodes[0];

        a.start = 0;
        a.end = num_rows as i64;
        a.curr_start = 0;
        a.curr_end = a.end - 1; 
        self.curr_pos = 0;
    }

    /// 
    /// Getter method returning the numeber of nodes in the index
    pub fn get_nodes_length(&self) -> usize
    {
        return self.nodes.len();
    }

    ///
    ///  Method returning the current node (next to be sorted progressively)
    pub fn get_current_node(&mut self) -> &mut QsNode<String>
    {
        return self.nodes.get_mut(self.curr_pivot).unwrap();
    }

    ///
    /// Method for checking recusively if the index is sorted (tree structure)
    /// 
    /// * `node_idx` - Index of the node to be checked
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

    ///
    /// Printing the index (values)
    pub fn print_index(&self)
    {
        match &self.data {
            Some(data) => println!("Index: {:?}", data),
            None => println!("Index empty!"),
        }
    }

    ///
    /// Printing nodes for debugging
    pub fn print_nodes(&self)
    {
        for n in &self.nodes
        {
            n.print_node();   
        }
    }

}