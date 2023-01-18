use std::io::{self, Write};
fn main() {
    print!("Enter number: ");
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let y: i32 = input.trim().parse().expect("Failed to parse input");
    print!("{} = ", y); 
    let mut x = y;
    loop {
        if x == 0 || x == 1 || x == 2 || x == 3 || x == 5 || x == 7 || x == 11 || x == 13{
            print!("{}", x);
            break;
        }
        else if x % 2 == 0 {
            x = x / 2 ;
            print!("2");
        }
        else if x % 3 == 0 {
            x = x / 3 ;
            print!("3");
        }
        else if x % 5 == 0 {
            x = x / 5 ;
            print!("5");
        }
        else if x % 7 == 0 {
            x = x / 7 ;
            print!("7");
        }
        print!("*");
    }
}