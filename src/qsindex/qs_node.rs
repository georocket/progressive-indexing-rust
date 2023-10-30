#[derive(Debug)]

/// 
/// Struct representing a node (part of the index) and its state regarding the sortedness
pub struct QsNode<T: Ord>
{
    pub position: i64,
    pub parent: Option<i64>,
    pub sorted: bool,
    pub start: i64,
    pub end: i64,
    pub curr_start: i64,
    pub curr_end: i64,
    pub pivot: T,
    pub left: Option<i64>,
    pub right: Option<i64>,
    pub min: T,
    pub max: T,
    already_rebalanced: bool,
    pub single_value_node: bool,
}

impl QsNode<String>
{
    ///
    /// * `pos` - Position of the node (index of node in list)
    /// * `parent` - Index of parent node
    pub fn new(pos: i64, parent: Option<i64>) -> Self
    {
        Self {
            position: pos,
            parent,
            sorted: false,
            start: -1, 
            end: -1, 
            curr_start: -1, 
            curr_end: -1, 
            pivot: String::from("Empty"), 
            left: None, 
            right: None, 
            min: String::from("MIN"), 
            max: String::from("MAX"), 
            already_rebalanced: false,
            single_value_node: false
        }
    }

    ///
    /// Resetting pointers representing progress of quicksort algorithm within node
    pub fn reset_curr_pointers(&mut self)
    {
        self.curr_start = self.start;
        self.curr_end = self.end - 1;
    }

    ///
    /// Splitting a node into two nodes (subarrays) following the quicksort algorithm
    /// 
    /// * `index` - Data-Vector of index
    /// * `pos` - Position of the node to be split
    /// * `parent` - Index of the parent node
    pub fn split(&mut self, index: &Vec<String>, pos: i64, parent: i64) -> (QsNode<String>, QsNode<String>)
    {
        self.left = Some(pos);
        self.right = Some(pos + 1);

        let mut left = QsNode::new(pos, Some(parent));
        left.start = self.start;
        left.end = if index[self.curr_start as usize] < self.pivot { self.curr_start + 1 } else { self.curr_start };
        left.curr_start = left.start;
        left.curr_end = left.end - 1;
        let idx = ((left.curr_start + left.curr_end)/2) as usize;
        left.pivot = index[idx].clone();
        left.min = self.min.clone();
        left.max = self.pivot.clone();

        let mut right = QsNode::new(pos + 1, Some(parent));
        right.start = left.end;
        right.end = self.end;
        right.curr_start = right.start;
        right.curr_end = right.end - 1;
        right.pivot = index[((right.curr_start + right.curr_end)/2) as usize].clone();
        right.min = self.pivot.clone();
        right.max = self.max.clone();

        (left, right)
    }

    ///
    /// Method for performing quicksort steps (according to a budget)
    /// 
    /// * `index` - Data-Vector of index
    /// * `pointers` - Pointer-Vector (Referring to objects within the raw file) of index
    /// * `max_time` - Maximum time budget for sorting
    pub fn do_budget_sorting(&mut self, index: &mut Vec<String>, pointers: &mut Vec<usize>, max_time: i64)
    {
        let time_constraint = true; // Place-holder for time measurement
        
        while self.curr_start < self.curr_end && time_constraint
        {
            let start = index[self.curr_start as usize].clone();
            let end = index[self.curr_end as usize].clone();

            let start_pointer = pointers[self.curr_start as usize];
            let end_pointer = pointers[self.curr_end as usize];

            let start_swap = start >= self.pivot;
            let end_swap = end < self.pivot;
            let swap = start_swap && end_swap;

            if swap
            {
                index[self.curr_start as usize] = end;
                index[self.curr_end as usize] = start;

                pointers[self.curr_start as usize] = end_pointer;
                pointers[self.curr_end as usize] = start_pointer;

                self.curr_start += 1;
                self.curr_end -= 1;
            } else 
            {
                if !start_swap
                {
                    self.curr_start += 1;
                }
                if !end_swap
                {
                    self.curr_end -= 1;
                }    
            }
        }
    }

    ///
    /// Checking if a node is badly balanced (one subarray is empty)
    /// 
    /// * `index` - Data-Vector of index
    pub fn check_for_bad_balance(&mut self, index: &Vec<String>) -> bool
    {
        // Node has not a bad balance
        if !(self.curr_start == self.start || self.curr_end == self.end -1)
        {
            return false;
        }

        let current_pivot = self.pivot.clone();
        let mut single_value_node = true;

        for i in self.start..self.end
        {
            if index[i as usize] != current_pivot
            {
                single_value_node = false;
                self.pivot = index[i as usize].clone();
                self.reset_curr_pointers();
                break;
            }
        }

        if single_value_node
        {
            self.sorted = true;
            self.single_value_node = true;
            return false;
        }
        return true;
    }

    pub fn print_node(&self)
    {
        println!("Parent[{:?}]", self.parent);
        println!("Node[{}]:", self.position);
        println!("L[{:?}]; R[{:?}]", self.left, self.right);
        println!("##############################################");
    }
}
