#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// bugged error[E0004]: non-exhaustive patterns: `None` not covered
// fn plus_one_bugged(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn main() {
    let a_coin = Coin::Penny;
    let result = value_in_cents(a_coin);
    println!("The value of the coin is: {}", result);

    let a_state = UsState::Alaska;
    let b_coin = Coin::Quarter(a_state);
    let result1 = value_in_cents(b_coin);
    println!("The value of the coin is: {}", result1);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("The value of five is: {:?}", five);
    println!("The value of six is: {:?}", six);
    println!("The value of none is: {:?}", none);

    let dice_roll = 9;
    let a = match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    };
    println!("The value of a is: {:?}", a);
}
