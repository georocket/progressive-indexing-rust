mod file_buffer;
use std::fs::File;
use std::io::{Read};

fn main() {
    const TESTFILE_1: &str = "/home/derpadi/Documents/Work/Fraunhofer_IGD/ProgressiveIndexingRust/src/DA12_3D_Buildings_Merged.gml";
    let mut f = file_buffer::FileBuffer::new(TESTFILE_1, 1024*1024).expect("Error!");
    //let r = f.get(0).expect("Fehler!");
    let pattern = "<gen:value>";
    let pattern_size = pattern.chars().count();

    let counter = 0;
    let file_size = f.get_size();

    for i in 0..file_size {
        let read_char = char::from_u32(f.get(i).unwrap() as u32).unwrap();
        if i % 9000000 == 0{
            println!("Line: {}\t%", ((i as f64) / (file_size as f64)) * 100.0);
        }
    }

    println!("{}", counter);

    for i in 2000000..2000500 {
        let val: u8 = f.get(i).expect("Read error!");
        let c = char::from_u32(val as u32).unwrap();
        //print!("{}", c)
    }
    //f.print_buffer_content();
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
