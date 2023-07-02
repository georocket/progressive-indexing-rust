mod file_buffer;
use std::fs::File;
use std::io::{Read};
use grep::matcher::Matcher;
use grep::printer::{StandardSink, StandardBuilder};
use grep::regex::RegexMatcher;
use grep::searcher::{SearcherBuilder, SinkMatch};
use grep::searcher::Sink;

fn main() {
    const TESTFILE_1: &str = "/home/derpadi/Documents/Work/Fraunhofer_IGD/ProgressiveIndexingRust/src/DA12_3D_Buildings_Merged.gml";
    let mut f = file_buffer::FileBuffer::new(TESTFILE_1, 1024*1024).expect("Error!");
    //let r = f.get(0).expect("Fehler!");
    let pattern = "<gen:value>";
    let _pattern_size = pattern.chars().count();

    let counter = 0;
    let file_size = f.get_size();

    //for i in 2000000..2000500 {
    //    let val: u8 = f.get(i).expect("Read error!");
    //    let _c = char::from_u32(val as u32).unwrap();
    //    print!("{}", _c)
    //}
    //f.print_buffer_content();

    let pattern = "cityObjectMember>";
    let matcher = RegexMatcher::new_line_matcher(pattern).unwrap();
    let file = File::open(TESTFILE_1).expect("Error opening file!");
    let mut sink = CustomSink{counter: 0};

    let mut searcher = SearcherBuilder::new().line_number(true).build();

    searcher.search_file(&matcher, &file, &mut sink).expect("Something went wrong!");
    
    sink.print_num_results();
}


struct CustomSink{
    counter: u64
}

impl CustomSink {
    fn print_num_results(&mut self){
        println!("{}", self.counter);
    }
}

impl Sink for CustomSink {
    type Error = std::io::Error;

    fn matched(&mut self, _: &grep::searcher::Searcher, line: &SinkMatch) -> Result<bool, Self::Error> {
        //line.line_number();
        self.counter += 1;
        //println!("Found results: {}", self.counter);
        //println!("{}: {}", line.line_number().unwrap(), std::str::from_utf8(line.bytes()).unwrap());
        Ok(true)
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
