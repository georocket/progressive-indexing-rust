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
struct my_struct
{
    a: String,
    b: String,
    c: i64
}

#[derive(Debug)]
struct Foo{}

fn main() {
    const TESTFILE_1: &str = "/home/derpadi/Documents/Work/Fraunhofer_IGD/ProgressiveIndexingRust/src/DA12_3D_Buildings_Merged.gml";
    const TESTFILE_2: &str = "/home/derpadi/Documents/Work/Fraunhofer_IGD/ProgressiveIndexingRust/src/rawfile.txt";

    // let mut some_vector:Vec<QsNode> = Vec::new();

    // for i in 0..10
    // {
    //     some_vector.push(QsNode::new(i, i-1));
    // }

    // let x = some_vector.get_mut(5).unwrap();

    // let mut str = my_struct{
    //     a: String::from("a"), 
    //     b: String::from("b"), 
    //     c: 1
    // };

    // println!("Before: {:?}", str);
    // let mut first_mutable_reference = &mut str.a;
    // let second_reference = &str.b;

    // *first_mutable_reference = String::from("abv");

    // println!("After: {:?}", str);


    //let mut index = IncrQsIndex::new();
    //index.init_index(100);
    //some_function(&mut index);

    let mut vector = vec![Foo{},Foo{},Foo{}];
    
    let mut second = &mut vector[1];
    println!("Value: {:?}", &mut vector[0]);
    println!("Value: {:?}", &mut vector[1]);


}


pub fn some_function(qs_index: &mut IncrQsIndex)
{
    let data = qs_index.data.as_mut().unwrap();
    let pointers = qs_index.index.as_mut().unwrap();
    
    let s = qs_index.nodes.len();
    let n = qs_index.get_current_node();

    let input = vec![String::from("A"), String::from("B")];
    //n.split(data.to_vec(), 0, 0);
}


pub fn testing_rqiqst(TESTFILE_2: &str)
{
    let key = "ownername";
    let low = "C";
    let high = "D";
    let mut index = IncrQsIndex::new();
    let qe = QueryEngine::new(TESTFILE_2.to_owned());
    index.init_index(qe.num_rows);

    let r = range_query_incremetal_quicksort_time(key, low, high, &mut index, qe, 100 * MS_TO_NS);
    for v in r
    {
        println!("{:?}", v);
    }
}


pub fn testing_boyer_moore_iterator(TESTFILE_2: &str)
{
    let mut fb = FileBuffer::new(TESTFILE_2, 1024*1024).expect("Test");
    
    //let mut bmi = BoyerMooreBasicIterator::new("cityObjectMember>", &mut fb);

    let qe = QueryEngine::new(TESTFILE_2.to_string());
    let mut bmoo = BoyerMooreAttributeByKeyIterator::new("<gen:stringAttribute name=\"ownername\">", &mut fb, &qe.offset_list);
    
    let mut found_pos:Vec<(String, usize)> = vec![];


    let mut has_next = true;
    while has_next 
    {
        match bmoo.next() {
            Some(val) => {found_pos.push(val)},
            None => {has_next = false},
        }  
    }

    for offset in found_pos
    {
        println!("Found: {:?}", offset);
    }
}


struct NumberSequence 
{
    index: i64
}

impl NumberSequence
{
    fn new(start: i64) -> NumberSequence
    {
        NumberSequence 
        { 
            index: start,
        }
    }
}

impl Iterator for NumberSequence
{
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.index < 50000
        {
            self.index += 1;
            return Some(self.index)
        } else {
            None 
        }
    }
}



#[allow(dead_code)]
fn search(pattern: &str, filename: &str) {
    let file = File::open(filename).expect("Error opening file!");
    let matcher = RegexMatcher::new_line_matcher(&pattern).expect("Error");
    let mut searcher = SearcherBuilder::new()
        .binary_detection(BinaryDetection::quit(b'\x00'))
        .line_number(false)
        .build();

    //let mut printer = StandardBuilder::new()
    //    .color_specs(ColorSpecs::default_with_color())
    //    .build(cli::stdout(if cli::is_tty_stdout() {
    //        ColorChoice::Auto
    //    } else {
    //        ColorChoice::Never
    //    }));

    
    let mut printer = StandardBuilder::new()
        .color_specs(ColorSpecs::default_with_color())
        .build(cli::stdout(if cli::is_tty_stdout() {
            ColorChoice::Auto
        } else {
            ColorChoice::Never
        }));

        
    let start = Instant::now();
    let res = searcher.search_file(&matcher, &file, printer.sink(&matcher));
    let end = start.elapsed();
    println!("Time elapsed: {:?}", end);
}

#[allow(dead_code)]
fn search_alternative(filename: &str, pattern: &str, offset: usize) {
    let mut file = File::open(filename).unwrap();
    let mut buffer = String::new();

    //file.seek(io::SeekFrom::Start(offset as u64))?;
    file.read_to_string(&mut buffer);
}

#[allow(dead_code)]
fn tmp(){
    const TESTFILE_1: &str = "/home/derpadi/Documents/Work/Fraunhofer_IGD/ProgressiveIndexingRust/src/DA12_3D_Buildings_Merged.gml";
    let mut f = file_buffer::FileBuffer::new(TESTFILE_1, 1024*1024).expect("Error!");
    for i in 0..5{
        let s = Instant::now();
        create_offset_list(TESTFILE_1, "offsetList.txt");
        let d = s.elapsed();
        println!("Runtime: {:?}", d);
    }
    let mut load_data = vec![];
    read_into_vec64("offset.bin", &mut load_data);

    //println!("{:?}", load_data);
}


