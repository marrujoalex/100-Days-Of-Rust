// initial thoughts
// this is a two-pointer problem
// track left and check if right > left, then ++ the progress counter

fn progress_checker(runs: Vec<u8>) -> u8 {
    let mut progress_count = 0;
    let mut left = 0;
    for right in 1..runs.len() {
        if runs.get(right) > runs.get(left) {
            progress_count = progress_count + 1;
        }

        left = left + 1;
    }

    progress_count
}

fn main() {
    let runs = vec![1,2,3,2];
    let result = progress_checker(runs);

    println!("{}", result)
}
