/// A [`UnionFind`] structure allows you to maintain items
/// indexed by natural numbers, each in a disjoint set.
pub struct UnionFind {
    backing: Vec<Element>,
}

#[derive(Clone, Copy, Debug)]
struct Element {
    parent: usize,
    rank: usize,
}

impl UnionFind {
    /// Construct a new [`UnionFind`] with the given number of initial elements,
    /// each in their own set.
    pub fn new(size: usize) -> Self {
        UnionFind {
            backing: (0..size).map(|i| Element { parent: i, rank: 0 }).collect(),
        }
    }

    /// Add a fresh element into the union find structure.
    pub fn fresh(&mut self) -> usize {
        let fresh = self.backing.len();
        self.backing.push(Element {
            parent: fresh,
            rank: 0,
        });
        fresh
    }

    /// Find the representative for the set that this element belongs to.
    pub fn find(&mut self, element_id: usize) -> Option<usize> {
        if element_id >= self.backing.len() {
            None
        } else {
            let mut current = element_id;
            // First, we loop through the pointer structure starting at our element_id
            // and find the root, which is an element which points to itself.
            loop {
                let element = self.backing[element_id];
                // If the current element's parent is equal to itself, it is by
                // definition the root.
                if element.parent == current {
                    break;
                }
                // Otherwise, we set current equal to the parent and continue
                // the loop.
                current = element.parent;
            }
            let rep = current;
            current = element_id;
            // Next, we loop through the pointer structure again, updating each element
            // on the way to point to the representative of our group. This way, in the
            // future, this will complete much faster.
            loop {
                let element = self.backing[current];
                // If the current node is equal to its parent, then we have
                // reached the representative element for this set.
                if current == element.parent {
                    break;
                }
                // Otherwise, we set the parent to be the representative element,
                // maintaining the previous rank, update current to be equal to the
                // parent, and continue the loop.
                self.backing[current].parent = rep;
                current = element.parent;
            }
            Some(rep)
        }
    }

    /// Cause the union of the sets which two elements belong to.
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
    const SIZE: usize = 10000;

    use super::*;

    #[test]
    fn union_all() {
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
        let mut uf = UnionFind::new(SIZE);
        let rep = uf.find(0).unwrap();
        for i in 1..SIZE {
            assert_ne!(uf.find(i).unwrap(), rep);
        }
    }

    #[test]
    fn union_evens() {
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
