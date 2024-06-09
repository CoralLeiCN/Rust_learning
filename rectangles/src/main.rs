#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    // println!("rect2 is {}", rect2); // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    // println!("rect1 is {:?}", rect2); // error[E0277]: `Rectangle` doesn't implement `Debug`

    println!("rect1 is {:?}", rect2); // add #[derive(Debug)]
    println!("rect1 is {:#?}", rect2); // different print format {:#?}

    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect2)
    );

    // dbg! macro
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
