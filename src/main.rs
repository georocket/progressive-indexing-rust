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
use std::time::Instant;
use std::vec;
use grep::printer::{StandardBuilder, ColorSpecs};
use grep::regex::RegexMatcher;
use grep::searcher::{SearcherBuilder, SinkMatch, SinkFinish, BinaryDetection};
use grep::searcher::Sink;
use grep::searcher::Searcher;
use grep::cli;
use termcolor::ColorChoice;

use crate::boyer_moore::BoyerMoore;
use crate::query_engine::QueryEngine;

fn main() {
    const TESTFILE_1: &str = "/home/derpadi/Documents/Work/Fraunhofer_IGD/ProgressiveIndexingRust/src/DA12_3D_Buildings_Merged.gml";
    const TESTFILE_2: &str = "/home/derpadi/Documents/Work/Fraunhofer_IGD/ProgressiveIndexingRust/src/rawfile.txt";

    //let mut fb = file_buffer::FileBuffer::new(TESTFILE_2, 1024*1024).unwrap();


    //let mut qe = QueryEngine::new(String::from(TESTFILE_2));

    //let o = String::from("ownername");
    //let res = qe.search_attribute_by_key(o, 0, 25);
    //println!("Result: {:?}", res);
    //let mut v:Vec<(u64, u64)> = Vec::new();
    //let x = String::from("/home/derpadi/Documents/Work/Fraunhofer_IGD/ProgressiveIndexingRust/src/rawfile.txt");
    println!("Program ran!");

    for i in 0..40
    {
        println!("Fibonacci-Sequence [{}]: {}", i, progressive_quicksort_time::fibonacci(i));
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
