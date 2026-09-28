use std::io::{self, BufRead};

fn main() {
    let mut s: i128 = 0;

    for line in io::stdin().lock().lines() {
        let n: i128 = match line.unwrap().trim().parse() {
            Ok(k) => k,
            Err(_) => {
                println!("NaN");
                return;
            }
        };
        if n == -1 {
            break;
        }
        if n <= 0 {
            println!("NaN");
            return;
        }
        s += n;
    }

    println!("{}", s);
}
