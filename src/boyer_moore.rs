use std::collections::{HashMap, LinkedList};

use crate::{file_buffer::FileBuffer, matcher};

pub struct BoyerMoore<'a> {
    bad_char_lookup_table: Vec<HashMap<char, isize>>,
    pattern: &'a str
}


impl<'a> BoyerMoore<'a> {


    pub fn new(pattern: &'a str) -> Result<Self, std::io::Error> {
        Ok(Self { 
            bad_char_lookup_table: BoyerMoore::bad_char_lookup_table(pattern),
            pattern
        })
    }

    pub fn bad_char_lookup_table(pattern: &str) -> Vec<HashMap<char, isize>> {
        let mut char_map: HashMap<char, isize> = HashMap::new();
        let mut result = vec![HashMap::new(); pattern.len()];

        for (i, c) in pattern.chars().enumerate() {
            for (key, value) in &char_map {
                if *key != c {
                    result[i].insert(*key, i as isize - value);
                }
            }
            char_map.insert(c, i as isize);
        }
        result
    }

    pub fn boyer_moore_bad_char_only(&self, file: &mut FileBuffer) -> Result<LinkedList<isize>, std::io::Error>{
        let file_size = file.get_size();
        let pattern_size = self.pattern.len();
        let mut result:LinkedList<isize> = LinkedList::new();
        let mut num_skipped = 0;
        let mut i:u64 = 0;
        while i < file_size {
            if pattern_size <= (file_size - i) as usize {
                let mut j = (pattern_size - 1) as isize;
                while j >= 0 {
                    let t = file.get(i+j as u64).unwrap() as char;
                    let p = self.pattern.as_bytes()[j as usize] as char;

                    if t != p {
                        let contains = &self.bad_char_lookup_table[j as usize].contains_key(&t);
                        let skips = if !contains {
                            j+1 as isize
                        } else {
                            self.bad_char_lookup_table[j as usize][&t]
                        };
                        i += (skips-1) as u64;
                        j-= 1;
                        num_skipped += skips;
                        break;
                    }
                    if(j == 0) {
                        result.push_back(i as isize);
                        i += (pattern_size-1) as u64;
                    }
                    j-= 1;
                }
            }
            i += 1;
        }
        println!("output: {}", num_skipped);
        Ok(result)
    }

    pub fn scan_attribute_by_key(&self, file: &mut FileBuffer, offset_list: &Vec<(u64, u64)>, from: usize, to: usize) -> Vec<String> {
        let file_size = file.get_size();                                
        let pattern_size = self.pattern.len();
        //let result = vec![String::from(""); to - from];
        let mut result = vec![];
    
        //let range = offset_list[from].0..offset_list[to].0;

        let mut i = 0;
        while i < file_size {
            if pattern_size < (file_size - 1) as usize {
                let mut j = (pattern_size - 1) as isize;
                while j >= 0 {
                    let t = file.get(i+j as u64).unwrap() as char;
                    let p = self.pattern.as_bytes()[j as usize] as char;

                    if t != p {
                        let contains = &self.bad_char_lookup_table[j as usize].contains_key(&t);
                        let skips = if !contains {
                            (j+1) as usize
                        } else {
                            self.bad_char_lookup_table[j as usize][&t] as usize
                        };
                        i += (skips-1) as u64;
                        j -= 1;
                        break;
                    }
                    if j == 0 {
                        let mut matcher = matcher::SimpleMatcher::new("<gen:value>");
                        let mut result_value = String::from("");
                        i += pattern_size as u64;

                        while !matcher.step(&(file.get(i).unwrap() as char)){ i += 1 }
                        
                        let index = 0;
                        let mut act_char = file.get(i).unwrap() as char;
                        while(act_char != '<') {
                            result_value.push(act_char);
                            i += 1;
                            act_char = file.get(i).unwrap() as char;
                        }
                        result.push(result_value);
                    }
                    j -= 1;
                }
            }
            i += 1;
        }
        result
    }
}