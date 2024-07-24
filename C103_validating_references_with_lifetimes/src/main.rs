// error[E0597]: `x` does not live long enough
// fn main() {
//     let r;

//     {
//         let x = 5;
//         r = &x;
//     }

//     println!("r: {r}");
// }
// error[E0106]: missing lifetime specifier
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn get_first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// error[E0515]: cannot return value referencing local variable `result`
// fn longest_no_related<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

fn main() {
    let x = 5; // ----------+-- 'b
               //           |
    let r = &x; // --+-- 'a  |
                //   |       |
    println!("r: {r}"); //   |       |
                        // --+       |

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    // error[E0597]: `string2` does not live long enough
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {result}");
} // ----------+
