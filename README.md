# Union Find

`union-find` is an implementation of the [disjoint-set
data structure](https://en.wikipedia.org/wiki/Disjoint-set_data_structure)
of the same name.

The interface to this data structure is composed of two functions. [`UnionFind::find`]
allows you to compute a canonical representative for a set, given an element
of it. [`UnionFind::union`] allows you to combine two sets, given an arbitrary representative
from each. In the future, if you were to [`UnionFind::find`] a canonical representative
from any member of either set, you will find them equal, indicating they are in
the same set according to the data structure.

The interface can be used like this:

```rust
let mut uf = UnionFind::new(10);

assert_eq!(uf.find(0).unwrap(), 0);

let rep = uf.union(0, 1).unwrap();

assert_eq!(rep, uf.find(1).unwrap());
