use std::io;
// fn readint() -> u64 {
//     let mut s = String::new();
//     io::stdin().read_line(&mut s).ok().expect("read error");
//     return s.trim().parse().ok().expect("parse error");
// }

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
    let st = readvec();
    let ab = readvec();
    let _ = readvec();
    let mut fruits = Vec::new();
    fruits.push(readvec());
    fruits.push(readvec());

    let s = st[0];
    let t = st[1];

    let is_include = |pos, d| {
        let x = pos + d;
        s <= x && x <= t
    };
    
    for i in 0..2 {
        let fruit = &fruits[i];
        let mut count = 0;
        for d in fruit {
            if is_include(ab[i], d) {
                count += 1;
            }
        }
        println!("{}", count)
    }
}
