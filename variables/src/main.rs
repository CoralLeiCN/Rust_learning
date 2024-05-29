fn main() {
    let x: i32 = 5;
    println!("The value of x is: {x}");
    let x = 6;
    println!("The value of x is: {x}");

    let x: i32 = x + 1;

    {
        let x: i32 = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // let mut spaces = "   ";
    // spaces = spaces.len();
    // error message
    // ^^^^^^^^^^^^ expected `&str`, found `usize`

    let spaces: &str = "   ";
    let _spaces: usize = spaces.len();

    // data types
    let _guess: u32 = "42".parse().expect("Not a number!");

    // float type
    let _x: f64 = 2.0; // f64

    let _y: f32 = 3.0; // f32

    let _t = true;

    let _f: bool = false; // with explicit type annotation

    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // compound types
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    {
        let tup = (500, 6.4, 1);

        let (x, y, z) = tup;

        println!("The value of y is: {y}");
    }

    let _a = [1, 2, 3, 4, 5];

    let x = [3; 5]; //same as writing let a = [3, 3, 3, 3, 3];

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    println!("The value of first is: {first}");

    use std::io;
    {
        let a = [1, 2, 3, 4, 5];

        println!("Please enter an array index.");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let element = a[index];

        println!("The value of the element at index {index} is: {element}");
    }
}
