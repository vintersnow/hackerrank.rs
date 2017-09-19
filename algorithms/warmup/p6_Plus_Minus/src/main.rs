use std::io;
fn readint() -> u64 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok().expect("read error");
    return s.trim().parse().ok().expect("parse error");
}

fn readvec() -> Vec<i64> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok().expect("read error");
    let split: Vec<&str> = s.split(' ').collect();
    return split.iter().map(|x| x.trim()
                                .parse::<i64>()
                                .expect("parse error")
                            ).collect();
}

fn main() {
    let _ = readint();
    let vec : Vec<i64> = readvec();
    let pos : usize = vec.iter().filter(|x| **x > 0i64).collect::<Vec<_>>().len();
    let neg : usize = vec.iter().filter(|x| **x < 0i64).collect::<Vec<_>>().len();
    let zero = vec.len() - pos - neg;
    println!("{}", pos as f64 / vec.len() as f64);
    println!("{}", neg as f64 / vec.len() as f64);
    println!("{}", zero as f64 / vec.len() as f64);
}
