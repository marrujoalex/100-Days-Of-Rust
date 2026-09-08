use std::collections::HashMap;

fn pairs_of_socks(input_str: &str) -> u8 {
    let mut pairs: HashMap<String, u8> = HashMap::new();

    for (_, c) in input_str.chars().enumerate() {
        let pair_key = String::from(c);
        if let Some(value) = pairs.get_mut(&pair_key) {
             *value += 1;
        } else {
            pairs.insert(pair_key, 1);
        }
    }

    for value in pairs.values_mut() {
        *value /= 2;
    }

    let total_count = pairs.values().sum();
    total_count
}

/*
* Examples from project README.md
*   SockPairs("AA") ➞ 1
*   SockPairs("ABABC") ➞ 2
*   SockPairs("CABBACCC") ➞ 4
*/
fn main() {
    // let test_string = "AABBBCCCCAACCA";
    // let test_string = "AA";
    // let test_string = "ABABC";
    let test_string = "CABBACCC";

    let result = pairs_of_socks(test_string);
    println!("{:?}", result);
}
