fn main() {
    for num in 5..8 {
        println!("{}", num);
    }

    let s = String::from("hello, world.");

    first(&s);
}

fn first(s: &String) {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("{}, {}", i, item);
    }
}
