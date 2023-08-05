

pub fn binary_search_for_offset_range(offset: u64, offset_list: &Vec<(u64, u64)>) -> usize
{
    let size = offset_list.len();
    let mut act_pos = size/2;
    let mut found = false;

    let mut lower = 0;
    let mut upper = size;
    while(!found) {
        act_pos = lower + (upper - lower)/2;
        let actLower = offset_list[act_pos].0;
        let actUpper = if act_pos < (size-1) {offset_list[act_pos+1].0} else {offset_list[size-1].1};
        if offset >= actLower && offset < actUpper {
            found = true;
            continue;
        }
        if offset < actLower {
            upper -= (upper - lower) / 2;
        }
        if offset >= actUpper {
            lower += (upper - lower) / 2;
        }
        println!("Position: {}", act_pos);
        println!("L: {}, U: {}", actLower, actUpper);
        println!("Val = {}", offset);
    }
    act_pos
}