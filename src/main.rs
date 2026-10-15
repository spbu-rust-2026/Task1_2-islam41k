use std::io;

fn main() {
    let mut s: i128 = 0;

    loop {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");
        let mut n = n.trim();
        if n == "" {
            continue;
        } else {
            let n: i128 = match n.parse() {
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
                s += n;
            }
        }
    }

    println!("{}", s);
}
