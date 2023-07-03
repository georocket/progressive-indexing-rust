mod file_buffer;
use std::fs::{File, self};
use std::io::{Read, Write, Seek};
use std::os::unix::prelude::FileExt;
use std::time::Instant;
use std::vec;
use grep::printer::{StandardSink, StandardBuilder};
use grep::regex::RegexMatcher;
use grep::searcher::{SearcherBuilder, SinkMatch, SinkFinish};
use grep::searcher::Sink;
use grep::searcher::Searcher;
use std::{mem, slice};

fn main() {
    const TESTFILE_1: &str = "/home/derpadi/Documents/Work/Fraunhofer_IGD/ProgressiveIndexingRust/src/DA12_3D_Buildings_Merged.gml";
    let mut f = file_buffer::FileBuffer::new(TESTFILE_1, 1024*1024).expect("Error!");
    //for i in 0..5{
    //    let s = Instant::now();
    //    create_offset_list(TESTFILE_1, "offsetList.txt");
    //    let d = s.elapsed();
    //    println!("Runtime: {:?}", d);
    //}
    let mut load_data = vec![0u64];
    read_into_vec64("offset.bin", &mut load_data);
    println!("{:?}", load_data);
}


fn create_offset_list(filename: &str, output: &str) {
    let pattern = "cityObjectMember>";
    let matcher = RegexMatcher::new_line_matcher(pattern).unwrap();
    let file = File::open(filename).expect("Error opening file!");
    let mut sink = CustomSink{output_filename: "OffsetList.txt".to_string(), counter: 0, data: vec![0u64, 0]};

    let mut searcher = SearcherBuilder::new().line_number(true).build();

    searcher.search_file(&matcher, &file, &mut sink).expect("Something went wrong!");
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
    fn print_num_results(&mut self){
        println!("{}", self.counter/2);
    }

    fn write_data_to_file(&mut self, file_name: &str) -> &Vec<u64>{
        let mut f = File::create("offset.bin").expect("Error creating file!");
        for &d in &self.data{
            f.write_all(&d.to_be_bytes());
        }
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
