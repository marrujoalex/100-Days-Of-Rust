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

/*
* Examples from project README.md
*   progressDays([3, 4, 1, 2]) ➞ 2
*   progressDays([10, 11, 12, 9, 10]) ➞ 3
*   progressDays([6, 5, 4, 3, 2, 9]) ➞ 1
*   progressDays([9, 9])  ➞ 0
*/

fn main() {
    let runs = vec![3, 4, 1, 2];
    // let runs = vec![10, 11, 12, 9, 10];
    // let runs = vec![6, 5, 4, 3, 2, 9];
    // let runs = vec![9, 9];

    let result = progress_checker(runs);
    println!("{}", result)
}
