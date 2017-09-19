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
    let a = readvec();
    let b = readvec();
    
    let mut apoint = 0;
    let mut bpoint = 0;
    for i in 0..3 {
        if a[i] < b[i] {
            bpoint += 1;
        } else if a[i] > b[i] {
            apoint += 1;
        }
    }
    println!("{} {}", apoint, bpoint);
}
