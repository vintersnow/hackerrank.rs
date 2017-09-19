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
    let vec = readvec();
    let max = vec.iter().max().expect("no max");
    let count = vec.iter().fold(0, |a,x|if x==max { a+1 }else {a});

    println!("{}", count);
}
