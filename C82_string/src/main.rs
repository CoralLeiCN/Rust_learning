fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}");
    println!("s2 is {s2}");

    let mut s = String::from("lo");
    //The push method takes single character
    s.push('l');

    let ss1 = String::from("Hello, ");
    let ss2 = String::from("world!");
    let ss3 = ss1 + &ss2; // note s1 has been moved here and can no longer be used

    // println!("ss1 is {ss1}"); ss1 is no longer valid after addition
    println!("ss2 is {ss2}");
    println!("ss3 is {ss3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    let sss1 = String::from("tic");
    let sss2 = String::from("tac");
    let sss3 = String::from("toe");

    let sss = format!("{sss1}-{sss2}-{sss3}");
    println!("sss1 is {sss1}");
    println!("sss2 is {sss2}");
    println!("sss3 is {sss3}");
    println!("sss is {sss}");

    //error[E0277]: the type `str` cannot be indexed by `{integer}`
    // let s1 = String::from("hello");
    // let h = s1[0];

    // error[E0277]: the type `str` cannot be indexed by `{integer}`
    // let hello = "Здравствуйте";
    // let answer = &hello[0];

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("s is {s}");

    for c in "Зд".chars() {
        println!("{c}");
    }

    for c in "Зд".bytes() {
        println!("{c}");
    }
}
