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
    let result:Vec<(String,i64)> = Vec::new();
    let index_data = qs_index.data.expect("Error!");
    let pointers = qs_index.index.expect("Error!");

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
        
    }
}