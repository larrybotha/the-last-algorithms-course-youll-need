fn two_crystal_balls(xs: &Vec<bool>) -> (i32, i32) {
    let sqrt_xs = f32::sqrt(xs.len() as f32).floor() as usize;
    let (mut safe_index, mut result, mut iterations) = (0, -1, 0);
    let indices = (0..(sqrt_xs)).map(|x| x * sqrt_xs);

    for index in indices {
        iterations += 1;

        if xs[index] {
            break;
        }

        safe_index = index;
    }

    for (index, is_broken) in xs.iter().skip(safe_index).take(sqrt_xs).enumerate() {
        iterations += 1;

        if *is_broken {
            result = (safe_index + index) as i32;
            break;
        }
    }

    (result, iterations)
}

fn main() {
    let max_ball_height = 1234;
    let xs = (0..10000).map(|x| x > max_ball_height).collect();
    let (index, iterations) = two_crystal_balls(&xs);

    println!(
        "
            With max ball height at {max_ball_height},
            index {index} found after {iterations} iterations
        "
    )
}
