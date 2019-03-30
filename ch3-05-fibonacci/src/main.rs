use std::io;

fn fibonacci(x: u64) -> u64 {
    match x {
        0 => 0,
        1 => 1,
        _ => fibonacci(x-1) + fibonacci(x-2)
    }
}

fn main() {
    println!("Number:");

    let mut x = String::new();
    io::stdin().read_line(&mut x)
        .expect("I/O error");
    let x = x.trim().parse::<u64>()
        .expect("Please give correct number");

    let y = fibonacci(x);

    println!("f({}) = {}", x, y);
}
