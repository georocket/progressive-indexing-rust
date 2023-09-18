#![allow(dead_code)]
#![allow(unused_variables)]
//#![allow(unused_imports)]

use std::fmt::Display;
use std::ptr::null;

use fileaccess::query_engine::QueryEngine;
use progressive_quicksort_time::range_query_incremetal_quicksort_time;
use rand::Rng;

use crate::utility::{binary_search_gte, binary_search_lte, test_function, linear_check};
use crate::qsindex::qs_index::IncrQsIndex;

mod fileaccess;
mod qsindex;
mod progressive_quicksort_time;
mod utility;


const MS_TO_NS: u32 = 1000000;


#[derive(Debug)]
struct Node<T>
{
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl Node<i64>
{
    pub fn new(value: i64) -> Self
    {
        Self { value, left: None, right: None }
    }
}

#[derive(Debug)]
struct BinaryTree<T>
{
    root: Option<Box<Node<T>>>,
}

impl BinaryTree<i64>
{
    pub fn new() -> Self
    {
        Self { root: None }
    }

    pub fn insert(&mut self, value: i64)
    {
        match &mut self.root
        {
            None => { 
                self.root = Some(Box::new(Node::new(value))); 
            },
            Some(node) => 
            {
                BinaryTree::insert_recursive_helper(node, value);
            }
        }   
    }

    fn insert_recursive_helper(node: &mut Box<Node<i64>>, value: i64)
    {
        if value > node.value
        {
            match &mut node.right {
                None => { 
                    node.right = Some(Box::new(Node::new(value))); 
                },
                Some(right) => { 
                    BinaryTree::insert_recursive_helper(right, value); 
                }
            }
        } else {
            match &mut node.left {
                None => { 
                    node.left = Some(Box::new(Node::new(value))); 
                },
                Some(left) => { 
                    BinaryTree::insert_recursive_helper(left, value); 
                }
            }
        }
    }

    pub fn traverse_tree(&self, mode: i32)
    {
        print!("\n\nTree: ");
        match &self.root
        {
            None => { println!("Tree is empty!"); },
            Some(node) => { 
                BinaryTree::trav_rec_helper(node, mode); 
            }
        }
        println!("\n");
    }

    fn trav_rec_helper(node: &Box<Node<i64>>, mode: i32)
    {
        if mode == 0 { print!("{}, ", node.value) };
        match &node.left
        {
            None => {},
            Some(left) => { 
                BinaryTree::trav_rec_helper(left, mode); 
            }
        }
        if mode == 1 { print!("{}, ", node.value) };
        match &node.right
        {
            None => {},
            Some(right) => { 
                BinaryTree::trav_rec_helper(right, mode); 
            }
        }
        if mode == 2 { print!("{}, ", node.value) };
    }
}


#[derive(Debug)]
struct IndexNode
{
    value: i32,
    parent: i32,
    left: Option<i32>,
    right: Option<i32>,
    sorted: bool
}

impl IndexNode
{
    pub fn new(value: i32, parent: i32) -> Self
    {
        Self { 
            value,
            parent,
            left: None, 
            right: None,
            sorted: false
        }
    }

    pub fn split(&mut self, pos: i32, parent: i32) -> (IndexNode, IndexNode)
    {
        self.left = Some(pos);
        self.right = Some(pos + 1);
        let left = IndexNode::new(self.value/2, parent);
        let right = IndexNode::new((self.value/2) + self.value, parent);

        (left, right)
    }
}

#[derive(Debug)]
struct Index
{
    nodes: Vec<Box<IndexNode>>,
    curr_pos: usize
}

impl Index
{
    pub fn new() -> Self
    {
        Self { nodes: vec![Box::new(IndexNode::new(50, -1))], curr_pos: 0 }
    }

    pub fn get_act_node(&mut self) -> (&mut Box<IndexNode>, i32)
    {
        return (&mut self.nodes[self.curr_pos], self.curr_pos as i32);
    }

    pub fn add(&mut self, node: IndexNode)
    {
        self.nodes.push(Box::new(node));
    }

