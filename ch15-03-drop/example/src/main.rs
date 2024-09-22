struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    {
        let e = CustomSmartPointer {
            data: String::from("some data e"),
        };
        println!("CustomSmartPointer created. e");
        drop(e);
        println!("CustomSmartPointer dropped before the end of main. e");
    }
    {
        let f = CustomSmartPointer {
            data: String::from("other stuff f"),
        };
        println!("CustomSmartPointers created. f");
    }
    // f will drop before the end of main, because f goes out of scope.
    println!("c {}", c.data);
}
