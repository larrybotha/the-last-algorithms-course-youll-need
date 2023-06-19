fn binary_search<T: Ord>(haystack: Vec<T>, needle: T) -> bool {
    use std::cmp::Ordering;

    let mut result = false;
    let mut min_index = 0;
    let mut max_index = haystack.len();

    while min_index < max_index {
        let mid_index = min_index + (max_index - min_index) / 2;
        let mid_value = &haystack[mid_index];

        match mid_value.cmp(&needle) {
            Ordering::Equal => {
                result = true;
                break;
            }
            Ordering::Less => min_index = mid_index + 1,
            _ => max_index = mid_index,
        }
    }

    result
}

fn main() {
    let xs = [1, 2, 6, 8, 234, 24524, 123157];
    let search_values = [6, 12, 24524];

    for value in search_values {
        let result = binary_search(xs.into(), value);
        println!("is {value} in {xs:?}: {result}");
    }
}
