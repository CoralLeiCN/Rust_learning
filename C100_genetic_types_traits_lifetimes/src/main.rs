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

struct Point_multi_generics<T, U> {
    x: T,
    y: U,
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

    let integer_and_float = Point_multi_generics { x: 5, y: 4.0 };
}
