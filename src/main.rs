use std::io::{self, BufRead};

fn main() {
    let mut s: i128 = 0;

    for n in io::stdin().lock().lines() {
        let n = n.unwrap();
        if n.trim() == "" {
            continue;
        } else {
            let n: i128 = match n.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("NaN");
                    return;
                }
            };
            if n == -1 {
                break;
            } else if n <= 0 {
                println!("NaN");
                return;
            } else {
                s += n
            }
        }
    }

    println!("{}", s);
}
