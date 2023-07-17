use std::{fs::File, path::Path, io::{BufWriter, BufRead, BufReader}};

use grep::{regex::RegexMatcher, searcher::{Sink, Searcher, SinkMatch, SinkFinish, SearcherBuilder}};
use std::io::Write;




struct QueryEngine {
    filename: String,
    file: File,
    file_format: FileFormat,
    offset_list: Vec<(u64, u64)>,
    num_rows: usize
}

impl QueryEngine {
    fn new(&mut self, filename: String) -> QueryEngine {
        let file = File::open(&filename).expect("Error opening file!");
        let file_format = FileFormat::CityGML; // At the moment hard coded
        let mut offset_list:Vec<(u64, u64)> = Vec::new();
        let num_rows = 0;

        // Initialize Offset List
        let offset_list_filename = filename.clone() + ".qry";
        if !Path::new(&offset_list_filename).exists() {
            // TODO: Create offset-list and store it to file
            let matcher = RegexMatcher::new_line_matcher("cityObjectMember>").unwrap();
            let mut sink = OffsetSink{output_filename: offset_list_filename, counter: 0, first: 0, data: &mut offset_list};
            let mut searcher = SearcherBuilder::new()
                                            .line_number(true)
                                            .build();
            searcher.search_file(&matcher, &file, &mut sink).expect("Error creating Offset-List!");
        } else {
            self.read_offset_list();
        }

        QueryEngine {
            filename,
            file: file, 
            file_format: file_format, 
            offset_list: offset_list,
            num_rows: num_rows
        }

    }

    fn read_offset_list(&mut self) {
        let offset_list_filename = self.filename.clone() + ".qry";
        let file = File::open(offset_list_filename).expect("Error opening offset-list file!");
        let r = BufReader::new(file);

        for l in r.lines() {
            let line = l.expect("Error reading line!");
            let values: Vec<u64> = line
                                        .split(',')
                                        .map(
                                            |x| x.trim().parse::<u64>().expect("Error parsing value")
                                        ).collect();
            if values.len() != 2 {
                println!("Incorrect offset-list file!");
            }
            let first = values[0];
            let second = values[1];
            
            self.offset_list.push((first, second));
        }
    }
}

struct OffsetSink<'a>{
    output_filename: String,
    counter: u64,
    first: u64,
    data: &'a mut Vec<(u64,u64)>
}

impl OffsetSink<'_>{
    fn print_num_results(&self){
        println!("{}", self.counter/2);
    }

    //fn write_data_to_file(&self, file_name: &str) -> &Vec<(u64,u64)>{
    //    let f = File::create("offset.bin").expect("Error creating file!");
    //    let mut w = BufWriter::new(f);
    //    let mut buffer:Vec<u8> = vec![];
    //    for d in self.data{
    //        buffer.extend_from_slice(&d.0.to_be_bytes());
    //        buffer.extend_from_slice(&d.1.to_be_bytes());
    //    }
    //    w.write_all(&buffer).expect("Error writing!");
    //    return &self.data;
    //}
}

impl Sink for OffsetSink<'_> {
    type Error = std::io::Error;

    fn matched(&mut self, _: &Searcher, line: &SinkMatch) -> Result<bool, Self::Error> {
        // Case: 
        if self.counter % 2 == 0 {
            self.first = line.absolute_byte_offset();
        } else {
            self.data.push((self.first,line.absolute_byte_offset()));
        }
        self.counter += 1;
        Ok(true)
    }

    fn finish(&mut self, _searcher: &Searcher, _: &SinkFinish) -> Result<(), Self::Error> {
        let mut f = File::create(&mut self.output_filename).expect("Error creating file!");
        
        for i in (1..self.data.len()-1).step_by(2){
            write!(f, "{},{}\n", self.data[i].0, self.data[i].1).expect("Error writing offsetlist!");
        }
        println!("Wrote data to file!");
        self.print_num_results();
        Ok(())
    }

}

enum FileFormat {
    CityGML, CityJSON
}