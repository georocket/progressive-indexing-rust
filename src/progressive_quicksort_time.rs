use std::{time::{Instant, Duration, SystemTime}, alloc::System};

use crate::{qs_index::{self, IncrQsIndex, QsNode}, query_engine::QueryEngine, boyer_moore::BoyerMooreAttributeByKeyIterator, file_buffer};



pub fn fibonacci(n: i64) -> i64 
{
    if n == 0 || n == 1 {
        n
    } else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}

pub fn range_query_incremental_quicksort_recursive_time(key: String, qs_index: IncrQsIndex, node: QsNode, low: &str, high: &str, result: Vec<(String, usize)>)
{

}

pub fn range_query_incremetal_quicksort_time(key: &str, low: &str, high: &str, qs_index: &mut IncrQsIndex, budget: i64, mut query: QueryEngine, time_budget: u32)
{
    let mut result:Vec<(String,i64)> = Vec::new();
    let mut index_data = qs_index.data.as_mut().unwrap();
    let mut pointers = qs_index.index.as_mut().unwrap();

    let timer = Instant::now();
    let max_time = Duration::new(0, time_budget);

    if qs_index.root.as_ref().as_ref().unwrap().sorted
    {
        // Perform range query on index
        qs_index.nodes.clear();

        // Filter result (Z-Order curve needs it)
    }

    let initial_run = match qs_index.root.as_ref().as_ref().unwrap().left 
    {
        Some(_) => { true },
        None => { false },
    };

    if initial_run 
    {
        let node = qs_index.root.as_mut().as_mut().unwrap();

        let piv = node.pivot.as_str();
        if low < piv 
        {
            for i in 0..node.curr_start
            {
                if (low..high).contains(&index_data[i as usize].as_str())
                {
                    let value = index_data[i as usize].clone();
                    let position = pointers[i as usize] as i64;
                    result.push((value,position));
                }
            }
        }
        if high >= piv 
        {
            for i in node.curr_end..query.num_rows as i64
            {
                if (low..high).contains(&index_data[i as usize].as_str())
                {
                    let value = index_data[i as usize].clone();
                    let position = pointers[i as usize] as i64;
                    result.push((value,position));
                }
            }
        }

        let ctr = 0;
        let start_idx = qs_index.curr_pos;

        let pattern = format!("<gen:stringAttribute name=\"{}\">", key);
        let mut rows = BoyerMooreAttributeByKeyIterator::new(&pattern, &mut query.file_buffer, &query.offset_list);

        if qs_index.first_run
        {
            node.pivot = String::from("B");
            qs_index.first_run = false;
        }

        // Time limited loop ()
        let mut has_next = true;
        while timer.elapsed() < max_time
        {
            let next_val = match rows.next() {
                Some(val) => {val},
                None => {break},
            };

            if (low..high).contains(&next_val.0.as_str())
            {
                result.push((next_val.0.clone(), 0));
            }

            let add_to_index = next_val.0.as_str() >= node.pivot.as_str();

            if add_to_index
            {
                //index_data = next_val;
                index_data.insert(node.curr_end as usize, next_val.0);
                pointers.insert(node.curr_end as usize, next_val.1);

                node.curr_end -= 1;
            } else
            {
                index_data.insert(node.curr_start as usize, next_val.0);
                pointers.insert(node.curr_start as usize, next_val.1);

                node.curr_start += 1;
            }

            qs_index.curr_pos = std::cmp::max(qs_index.curr_pos + 1, 0 + 1);

            println!("Fill index loop!");

        }
    }
}