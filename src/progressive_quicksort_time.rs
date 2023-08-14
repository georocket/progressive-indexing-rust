use std::{time::{Instant, Duration, SystemTime}, alloc::System};

use crate::{qs_index::{self, IncrQsIndex, QsNode}, query_engine::QueryEngine};



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

pub fn range_query_incremetal_quicksort_time(key: &str, low: &str, high: &str, qs_index: &mut IncrQsIndex, budget: i64, query: QueryEngine)
{
    let mut result:Vec<(String,i64)> = Vec::new();
    let index_data = qs_index.data.as_ref().unwrap();
    let pointers = qs_index.index.as_ref().unwrap();

    let timer = Instant::now();
    let max_time = Duration::new(0, 5000);

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

        // Time limited loop ()
        while timer.elapsed() < max_time
        {
            println!("Fill index loop!");

        }
    }
}