fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // change(&s);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // not working, because can only have one mutable reference
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    //error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    // {
    //     let mut s = String::from("hello");

    //     let r1 = &s; // no problem
    //     let r2 = &s; // no problem
    //     let r3 = &mut s; // BIG PROBLEM

    //     println!("{}, {}, and {}", r1, r2, r3);
    // }

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    println!("{} and {}", r1, r2);
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    let reference_to_nothing = no_dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

//This function's return type contains a borrowed value, but there is no value for it to be borrowed from
// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

fn change(some_string: &mut String) {
    some_string.push_str(", world from change func");
}

//without the & symbol, the function would take ownership of the string
fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// error[E0596]:
// `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }
