use std::io;
fn readint() -> u64 {
    let s = String::new();
    io::stdin().read_line(&mut s).ok().expect("read error");
    return s.trim().parse().ok().expect("parse error");
}

fn main() {
    let t = readint();
    for _ in 0..t {
        let x = readint();
        let log = (x as f64).log10() as i32 + 1;
        let mut y = x.clone();
        let mut count = 0;
        for i in 0..log {
            let base = (10f64).powi(log - i - 1);
            // println!("{}", base);
            let digit = (y as f64 / base) as u64;
            // println!("i: {}, digit: {}, y: {}", i, digit, y);
            if digit != 0 && x % digit == 0{
                count += 1;
            }
            y = y % base as u64;
        }
        println!("{}", count);
    }
}
