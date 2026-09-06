fn skewer_check(kabobs: Vec<String>) -> Vec<u8> {
    let mut vegetarian_skewers: u8 = 0;
    let mut non_vegetarian_skewers: u8 = 0;

    for (_, row) in kabobs.iter().enumerate() {
        if row.contains('x') {
            non_vegetarian_skewers = non_vegetarian_skewers + 1;
        } else {
            vegetarian_skewers = vegetarian_skewers + 1;
        }
    }

    vec![vegetarian_skewers, non_vegetarian_skewers]
}

fn main() -> Result<(), serde_json::Error> {
    // let test_string = r#"
    // [
    //   "--oooo-ooo--",
    //   "--xx--x--xx--",
    //   "--o---o--oo--",
    //   "--xx--x--ox--",
    //   "--xx--x--ox--"
    // ]
    // "#;

    let test_string_2 = r#"
    [
      "--oooo-ooo--",
      "--xxxxxxxx--",
      "--o---",
      "-o-----o---x--",
      "--o---o-----"
    ]
    "#;

    // to handle the incoming text, parse the raw string into a vector of strings
    // via deserialization in serde_json
    // NOTE: test strings are found in README.md at root
    let kabobs: Vec<String> = serde_json::from_str(test_string_2)?;
    let skewers = skewer_check(kabobs);

    println!("{:?}", skewers);
    Ok(())
}
