use std::io;
use std::collections::HashMap;

fn readint() -> u64 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    return line.trim().parse().unwrap();
}

fn main() {
    let n = readint();

    let mut ns: Vec<u64> = Vec::with_capacity(n as usize);

    for _ in 0..n {
        ns.push(readint());
    }

    let max_n = ns.iter().max().unwrap().clone();

    let num35 = (3..max_n).filter(|x| x%3==0 || x%5==0);

    let mut sort_ns = ns.clone();
    sort_ns.sort();
    sort_ns.dedup();
    Vec::

    let mut map = HashMap::new();
    
    let mut t = 0;
    let mut sum : u64 = 0;
    for i in num35 {
        while i >= sort_ns[t] {
            map.insert(sort_ns[t], sum);
            t += 1;
        }
        sum += i;
    }

    map.insert(sort_ns[t], sum);
    
    for i in ns.iter() {
        match map.get(i) {
            Some(num) => println!("{}", num),
            None => println!("0")
        }
    }
}
