use std::collections::HashMap;
use std::cell::RefCell;

pub fn solve(x: u8, y: u8, max_x: u8, max_y: u8, value_cache: &RefCell<HashMap<(u8, u8), u64>>) -> u64 {
    get_number_of_paths_from(x, y, max_x, max_y, &value_cache)
}

fn get_number_of_paths_from(x: u8, y: u8, max_x: u8, max_y: u8, value_cache: &RefCell<HashMap<(u8, u8), u64>>) -> u64 {
    if x == max_x && y == max_y {
        1
    } else if x < max_x && y < max_y {
        determine_output(x + 1, y, max_x, max_y, &value_cache) + determine_output(x, y + 1, max_x, max_y, &value_cache)
    } else if x < max_x && y == max_y {
        determine_output(x + 1, y, max_x, max_y, &value_cache)
    } else {
        determine_output(x, y + 1, max_x, max_y, &value_cache)
    }
}

fn determine_output(x: u8, y: u8, max_x: u8, max_y: u8, value_cache: &RefCell<HashMap<(u8, u8), u64>>) -> u64 {
    let cached_value = value_cache.borrow().get(&(x, y)).cloned();
    if let Some(value) = cached_value {
        value
    } else {
        let value = get_number_of_paths_from(x, y, max_x, max_y, &value_cache);
        value_cache.borrow_mut().insert((x, y), value);
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_correct_result() {
        let value_cache = RefCell::new(HashMap::new());
        assert_eq!(solve(0, 0, 20, 20, &value_cache), 137846528820);
    }
}
