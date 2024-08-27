#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

use std::thread;
use std::time::Duration;
fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    let example_closure = |x| x + 1;

    // let s = example_closure(String::from("hello"));
    let n = example_closure(5);

    //Defining and calling a closure that captures an immutable reference
    println!("closure that captures a immutable reference");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let only_borrows = || println!("From closure: {list:?}");
    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    //Defining and calling a closure that captures a mutable reference
    println!("closure that captures a mutable reference");
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    let mut borrows_mutably = || list.push(7);
    // cannot borrow `list` as immutable because it is also borrowed as mutable
    // println!("Before calling closure: {list:?}");
    borrows_mutably();
    println!("After calling closure: {list:?}");

    //Using move to force the closure for the thread to take ownership of list
    println!("move to force the thread to take ownership");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();
    //error[E0382]: borrow of moved value: `list`
    // println!("{list:?}");

    {
        let mut list = [
            Rectangle {
                width: 10,
                height: 1,
            },
            Rectangle {
                width: 3,
                height: 5,
            },
            Rectangle {
                width: 7,
                height: 12,
            },
        ];

        list.sort_by_key(|r| r.width);
        println!("{list:#?}");

        //error[E0507]: cannot move out of `value`, a captured variable in an `FnMut` closure
        // let mut sort_operations = vec![];
        // let value = String::from("closure called");
        // list.sort_by_key(|r| {
        //     sort_operations.push(value);
        //     r.width
        // });
        // println!("{list:#?}");

        let mut sort_operations = vec![];
        let value = String::from("closure called");
        // error[E0507]: cannot move out of `value`, a captured variable in an `FnMut` closure
        list.sort_by_key(|r| {
            sort_operations.push(value.clone());
            r.width
        });
        println!("{list:#?}");
        println!("{sort_operations:#?}");

        let mut num_sort_operations = 0;
        list.sort_by_key(|r| {
            num_sort_operations += 1;
            r.width
        });
        println!("{list:#?}, sorted in {num_sort_operations} operations");

        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();
        for val in v1_iter {
            println!("Got: {val}");
        }
        //Iterator adaptors
        let v1: Vec<i32> = vec![1, 2, 3];
        v1.iter().map(|x| x + 1);
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        println!("{v2:#?}");
        assert_eq!(v2, vec![2, 3, 4]);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
        // error[E0382]: borrow of moved value: `v1_iter`
        // We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the iterator we call it on.
        // println!("v1_iter: {v1_iter:?}");
    }
}
