use std::io;
fn readint() -> usize {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok().expect("read error");
    return s.trim().parse().ok().expect("parse error");
}

fn main() {
    let n= readint();
    for i in 0..n {
        println!("{}{}", " ".repeat(n - i - 1), "#".repeat(i+1));
    }
}
