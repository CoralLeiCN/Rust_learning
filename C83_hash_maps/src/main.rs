fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // get values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{team_name}: {score}");

    // iterate over each key-value pair
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // println!("{field_name}");
    // println!("{field_value}");

    // Overwriting a Value
    let mut scores1 = HashMap::new();

    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Blue"), 25);

    println!("{scores1:?}");

    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);

    scores2.entry(String::from("Yellow")).or_insert(50);
    // return a mutable reference
    let a = scores2.entry(String::from("Yellow")).or_insert(50);
    println!("{a}");
    scores2.entry(String::from("Blue")).or_insert(50);
    let b = scores2.entry(String::from("Blue")).or_insert(50);
    println!("{b}");

    println!("{scores2:?}");

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");
}
