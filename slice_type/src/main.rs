fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    println!("the length of first work is {}", word);
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); //convert our String to an array of bytes using the as_bytes method.

    // println!("{:?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
