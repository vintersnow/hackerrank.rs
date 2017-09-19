use std::io;
use std::str::FromStr;
// use std::io::{self, Write};

fn readint<T: FromStr>() -> T {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok().expect("read error");
    return s.trim().parse().ok().expect("parse error");
}

fn readvec<T: FromStr>() -> Vec<T> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok().expect("read error");
    let split: Vec<&str> = s.split(' ').collect();
    return split.iter().map(|x| x.trim()
                                .parse::<T>()
                                .ok()
                                .expect("parse error")
                            ).collect();
}
