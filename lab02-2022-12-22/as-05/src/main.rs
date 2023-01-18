use std::io::{self, Write};
fn main() {
    print!("Input y = ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let x: i32 = input.trim().parse().expect("Failed to parse input");
    let (mut y,mut z) = (0 , 0);
    loop {
        if y == x {
            break;
        }
        loop {
            if z == x {
                break;
            }
            else if z == 0 || z == x-1 || z == y{
                print!("X ");
            }
            else {
                print!("O ");
            }
            z += 1;
        }
        println!("");
        z = 0;
        y += 1;
    }
}