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

struct Node {
    id: usize,
    children: Vec<Node>,
}

impl Node {
    pub fn insert(&mut self, parent: usize, val: usize) {
        if parent == self.id {
            match self.find(val, 1) {
                Some(_) => println!("node {} exist under {}", val, parent),
                None => self.children.push(Node{id: val, children: Vec::new()}),
            }
            return
        }
        for e in &mut self.children {
            e.insert(parent, val);
        }
    }

    pub fn find(&self, id: usize, depth: i8) -> Option<&Node> {
        if depth < 0 {
            return None;
        }
        if id == self.id {
            return Some(self);
        }

        for e in &self.children {
            match e.find(id, depth - 1) {
                Some(n) => return Some(n),
                None => continue,
            }
        }

        return None
    }
    
    pub fn print_all(&self) {
        print!("{} -> ", self.id);
        for c in &self.children {
            print!("{} ", c.id);
        }
        println!("");

        for c in &self.children {
            c.print_all();
        }
        
    }
}

fn main() {
    let mn : Vec<usize> = readvec();
    // let n = mn[0];
    let m = mn[1];
    let mut edges : Vec<(usize, usize)> =  Vec::with_capacity(m);

    let mut root = Node { id: 1, children: Vec::new() };

    for _ in 0..m {
        let mut e = readvec();
        e.sort();
        edges.push((e[0], e[1]));
    }
    edges.sort();
    // println!("{:?}, {:?}", mn, edges);

    for e in edges {
        root.insert(e.0, e.1);
    }

    // root.print_all();

    fn traverse(node: &Node) -> (u8, u8) {
        let sum = node.children
            .iter()
            .map(|n| traverse(&n))
            // 1 for self node
            .fold((1, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

        if sum.0 % 2 == 0 {
            return (0, sum.1 + 1);
        } else {
            return (sum.0, sum.1);
        }
    }

    let res = traverse(&root);
    // println!("{}, {}", res.0, res.1 - 1);
    println!("{}", res.1 - 1);
}
