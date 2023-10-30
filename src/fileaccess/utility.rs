#![allow(dead_code)]
#![allow(unused_variables)]

///
/// Function performing binary search on the offset list (adapted to the structure of the offset list).
/// We provide a found offset and the function outputs the position in the offset list where to find the object boundries.
/// 
/// * `offset` - Offset to search for
/// * `offset_list` - Offset list to search in
pub fn binary_search_for_offset_range(offset: u64, offset_list: &Vec<(u64, u64)>) -> usize
{
    let size = offset_list.len();
    let mut act_pos = size/2;
    let mut found = false;

    let mut lower = 0;
    let mut upper = size;
    while !found {
        act_pos = lower + (upper - lower)/2;
        let act_lower = offset_list[act_pos].0;
        let act_upper = if act_pos < (size-1) {offset_list[act_pos+1].0} else {offset_list[size-1].1};
        if offset >= act_lower && offset < act_upper {
            found = true;
            continue;
        }
        if offset < act_lower {
            upper -= (upper - lower) / 2;
        }
        if offset >= act_upper {
            lower += (upper - lower) / 2;
        }
        //println!("Position: {}", act_pos);
        //println!("L: {}, U: {}", actLower, actUpper);
        //println!("Val = {}", offset);
    }
    act_pos
}