pub struct UnionFind {
    backing: Vec<Element>,
}

#[derive(Clone, Copy, Debug)]
struct Element {
    parent: usize,
    rank: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind {
            backing: (0..size).map(|i| Element { parent: i, rank: 0 }).collect(),
        }
    }

    pub fn fresh(&mut self) -> usize {
        let fresh = self.backing.len();
        self.backing.push(Element {
            parent: fresh,
            rank: 0,
        });
        fresh
    }

    pub fn find(&mut self, element_id: usize) -> Option<usize> {
        if element_id >= self.backing.len() {
            None
        } else {
            let mut current = element_id;
            loop {
                let element = self.backing[element_id];
                if element.parent == current {
                    break;
                }
                current = element.parent;
            }
            let rep = current;
            current = element_id;
            loop {
                let element = self.backing[current];
                if current == element.parent {
                    break;
                }
                self.backing[current] = Element {
                    parent: rep,
                    rank: element.rank,
                };
                current = element.parent;
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

            if self.backing[rep1].rank < self.backing[rep2].rank {
                self.backing[rep1].parent = rep2;
                Some(rep2)
            } else if self.backing[rep1].rank > self.backing[rep2].rank {
                self.backing[rep2].parent = rep1;
                Some(rep1)
            } else {
                self.backing[rep1].parent = rep2;
                self.backing[rep2].rank = self.backing[rep2].rank + 1;
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
