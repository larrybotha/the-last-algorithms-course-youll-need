use std::cmp::Ordering;

fn bubble_sort_naive<T: PartialOrd + Copy>(xs: &mut Vec<T>) {
    let length = xs.len();

    for i in 0..length {
        for j in 0..(length - 1 - i) {
            let curr = xs[j];
            let next = xs[j + 1];

            if curr > next {
                xs[j + 1] = curr;
                xs[j] = next;
            }
        }
    }
}

// improvements:
//  - use [T] instead of Vec<T> - this allows for passing in either
//      arrays or vectors without changing or casting any types
//  - use Ord instead of PartialOrd - PartialOrd is useful if we're
//      dealing with types that have indeterminate values when compared
//      to other values, such as f32::NAN. For integers, strings, etc,
//      we're always guaranteed a comparison, so we can use Ord
fn bubble_sort_improved<T: Ord>(xs: &mut [T]) {
    let length = xs.len();

    for i in 0..length {
        for j in 0..(length - 1 - i) {
            if xs[j] > xs[j + 1] {
                // improvements:
                //  - use Rust's slice::swap method to swap the two values;
                //      no need for Copy
                xs.swap(j, j + 1)
            }
        }
    }
}

fn bubble_sort_partial_ord<T: PartialOrd>(xs: &mut [T]) {
    let length = xs.len();

    for i in 0..length {
        for j in 0..(length - 1 - i) {
            match xs[j].partial_cmp(&xs[j + 1]) {
                None => xs.swap(j, j + 1),
                Some(Ordering::Greater) => xs.swap(j, j + 1),
                _ => (),
            }
        }
    }
}

fn bubble_sort_functional<T: Ord>(xs: &mut [T]) {
    let length = xs.len();

    (0..length)
        .map(|i| 0..(length - 1 - i)) // build ranges for nested loops
        .for_each(|js| {
            js.for_each(|j| {
                if xs[j] > xs[j + 1] {
                    xs.swap(j, j + 1)
                }
            })
        });
}

fn assert_order<T: PartialOrd>(xs: &[T]) {
    for i in 0..(xs.len() - 1) {
        let curr = &xs[i];
        let next = &xs[i + 1];

        match curr.partial_cmp(next) {
            None => (),
            Some(Ordering::Less) => assert!(curr <= next),
            _ => (),
        }
    }
}

fn main() {
    let mut xs = vec![2, 1, 123, 4, 613, 23];
    let mut ys = vec!["Z", "foo", "1", "A", "quux", "-1", "a"];
    let mut zs = vec![2.1, 1.0, f32::NAN, 123.2, 4.2, 613.0, 23.0];
    let mut bs = ys.to_vec();

    println!("xs before: {xs:?}");
    bubble_sort_naive(&mut xs);
    assert_order(&xs);
    println!("xs after: {xs:?}\n");

    println!("ys before: {ys:?}");
    bubble_sort_improved(&mut ys);
    assert_order(&ys);
    println!("ys after: {ys:?}\n");

    println!("zs before: {zs:?}");
    bubble_sort_partial_ord(&mut zs);
    assert_order(&zs);
    println!("zs after: {zs:?}\n");

    println!("bs before: {bs:?}");
    bubble_sort_functional(&mut bs);
    assert_order(&bs);
    println!("bs after: {bs:?}\n");
}
