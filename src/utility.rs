use std::fmt::Display;

// Finding lower bound
pub fn binary_search_gte<T: Ord + Display>(data: &[T], value: &T, start: usize, end: usize) -> usize {
    return match data[start..end].binary_search(value) {
        // Case: Value found
        Ok(mut index) => {
            loop {
                if index <= start {
                    if data[index] != *value {
                        index += 1;
                    }
                    break;
                }

                if data[index] == *value {
                    index -= 1;
                } else {
                    index += 1;
                    break;
                }
            }
            index
        },
        // Case: Value not found
        Err(index) => {
            index
        },
    };
}

// Finding upper bound
pub fn binary_search_lte<T:Ord + Display>(data: &[T], value: &T, start: usize, end: usize) -> usize
{
    return match data[start..end].binary_search(value) {
        // Case: Value found
        Ok(mut index) => {
            loop {
                if index >= end {
                    break;
                }

                if data[index] == *value {
                    index += 1;
                } else {
                    break;
                }
            }
            index
        },
        // Case: Value not found
        Err(index) => {
            index
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

    //println!("Lower bound: {}", lower_bound);
    //println!("Upper bound: {}", upper_bound);

    for i in lower_bound..(upper_bound) {
        result.push(index[i].clone());
    }
    result
}

pub fn linear_check<T: Ord + Clone + Display>(index: &[T], low: T, high: T) -> Vec<T>
{
    let mut result: Vec<T> = vec![];
    for v in index {
        if *v >= low && *v <= high {
            result.push(v.clone());
        }
    }
    result
}

