use std::io::{self, Write};
fn main() {
    print!("X  = ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Failed to parse input");
    let mut a = [0,1,0];
    let mut y = 0;
    print!("Xn = ");
    if n == 0 {
        println!("{}",a[0]);
    }
    else if n == 1 {
        println!("{}",a[1]);
    }
    else {
        loop {
            if y == n-1 {
                break;
            }
            a[2] = a[0] + a[1];
            a[0] = a[1];
            a[1] = a[2];
            y += 1;
        }
        println!("{}",a[2]);
    }
}