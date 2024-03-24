fn main() {
    another_function(123);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let num = five();

    println!("The num is: {}", num);
}

fn another_function(x: i32) {
    println!("Another function. arg: {}", x);
}

fn five() -> i32 {
    5
}
