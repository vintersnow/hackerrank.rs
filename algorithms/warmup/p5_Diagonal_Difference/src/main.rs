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
    let n = readint();
    let mut dig1 = 0;
    let mut dig2 = 0;
    for i in 0..n {
        let row = readvec();
        dig1 += row[i as usize];
        dig2 += row[(n - i - 1) as usize];
    }
    println!("{}", (dig1 - dig2).abs());
}
