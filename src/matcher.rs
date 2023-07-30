use std::{collections::{HashMap, LinkedList}, ptr::NonNull};



pub struct Node {
    value: char,
    final_state: bool,
    start_node: bool,
    next_nodes: HashMap<char, NonNull<Node>>
}

impl Node {
    pub fn new(value: char) -> Self {
        Self {  value, 
                final_state: false, 
                start_node: false,     
                next_nodes: HashMap::new()
            }
    }

    pub fn step(&self, input: char) -> Option<&Node> {
        //return if self.next_nodes.contains_key(&input) {
        //    Some(&self.next_nodes[&input])
        //} else {
        //    None
        //};
        None
    }

    pub fn addNeighbour(&mut self, node: Box<Node>, step: char) {
        //self.next_nodes.insert(step, node);
    }

    pub fn setFinite(&mut self, state: bool) {
        self.final_state = state;
    }
}

pub struct OtherStructure {
    length: u32,
    // Basically a node pointer
    head: Option<NonNull<Node>>,
    // Act-Pos -> Another node pointer
    act_pos: Option<NonNull<Node>>
}

impl OtherStructure {
    pub fn new() -> Self {
        Self { 
            length: 0, 
            head: None, 
            act_pos: None 
        }
    }
}


pub struct SimpleMatcher {
    pattern: String,
    act_pos: usize,
    length: usize,
    found: bool
}

impl SimpleMatcher {
    pub fn new(pattern: &str) -> Self {
        Self { 
            pattern: String::from(pattern), 
            act_pos: 0,
            length: pattern.len(),
            found: false
        } 
    }

    pub fn step(&mut self, input: &char) -> bool {
        if self.found {
            return true
        }

        let c = self.pattern.chars().nth(self.act_pos).unwrap();
        if c == *input {
            self.act_pos += 1;
        } else {
            self.act_pos = 0;
        }
        if (self.act_pos >= self.length) {
            self.found = true;
            return true
        } else {
            return false          
        }
    }

    pub fn reset(&mut self) {
        self.found = false;
        self.act_pos = 0;
    }
}

pub struct Matcher {
    pattern: String,
    start_node: Option<NonNull<Node>>,
    act_step: Box<Node>,
    last_letter_node: Box<Node>,
    found: bool,

    // Only optional if needed!
    //adresses: LinkedList<usize>,
    //last_address: usize
}

impl Matcher {
    pub fn new(pattern: &str) -> Self {
        let mut start_node = Box::new(Node::new('0'));
        let act_step = start_node;
        let zero = 0;
        //start_node.addNeighbour(start_node, '0');

        
        Self { 
            pattern: String::from(pattern), 
            start_node: None, 
            act_step: Box::new(Node::new('c')), 
            last_letter_node: Box::new(Node::new('c')), 
            found: false 
        }
    }
}