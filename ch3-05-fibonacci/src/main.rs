use std::io;

fn fibonacci(x: u64) -> u64 {
    match x {
        0 => 0,
        _ => {
            let mut sum = 0;
            let mut last = 0;
            let mut current = 1;
            for _i in 1..x {
                sum = last + current;
                last = current;
                current = sum;
            }
            sum
        }
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
