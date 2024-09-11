use std::ops::Deref;
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct MyBox_noDeref<T>(T);

impl<T> MyBox_noDeref<T> {
    fn new(x: T) -> MyBox_noDeref<T> {
        MyBox_noDeref(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let x = 5;
    let y = &x;
    let y_box = Box::new(x);
    let y_mybox = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *y_box);
    assert_eq!(5, *y_mybox);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);

    let m_noDeref = MyBox_noDeref::new(String::from("Rust"));
    // error[E0308]: mismatched types
    // hello(&m_noDeref);

    // error[E0614]: type `MyBox_noDeref<String>` cannot be dereferenced
    // hello(&(*m_noDeref)[..]);
}
