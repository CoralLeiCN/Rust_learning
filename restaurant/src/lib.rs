// error[E0603]: module `hosting` is private
// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

mod customer {

    // if no use crate here, use if out of scope will have error below
    // error[E0433]: failed to resolve: use of undeclared crate or module `hosting`
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
use std::collections::HashMap;

fn create_map() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}
fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}

use std::fmt::Result;
use std::io::Result as IoResult;

fn function3() -> Result {
    // --snip--
    Ok(())
}
fn function4() -> IoResult<()> {
    // --snip--
    Ok(())
}

// use crate::front_of_house::hosting;
pub use crate::front_of_house::hosting;
use rand::Rng;

pub fn eat_at_restaurant() {
    // error[E0603]: module `hosting` is private
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // error[E0603]: module `hosting` is private
    // Relative path
    // front_of_house::hosting::add_to_waitlist();

    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    pub fn eat_at_restaurant() {
        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Salad;
    }

    pub fn eat_at_restaurant_pub() {
        hosting::add_to_waitlist();
    }

    let secret_number = rand::thread_rng().gen_range(1..=100);
}
