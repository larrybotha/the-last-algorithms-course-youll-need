fn two_crystal_balls_naive(xs: &Vec<bool>) -> (i32, i32) {
    let sqrt_xs = f32::sqrt(xs.len() as f32).floor() as usize;
    let (mut safe_index, mut break_index, mut iterations) = (0, -1, 0);
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
            break_index = (safe_index + index) as i32;
            break;
        }
    }

    (break_index, iterations)
}

fn two_crystal_balls_improved(xs: &Vec<bool>) -> (i32, i32) {
    // improvements
    //  - cast xs.len() instead of calling f32::sqrt
    let sqrt_xs = (xs.len() as f32).sqrt().floor();
    let mut iterations = 0;
    let mut break_index = -1;
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

        safe_index = index * (sqrt_xs as usize)
    }

    // improvements
    //  - use .skip to skip all values until safe index
    //  - take only sqrt_xs values
    //  - enumerate so that we have access to index and value
    for (index, &is_broken) in xs
        .iter()
        .skip(safe_index)
        .take(sqrt_xs as usize)
        .enumerate()
    {
        iterations += 1;

        if is_broken {
            break_index = (safe_index + index) as i32;
            break;
        }
    }

    (break_index, iterations)
}

fn two_crystal_balls_functional(xs: &Vec<bool>) -> (i32, i32) {
    let sqrt_xs = (xs.len() as f32).sqrt().floor();
    // improvements::
    //  - use .position to get index of broken ball
    //  - use .map_or to handle the returned Option
    let (first_iterations, safe_index) = xs
        .iter()
        .step_by(sqrt_xs as usize)
        .position(|&is_broken| is_broken) // first ball breaks here
        .map_or((0, 0), |position| {
            // if position is 0, we did 1 iteration, etc.
            let iterations = position + 1;
            let index = match position {
                0 => position,
                // the previous position was safe, if the position is greater than 0
                _ => (position - 1) * sqrt_xs as usize,
            };

            (iterations, index)
        });
    // improvements:
    //  - use .map after .enumerate to drop one level of reference
    //  - use .find, dereferencing the tuple, to find the first location
    //      of the break. This consumes our second breakable ball
    //  - use .map_or to handle the Option returned by .find
    let (iterations, break_index) = xs
        .iter()
        .skip(safe_index)
        .enumerate()
        .map(|(index, &is_broken)| (index, is_broken))
        .find(|&(_, is_broken)| is_broken) // second ball breaks here
        .map_or((0, -1), |(index, _)| {
            (
                (first_iterations + index + 1) as i32,
                (safe_index + index) as i32,
            )
        });

    (break_index, iterations)
}

fn main() {
    let max_ball_height = 1234;
    let xs = (0..10000).map(|x| x > max_ball_height).collect();
    let fns = [
        two_crystal_balls_naive,
        two_crystal_balls_improved,
        two_crystal_balls_functional,
    ];

    for algo in fns {
        let (index, iterations) = algo(&xs);

        println!(
            "
            With max ball height at {max_ball_height},
            balls break at {index},
            found after {iterations} iterations"
        )
    }
}
