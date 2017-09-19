use std::io;
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
    let vars = readvec();
    let x1 = vars[0];
    let v1 = vars[1];
    let x2 = vars[2];
    let v2 = vars[3];
    if (x1 - x2) * (v2 - v1) > 0 && (x1 - x2) % (v2 - v1) == 0  {
        println!("YES");
    } else {
        println!("NO");
    }
}
