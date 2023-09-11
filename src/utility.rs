
fn binary_search<T: Ord>(data: &[T], value: &T, start: usize, end: usize) -> Option<usize> {
    let mut left = start;
    let mut right = end;

    while left <= right {
        let mid = left + (right - left) / 2;

        if data[mid] == *value {
            return Some(mid);
        } else if data[mid] < *value {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None
}

fn binary_search_gte<T: Ord>(data: &[T], value: &T, start: usize, end: usize) -> usize {
    let mut e = match data[start..end].binary_search(value) {
        Ok(index) => index,
        Err(index) => index,
    };

    if e < 0 {
        e = e.wrapping_neg() - 1;
    } else {
        while e >= start && data[e] == *value {
            e -= 1;
        }
        e += 1;
    }

    e
}

