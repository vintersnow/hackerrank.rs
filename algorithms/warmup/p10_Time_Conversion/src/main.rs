use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok().expect("read error");

    let split: Vec<&str> = s.split(':').collect();
    let suffix = &(split[2])[2..4];
    let pm = match suffix {
        "PM" => true,
        "AM" => false,
        _ => panic!("Unknonw suffix")
    };
    let h = split[0].trim();
    let m = split[1].trim();
    let s = &(split[2])[0..2].trim();

    let hi = h.parse::<i32>().expect("parse error");
    let h = if pm { 
        let ht = hi + 12;
        if ht == 24 { ht - 12 } else { ht }
    } else {  if hi == 12 { 0 } else {hi} };

    println!("{:02}:{}:{}", h,m,s);
}