    fn sorted_check(&mut self, node_idx: usize)
    {   
        let n = self.nodes[node_idx].left;
        let r = self.nodes[node_idx].right;

        let left_sorted =  match n {
            Some(id) => {
                let left = &mut self.nodes[id as usize];
                left.sorted
            },
            None => {
                true
            },
        };

        let right_sorted = match r {
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
            node.left = None;
            node.right = None;

            if node.parent >= 0
            {
                let parent = node.parent as usize;
                self.sorted_check(parent);
            }
        }
    }
}


pub struct SomeTestStruct
{
    value: Vec<i32>
}


fn main() {
    const TESTFILE_2: &str = "/home/derpadi/Documents/Work/Fraunhofer_IGD/Rust2/ProgressiveIndexingRust/src/rawfile.txt";
    // let mut idx = Index::new();

    // {
    //     let p = idx.nodes.len() as i32;
    //     let (n, n2) = idx.get_act_node();
    //     let (a,b) = n.split(p, n2);
    //     idx.add(a);
    //     idx.add(b);
    //     idx.curr_pos +=1;
    // }

    // {
    //     let p = idx.nodes.len() as i32;
    //     let (n, n2) = idx.get_act_node();
    //     let (a,b) = n.split(p, n2);
    //     idx.add(a);
    //     idx.add(b);
    //     idx.curr_pos +=1;
    // }

    // {
    //     let p = idx.nodes.len() as i32;
    //     let (n, n2) = idx.get_act_node();
    //     let (a,b) = n.split(p, n2);
    //     idx.add(a);
    //     idx.add(b);
    //     idx.curr_pos +=1;
    // }

    //println!("{:?}", idx.nodes);

    // Testing the binary_search functions
    //binary_search();

    //let v = vec![7, 40, 40, 40, 68, 68, 69, 69, 77];
    //single_test(&v);
    //mass_test();

    // qs_index: &mut IncrQsIndex, mut query: QueryEngine, time_budget: u32

    let mut idx = IncrQsIndex::new();
    let mut qry = QueryEngine::new(TESTFILE_2.to_string());

    println!("Rows: {}", qry.num_rows);
    idx.init_index(qry.num_rows);

    range_query_incremetal_quicksort_time("ownername", "A", "D", &mut idx, &mut qry, 300 * MS_TO_NS);
    idx.print_nodes();
    println!("Data: {:?}", idx.data);
    println!("Index: {:?}", idx.index);
}








pub fn single_test(vec: &Vec<i32>)
{
    println!("TestArray: {:?}", vec);
    let res_test = test_function(vec, 40, 60);
    let res_val = linear_check(vec, 40, 60);

    if res_test.len() != res_val.len()
    {
        println!("Test: {:?}", res_test);
        println!("Vali: {:?}", res_val);
    }
}

pub fn mass_test()
{
    let mut test_array: Vec<i32> = vec![];

    for i in 0..10000
    {
        test_array.clear();
        fill_test_array_rnd(&mut test_array, 5);
        let res_test = test_function(&test_array, 40, 60);
        let res_val = linear_check(&test_array, 40, 60);
        //assert!(res_test.len() == res_val.len(), "Lengths do not match!");
        if res_test.len() != res_val.len()
        {
            println!("_________________________________________________________________________________________");
            println!("TestArray: {:?}", test_array);
            println!("Iteration: {}", i);
            //println!("TestArray: {:?}", test_array);
            println!("Test: {:?}", res_test);
            println!("Vali: {:?}", res_val);
        }
    }

    println!("Hello World!")
}

pub fn fill_test_array_rnd(array: &mut Vec<i32>, num_elements: i32)
{
    let mut rng = rand::thread_rng();
    for _ in 0..num_elements
    {
        let random_value: i32 = rng.gen_range(0..100);
        let duplicates: i32 = rng.gen_range(1..6);

        for _ in 0..duplicates
        {
            array.push(random_value);
        }
    }
    array.sort();
}


pub fn some_coparison<T: Ord>(a: &T, b: &T) -> bool
{
    a < b
}

pub fn some_generic_function<T : Display>(t: T)
{
    println!("Something: {}", t);
}
