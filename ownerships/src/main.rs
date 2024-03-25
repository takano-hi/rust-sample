fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    let mut s3 = String::from("hoge");

    takes_ownership(s1.clone());
    borrow_ownership(&s2);
    change(&mut s3);

    println!("s1: {}", s1);
    println!("s2: {}", s2);
    println!("s3: {}", s3);
}

fn takes_ownership(s: String) {
    println!("{}", s);
}

fn borrow_ownership(s: &String) {
    println!("{}", s);
}

fn change(s: &mut String) {
    s.push_str(" & fuga");
}
