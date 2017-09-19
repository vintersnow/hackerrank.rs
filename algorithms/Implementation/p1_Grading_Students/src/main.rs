use std::io;
fn readint() -> u64 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok().expect("read error");
    return s.trim().parse().ok().expect("parse error");
}
fn main() {
    let n = readint();
    // let mut scores = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let s =  readint();
        println!("{}", if s % 5 >= 3 {
            let new_score = s + (5 - s % 5);
            if new_score >= 40 { new_score } else { s }
        } else { s });
    }
}