fn create_offset_list(filename: &str, output: &str) {
    let pattern = "cityObjectMember>";
    let matcher = RegexMatcher::new_line_matcher(pattern).unwrap();
    let file = File::open(filename).expect("Error opening file!");
    //let bufReader = BufReader::new(file);
    
    let mut sink = CustomSink{output_filename: "OffsetList.txt".to_string(), counter: 0, data: vec![]};

    
    let mut searcher = SearcherBuilder::new()
    .line_number(true)
    .multi_line(true)
    .build();

    let start = Instant::now();
    searcher.search_file(&matcher, &file, &mut sink).expect("Something went wrong!");
    println!("Time for wrtiing: {:?}", start.elapsed());
    sink.write_data_to_file("offset.bin");
}

struct CustomSink{
    output_filename: String,
    counter: u64,
    data: Vec<u64>
}

fn read_into_vec64(filename: &str, vec: &mut Vec<u64>) {
    let f = File::open(filename).expect("Error opening file!");
    let size = fs::metadata(filename).expect("Error").len();
    for i in (0..size).step_by(8){
        let mut buff = [0u8; 8];
        f.read_exact_at(&mut buff, i).expect("Error reading!");
        vec.push(u64::from_be_bytes(buff));
    }
}

impl CustomSink{
    fn print_num_results(&self){
        println!("{}", self.counter/2);
    }

    fn write_data_to_file(&self, file_name: &str) -> &Vec<u64>{
        let f = File::create("offset.bin").expect("Error creating file!");
        let mut w = BufWriter::new(f);
        let mut buffer:Vec<u8> = vec![];
        for &d in &self.data{
            buffer.extend_from_slice(&d.to_be_bytes());
        }
        w.write_all(&buffer).expect("Error writing!");
        return &self.data;
    }
}

impl Sink for CustomSink {
    type Error = std::io::Error;

    fn matched(&mut self, _: &Searcher, line: &SinkMatch) -> Result<bool, Self::Error> {
        self.counter += 1;
        self.data.push(line.absolute_byte_offset());
        Ok(true)
    }

    fn finish(&mut self, _searcher: &Searcher, _: &SinkFinish) -> Result<(), Self::Error> {
        let mut f = File::create(&mut self.output_filename).expect("Error creating file!");
        
        for i in (1..self.data.len()-1).step_by(2){
            write!(f, "{},{}\n", self.data[i], self.data[i+1]).expect("Error writing to file!");
        }
        println!("Wrote data to file!");
        self.print_num_results();
        Ok(())
    }

}








// First test implementation
#[allow(dead_code)]
struct MyFileReader{
    file: File,
}

impl MyFileReader {
    #[allow(dead_code)]
    fn new(file_path: &str) -> MyFileReader {
        let file = File::open(file_path).expect("Works not!");
        Self { file }
    }
    #[allow(dead_code)]
    fn read_byte_by_byte(&mut self){
        let mut buffer: Vec<u8> = vec![0u8; 20];
        self.file.read_exact(&mut buffer).expect("Nothing works!");
        let str = String::from_utf8_lossy(&buffer);
        println!("{:?}", buffer);
        println!("{}", str);
    }
}


struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

fn is_sorted_tree(root: Option<Box<TreeNode>>, prev_value: &mut Option<i32>) -> bool {
    if let Some(mut node) = root {
        // Check left subtree
        if !is_sorted_tree(node.left.take(), prev_value) {
            return false;
        }
        
        // Check current node's value
        if let Some(prev) = *prev_value {
            if node.value < prev {
                return false;
            }
        }
        *prev_value = Some(node.value);

        // Check right subtree
        if !is_sorted_tree(node.right.take(), prev_value) {
            return false;
        }
    }
    
    true
}

    // const TESTFILE_1: &str = "/home/derpadi/Documents/Work/Fraunhofer_IGD/ProgressiveIndexingRust/src/DA12_3D_Buildings_Merged.gml";
    // const TESTFILE_2: &str = "/home/derpadi/Documents/Work/Fraunhofer_IGD/ProgressiveIndexingRust/src/rawfile.txt";

    // let mut x = MyStruct{
    //     data: None,
    //     index: None,
    //     node_num: 0,
    //     some_val: String::from("Hello")
    // };

    // x.data = Some(vec![1,2,3,4,5]);
    // x.index = Some(vec![2,2,1,3,4]);

    // let p1 = x.data.as_mut().unwrap();
    // let p2 = x.index.as_mut().unwrap();

    // // Construct a sorted binary tree
    // let mut root = Some(Box::new(TreeNode::new(4)));
    // root.as_mut().unwrap().left = Some(Box::new(TreeNode::new(2)));
    // root.as_mut().unwrap().right = Some(Box::new(TreeNode::new(6)));
    // root.as_mut().unwrap().left.as_mut().unwrap().left = Some(Box::new(TreeNode::new(1)));
    // root.as_mut().unwrap().left.as_mut().unwrap().right = Some(Box::new(TreeNode::new(3)));
    // root.as_mut().unwrap().right.as_mut().unwrap().left = Some(Box::new(TreeNode::new(5)));
    // root.as_mut().unwrap().right.as_mut().unwrap().right = Some(Box::new(TreeNode::new(7)));
    // let mut prev_value = None;
    // let is_sorted = is_sorted_tree(root, &mut prev_value);
    // if is_sorted {
    //     println!("The binary tree is sorted.");
    // } else {
    //     println!("The binary tree is not sorted.");
    // }