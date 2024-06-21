mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // error[E0603]: module `hosting` is private
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // error[E0603]: module `hosting` is private
    // Relative path
    // front_of_house::hosting::add_to_waitlist();
}
