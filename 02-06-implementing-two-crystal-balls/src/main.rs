fn two_crystal_balls_naive(xs: &Vec<bool>) -> (i32, i32) {
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

fn two_crystal_balls_idiomatic(xs: &Vec<bool>) -> (i32, i32) {
    // improvements
    //  - cast xs.len() instead of calling f32::sqrt
    let sqrt_xs = (xs.len() as f32).sqrt().floor();
    let mut iterations = 0;
    let mut result = -1;
    let mut safe_index = 0;

    // improvements:
    //  - use .step_by to skip elements
    //  - use .enumerate to get indexes _and_ items
    //      - no need to access item via index in list
    //  - destructure reference in tuple - no need to use * operator to deref
    for (index, &is_broken) in xs
        .iter() // get an iterator
        .step_by(sqrt_xs as usize) // step over values
        .enumerate()
    {
        iterations += 1;

        if is_broken {
            break;
        }

        safe_index = index;
    }

    // improvements
    //  - use .skip to skip all values until safe index
    //  - take only sqrt_xs values
    //  - enumerate so that we have access to index and value
    for (index, &is_broken) in xs
        .iter()
        .skip(safe_index * (sqrt_xs as usize))
        .take(sqrt_xs as usize)
        .enumerate()
    {
        iterations += 1;

        if is_broken {
            result = (safe_index * (sqrt_xs as usize) + index) as i32;
            break;
        }
    }

    (result, iterations)
}

fn two_crystal_balls_functional(xs: &Vec<bool>) -> (i32, i32) {
    let sqrt_xs = (xs.len() as f32).sqrt().floor();
    let (first_iterations, safe_index) = xs
        .iter()
        .enumerate()
        .step_by(sqrt_xs as usize)
        .map(|(_, &is_broken)| is_broken)
        .position(|is_broken| is_broken)
        .map_or((0, 0), |position| {
            (position + 1, (position - 1) * (sqrt_xs as usize))
        });
    let (iterations, result) = xs
        .iter()
        .skip(safe_index)
        .take(sqrt_xs as usize)
        .enumerate()
        .map(|(index, &is_broken)| (index, is_broken))
        .find(|&(_, is_broken)| is_broken)
        .map_or((0, -1), |(index, _)| {
            (
                (first_iterations + index + 1) as i32,
                (safe_index + index) as i32,
            )
        });

    (result, iterations)
}

fn main() {
    let max_ball_height = 1234;
    let xs = (0..10000).map(|x| x > max_ball_height).collect();
    let fns = [
        two_crystal_balls_naive,
        two_crystal_balls_idiomatic,
        two_crystal_balls_functional,
    ];

    for algo in fns {
        let (index, iterations) = algo(&xs);

        println!(
            "
            With max ball height at {max_ball_height},
            index {index} found after {iterations} iterations"
        )
    }
}
