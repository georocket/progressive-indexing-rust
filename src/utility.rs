use std::{cmp::{min,max}, fmt::Display};


pub fn binary_search_gte<T: Ord + Display>(data: &[T], value: &T, start: usize, end: usize) -> usize {
    return match data[start..end].binary_search(value) {
        // Case: Value found
        Ok(mut index) => {
            loop {
                index += 1;
                if index >= end || data[index] != *value {
                    break;
                }
            }

            index = min(index, end);
            if data[index] != *value {
                index -= 1;
            }
            index
        },
        // Case: Value not found
        Err(index) => {
            println!("[Debug]: {} < {}", data[index], *value);
            if data[index] > *value {
                index - 1
            } else {
                index
            }
        },
    };
}

pub fn binary_search_lte<T:Ord + Display>(data: &[T], value: &T, start: usize, end: usize) -> usize
{
    return match data[start..end].binary_search(value) {
        // Case: Value found
        Ok(mut index) => {
            loop {
                index = if index > 0 { index - 1 } else { break; };
                if index < start || data[index] != *value {
                    break;
                }
            }

            if data[index] != *value {
                index += 1;
            }
            index
        },
        // Case: Value not found
        Err(index) => {
            println!("Value: {}", data[index]);
            if data[index] < *value {
                index - 1
            } else {
                index
            }
        },
    };
}

pub fn range_query_sorted_subsequent_value<T: Ord + Clone + Display>(index: &[T], pointers: &[usize], low: T, high: T, results: &mut Vec<(T,usize)>, start: usize, end: usize)
{
    let lower_bound = binary_search_gte(index, &low, start, end);
    let upper_bound = binary_search_lte(index, &high, start, end);

    for i in lower_bound..upper_bound {
        results.push((index[i].clone(), pointers[i]));
    }
}

pub fn test_function<T: Ord + Clone + Display>(index: &[T], low: T, high: T) -> Vec<T>
{
    let mut result: Vec<T> = vec![];

    let lower_bound = binary_search_gte(index, &low, 0, index.len());
    let upper_bound = binary_search_lte(index, &high, 0, index.len());

    println!("Lower bound: {}", lower_bound);
    println!("Upper bound: {}", upper_bound);

    for i in lower_bound..(upper_bound+1) {
        println!("[i]: {}", i);
        result.push(index[i].clone());
    }
    result
}

