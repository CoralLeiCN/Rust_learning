fn main() {
    println!("Hello, world!");

    another_function();
    print_number(5);
    print_labeled_measurement(5, 'h');

    let y = 6; //statements
    let y = {
        let x = 3;
        x + 1
    }; //expressions

    let x = five();

    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Another function.");
}

fn print_number(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}
