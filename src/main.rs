use std::io::{self, BufRead};

fn main() {
    let mut s: i64 = 0;

    for line in io::stdin().lock().lines() {
        let n: i64 = match line.unwrap().trim().parse() {
            Ok(k) => k,
            Err(_) => {
                println!("NaN");
                return;
            }
        };
        if n == -1 {
            break;
        }
        s += n;
    }

    println!("{}", s);
}