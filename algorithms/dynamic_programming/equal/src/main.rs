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

fn distribute() {
    let n = readint() as usize;
    let mut choco = readvec();
    choco.sort();
    let mut counts : Vec<i32> = (0..3).map(|offset| {
        let mut pos = 0;
        let mut count = 0;
        let mut c = choco.clone();
        c[0] -= offset;
        while pos <  n - 1 {
            count += match c[pos+1] - c[pos] {
                0 => {pos+=1;0},
                1 => {c[pos+1]-=1;1},
                x if x < 5 => {c[pos+1]-=2;1},
                _ => {c[pos+1]-=5;1}
            };
        }
        count
    }).collect();

    // for offset
    counts[1] += 1;
    counts[2] += 1;

    println!("{}", counts.iter().min().expect("no min"));
}

fn main() {
    let n = readint();
    for _ in 0..n {
        distribute();
    }
}
