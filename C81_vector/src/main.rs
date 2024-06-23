enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // specify the type within angle brackets.
    let v: Vec<i32> = Vec::new();
    //Rust will infer the type of value you want to store
    let v1 = vec![1, 2, 3];

    let v2: Vec<i32> = vec![1, 2, 3];

    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    let v4 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v4[2];
    println!("The third element is {third}");

    let n = 4;
    let fourth: Option<&i32> = v4.get(n);
    match fourth {
        Some(fourth) => println!("The {n}th element is {fourth}"),
        None => println!("There is no {n} element."),
    }

    let tenth: Option<&i32> = v4.get(10);
    match tenth {
        Some(tenth) => println!("The third element is {tenth}"),
        None => println!("There is no tenth element."),
    }

    let v = vec![1, 2, 3, 4, 5];
    // [] method will cause the program to panic
    // let does_not_exist = &v[100];
    // When the get method is passed an index that is outside the vector, it returns None without panicking.
    // let does_not_exist = v.get(100);

    // // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    // let mut v5 = vec![1, 2, 3, 4, 5];
    // let first = &v5[0]; //-- immutable borrow occurs here
    // v5.push(6); //mutable borrow occurs here
    // println!("The first element is: {first}");

    let v5 = vec![100, 32, 57];
    for i in &v5 {
        println!("{i}");
    }

    let mut v6 = vec![100, 32, 57];
    for i in &mut v6 {
        *i += 50;
        println!("{i}");
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
