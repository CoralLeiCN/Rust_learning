fn find_largest_number(number_list: &[i32]) -> &i32 {
    let mut largest = &number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn find_largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// error[E0369]: binary operation `>` cannot be applied to type `&T`
// fn find_largest_generic<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// fn find_largest_generic<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
struct Point_multi_generics<T, U> {
    x: T,
    y: U,
}

struct Point1<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point1<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point1<X2, Y2>) -> Point1<X1, Y2> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = find_largest_number(&number_list);

    println!("The largest numberc is {largest}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let largest = find_largest_number(&number_list);

    println!("The largest number is {largest}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = find_largest_char(&char_list);
    println!("The largest char is {result}");

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // error[E0308]: mismatched types
    // let wont_work = Point { x: 5, y: 4.0 };

    let p = Point { x: 5, y: 4 };
    println!("p.x = {}", p.x());

    // error[E0599]: no method named `distance_from_origin` found for struct `Point<{integer}>` in the current scope
    // println!("distance_from_origin = {}", p.distance_from_origin());

    let p = Point { x: 5.0, y: 4.0 };
    println!("distance_from_origin = {}", p.distance_from_origin());

    let p1 = Point1 { x: 5, y: 10.4 };
    let p2 = Point1 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
