use std::io::{self, Write};

fn main() {
    print!("Input x = ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let x: i32 = input.trim().parse().expect("Failed to parse input");
    let mut y = 0;
    loop {
        if y == x {
            break;
        }
        let mut z=0;
        loop {
            if z == y {
                println!("*");
                break;
            }
            print!("* ");
            z = z + 1;
        }
        y = y + 1;
    }
}
