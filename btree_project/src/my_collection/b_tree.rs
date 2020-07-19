pub struct BTreeNode {
    vals: Vec<i32>,           // array of keys
    degree: i32,              // minimum degree (defines the range for number of keys)
    children: Vec<BTreeNode>, // array of children
}

impl BTreeNode {
    pub fn traverse(&self) -> () {
        for (i, k) in self.vals.iter().enumerate() {
            if !self.children.len() == 0 {
                self.children[i].traverse();
            }
            print!("{} ", k);
        }
    }

    

    pub fn search(&self, searchTerm: i32) -> bool {
        for index in 0..self.vals.len() {
            let ref currTerm = self.vals[index];
            match searchTerm.cmp(currTerm) {
                Greater => continue,
                Equal => return true,
                Less => return self.children[index].search(searchTerm),
            }
        }
        if self.isLeaf() {
            return false;
        }
        return self.children[self.vals.len()].search(searchTerm);
    }

    pub fn isLeaf(&self) -> bool {
        return self.children.len() == 0;
    }
}
pub struct BTree {
    root: Option<BTreeNode>,
    degree: i32,
}

impl BTree {
    pub fn new(d: i32) -> BTree {
        BTree {
            root: None,
            degree: d,
        }
    }
    pub fn traverse(&self) -> () {
        let head = &self.root;
        match head {
            Some(x) => x.traverse(),
            None => (),
        }
    }
    // insert a new value into the tree
    pub fn insert(&self, term: i32) -> () {
        match self.root {
            Some(x) => x.insert(term),
            None => {
                self.root = Some(BTreeNode {
                    vals: vec![term],
                    degree: self.degree,
                    children: Vec::new(),
                });
            }
        }
    }
    pub fn search(&self, searchTerm: i32) -> bool {
        match self.root {
            Some(x) => return x.search(searchTerm),
            None => return false,
        }
    }
}
