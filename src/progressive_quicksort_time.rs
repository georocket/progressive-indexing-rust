#![allow(dead_code)]
#![allow(unused_variables)]

use std::time::{Instant, Duration};

use crate::{qsindex::{qs_index::IncrQsIndex, qs_node::QsNode}, fileaccess::{query_engine::QueryEngine, boyer_moore::BoyerMooreAttributeByKeyIterator}, utility::range_query_sorted_subsequent_value};




pub fn fibonacci(n: i64) -> i64 
{
    if n == 0 || n == 1 {
        n
    } else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}

pub fn range_query_incremental_quicksort_recursive_time(key: String, qs_index: &mut IncrQsIndex, node: usize, low: &str, high: &str, result: &mut Vec<(String, usize)>)
{
    //let node = qs_index.nodes.get(node).unwrap();

    match qs_index.nodes.get(node) {
        Some(node) => {
            if node.sorted
            {
                if String::from(low) <= node.min && String::from(high) >= node.max
                {
                    for i in node.start..node.end
                    {
                        result.push((qs_index.data.as_ref().unwrap()[i as usize].clone(), qs_index.index.as_ref().unwrap()[i as usize]));
                    }
                } else 
                {
                    range_query_sorted_subsequent_value(&qs_index.data.as_ref().unwrap(), &qs_index.index.as_ref().unwrap(), String::from(low), String::from(high), result, node.start as usize, node.end as usize);  
                }
                return;
            }
        },
        None => {},
    }

    match qs_index.nodes.get(node).unwrap().left {
        Some(left) => {
            //let node = qs_index.nodes.get(node).unwrap();
            let pivot = qs_index.nodes.get(node).unwrap().pivot.clone();
            let right = qs_index.nodes.get(node).unwrap().right.unwrap();
            if low < pivot.as_str()
            {
                range_query_incremental_quicksort_recursive_time(key.clone(), qs_index, left as usize, low, high, result);
            }

            if high >= pivot.as_str()
            {
                range_query_incremental_quicksort_recursive_time(key.clone(), qs_index, right as usize, low, high, result);
            }
        },
        None => {
            let node = qs_index.nodes.get_mut(node).unwrap();
            if node.min == node.max {
                node.sorted = true;
            }
        },
    }
}

pub fn range_query_incremetal_quicksort_time(key: &str, low: &str, high: &str, qs_index: &mut IncrQsIndex, mut query: QueryEngine, time_budget: u32) -> Vec<(String, i64)>
{
    let mut result:Vec<(String,i64)> = Vec::new();

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
        Some(_) => { false },
        None => { true },
    };

    if initial_run 
    {
        let index_data = qs_index.data.as_mut().unwrap();
        let pointers = qs_index.index.as_mut().unwrap();

        let node = qs_index.root.as_mut().as_mut().unwrap(); // Borrow

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
        let has_next = true;
        let mut ctr = 0;
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
            ctr += 1;
        }
        println!("Elements added to index: {}", ctr);

        if qs_index.curr_pos == query.num_rows || !has_next
        {
            if node.curr_start < node.curr_end || node.curr_end < 0
            {
                if index_data.get(node.curr_start as usize).unwrap().as_str() < node.pivot.as_str()
                {
                    node.curr_start = node.curr_end;
                } else 
                {
                    node.curr_end = node.curr_start;    
                }
            }
            let (left, right) = node.split(&index_data, 0, -1);
            qs_index.nodes.push(left);
            qs_index.nodes.push(right);
            qs_index.curr_pivot = 0;
        } else 
        {
            let to_scan = query.search_attribute_by_key(String::from(key), qs_index.curr_pos, query.num_rows-1);
            for (idx, val) in to_scan.iter().enumerate()
            {
                if (low..high).contains(&val.as_str())
                {
                    result.push((String::from(val), (qs_index.curr_pos + idx) as i64));
                }
            }
        }
    } else 
    {
        println!("Recursive call!");
        //range_query_incremental_quicksort_recursive_time(key, qs_index, node, low, high, result)
    }

    while (timer.elapsed() < max_time) && qs_index.curr_pivot < qs_index.get_nodes_length()
    {
        println!("There was time left for refinement!");
        let nodes_len = qs_index.nodes.len();
        let node = qs_index.nodes.get_mut(qs_index.curr_pivot).unwrap(); // Get current node

        if node.sorted || node.left == None
        {
            qs_index.curr_pivot += 1;
            continue;
        }

        if node.min == node.max
        {
            node.sorted = true;
            println!("Perform sorted check!");
            qs_index.curr_pivot += 1;
        }
        else 
        {
            println!("Do node budget sort!");
            let index_data = qs_index.data.as_mut().unwrap();
            let pointers = qs_index.index.as_mut().unwrap();

            node.do_budget_sorting(index_data, pointers, 10);

            if node.curr_start >= node.curr_end
            {
                if node.start == node.end -1
                {
                    node.sorted = true;
                    let parent = node.parent;
                    qs_index.sorted_check(match parent {
                        Some(p) => {
                            p as usize
                        },
                        None => {
                            0
                        },
                    }); 
                    qs_index.curr_pivot += 1;
                    continue;
                }

                println!("Check for bad balance!");
                let pos = node.position;
                let (left, right) = node.split(&qs_index.data.as_mut().unwrap(), nodes_len as i64, pos);
                qs_index.nodes.push(left);
                qs_index.nodes.push(right);
                qs_index.curr_pivot += 1;
            }    
        }
    }

    println!("While... unspecific refinement here!");
    println!("Size {}", result.len());
    return result;
}
