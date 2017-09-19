use std::io;
fn readint() -> usize {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok().expect("read error");
    return s.trim().parse().ok().expect("parse error");
}

fn readvec() -> Vec<usize> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok().expect("read error");
    let split: Vec<&str> = s.split(' ').collect();
    return split.iter().map(|x| x.trim()
                                .parse::<usize>()
                                .expect("parse error")
                            ).collect();
}

fn repair() -> u64 {
    let vars = readvec();
    let n = vars[0]; let m = vars[1]; let clib = vars[2]; let croad = vars[3];
    let mut roads : Vec<Vec<usize>> = Vec::with_capacity(m);
    for _ in 0..m {
        let mut r = readvec();
        r.sort();
        roads.push(r);
    }

    if clib <= croad {
        return (clib * n) as u64;
    }
    
    roads.sort();

    // println!("{}, {}, {}, {}", n,m,clib,croad);
    // println!("{:?}", roads);

    let mut map = vec![0;n as usize];
    // println!("{:?}", map);
    
    let mut group = 0;
    for r in roads {
        if map[r[0] - 1] == 0 && map[r[1] - 1] == 0 {
            group += 1;
            map[r[0] - 1] = group;
        }
        map[r[1] - 1] = map[r[0] - 1];
    }
    // println!("{} {:?}",group, map);
    let mut city_num = vec![0;n as usize];
    for c in map {
        let t = if c == 0 {
            group+=1; group
        } else { c };
        city_num[t - 1] += 1;
    }
    // println!("{:?}",city_num);
    
    let mut cost = 0;
    for g in city_num {
        if g != 0 {
            cost += clib + (g - 1) * croad;
        }
    }

    return cost as u64;
}

fn main() {
    let q = readint();
    for _ in 0..q {
        let cost = repair();
        println!("{}", cost);
    }
}
