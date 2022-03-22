pub struct UnionFind {
    backing: Vec<(usize, usize)>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind {
            backing: (0..size).map(|i| (i, 0)).collect(),
        }
    }

    pub fn fresh(&mut self) -> usize {
        let fresh = self.backing.len();
        self.backing.push((fresh, 0));
        fresh
    }

    pub fn find(&mut self, element: usize) -> Option<usize> {
        if element >= self.backing.len() {
            None
        } else {
            let mut current = element;
            loop {
                let (parent, _parent_rank) = self.backing[element];
                if parent == current {
                    break;
                }
                current = parent;
            }
            let rep = current;
            current = element;
            loop {
                let (parent, _parent_rank) = self.backing[current];
                if current == parent {
                    break;
                }
                self.backing[current] = (rep, 0);
                current = parent;
            }
            Some(rep)
        }
    }

    pub fn union(&mut self, element1: usize, element2: usize) -> Option<usize> {
        if element1 >= self.backing.len() || element2 >= self.backing.len() {
            None
        } else {
            let rep1 = self.find(element1).unwrap();
            let rep2 = self.find(element2).unwrap();

            if rep1 == rep2 {
                return Some(rep1);
            }

            if self.backing[rep1].1 < self.backing[rep2].1 {
                self.backing[rep1].0 = rep2;
                Some(rep2)
            } else if self.backing[rep1].1 > self.backing[rep2].1 {
                self.backing[rep2].0 = rep1;
                Some(rep1)
            } else {
                self.backing[rep1].0 = rep2;
                self.backing[rep2].1 = self.backing[rep2].1 + 1;
                Some(rep1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn union_all() {
        const SIZE: usize = 10;
        let mut uf = UnionFind::new(SIZE);
        for i in 0..SIZE {
            uf.union(i, i + 1 % SIZE);
        }
        let rep = uf.find(0).unwrap();
        for i in 0..SIZE {
            assert_eq!(uf.find(i).unwrap(), rep);
        }
    }

    #[test]
    fn union_none() {
        const SIZE: usize = 10;
        let mut uf = UnionFind::new(SIZE);
        let rep = uf.find(0).unwrap();
        for i in 1..SIZE {
            assert_ne!(uf.find(i).unwrap(), rep);
        }
    }

    #[test]
    fn union_evens() {
        const SIZE: usize = 10;
        let mut uf = UnionFind::new(SIZE);
        for i in 0..SIZE {
            uf.union(2 * i, 2 * (i + 1) % SIZE);
        }
        let rep = uf.find(0).unwrap();
        for i in 0..SIZE {
            if i % 2 == 0 {
                assert_eq!(uf.find(i).unwrap(), rep);
            } else {
                assert_ne!(uf.find(i).unwrap(), rep);
            }
        }
    }
}
