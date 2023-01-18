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
        let mut w=0;
        loop {
            if w == x-1 {
                break;
            }
            else if w >= x-y-1 {
                print!("* ");
            }
            else {
                print!("  ");
            }
            w += 1;
        }
        w=0;
        loop {
            if w == y {
                println!("*");
                break;
            }
            print!("* ");
            w += 1;
        }
        y += 1;
    }
}