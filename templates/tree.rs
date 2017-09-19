// multi children Tree  

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
