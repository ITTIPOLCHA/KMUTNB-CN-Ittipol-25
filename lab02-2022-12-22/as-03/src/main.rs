use std::io::{self, Write};

fn main() {
    print!("Input x = ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let xin: i32 = input.trim().parse().expect("Failed to parse input");
    let (x, mut y) = ((xin*2)-1,0);
    loop {
        if y >= xin { 
            break;
        }
        let mut w = 0;
        loop {
            if w == x { 
                break;
            }
            else if w < (y) || w > (x-y-1) {
                print!("  ");
            }
            else {
                print!("* ");
            }
            w += 1;
        }
        println!("");
        y += 1;
    }
}