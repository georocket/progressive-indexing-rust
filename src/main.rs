mod file_buffer;
mod query_engine;
mod boyer_moore;
mod matcher;
mod utility;
mod own_linked_list;
mod progressive_quicksort_time;
mod qs_index;
use std::fs::{File, self};
use std::io::{Read, Write, BufWriter};
use std::os::unix::prelude::FileExt;
use std::time::{Instant, Duration};
use std::vec;
use grep::printer::{StandardBuilder, ColorSpecs};
use grep::regex::RegexMatcher;
use grep::searcher::{SearcherBuilder, SinkMatch, SinkFinish, BinaryDetection};
use grep::searcher::Sink;
use grep::searcher::Searcher;
use grep::cli;
use termcolor::ColorChoice;
use crate::boyer_moore::{BoyerMoore, BoyerMooreBasicIterator, BoyerMooreAttributeByKeyIterator};
use crate::file_buffer::FileBuffer;
use crate::progressive_quicksort_time::range_query_incremetal_quicksort_time;
use crate::qs_index::{IncrQsIndex, QsNode};
use crate::query_engine::QueryEngine;

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
struct binary_tree<T>
{
    root: Option<Box<Node<T>>>,
}

impl binary_tree<i64>
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
                binary_tree::insert_recursive_helper(node, value);
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
                    binary_tree::insert_recursive_helper(right, value); 
                }
            }
        } else {
            match &mut node.left {
                None => { 
                    node.left = Some(Box::new(Node::new(value))); 
                },
                Some(left) => { 
                    binary_tree::insert_recursive_helper(left, value); 
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
                binary_tree::trav_rec_helper(node, mode); 
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
                binary_tree::trav_rec_helper(left, mode); 
            }
        }
        if mode == 1 { print!("{}, ", node.value) };
        match &node.right
        {
            None => {},
            Some(right) => { 
                binary_tree::trav_rec_helper(right, mode); 
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


fn main() {
    let mut idx = Index::new();

    {
        let p = idx.nodes.len() as i32;
        let (n, n2) = idx.get_act_node();
        let (a,b) = n.split(p, n2);
        idx.add(a);
        idx.add(b);
        idx.curr_pos +=1;
    }

    {
        let p = idx.nodes.len() as i32;
        let (n, n2) = idx.get_act_node();
        let (a,b) = n.split(p, n2);
        idx.add(a);
        idx.add(b);
        idx.curr_pos +=1;
    }

    {
        let p = idx.nodes.len() as i32;
        let (n, n2) = idx.get_act_node();
        let (a,b) = n.split(p, n2);
        idx.add(a);
        idx.add(b);
        idx.curr_pos +=1;
    }

    println!("{:?}", idx.nodes);

    println!("Hello World!")

}
