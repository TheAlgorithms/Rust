### [Binary](./binary_search.rs)

B-Trees are version of 2-3 trees, which are self-balancing. They are used to improve Disk reads and have a complexity of
O(log(n)), for every tree operations.The number of Childrens/Keys a particular node has, is
determined by the Branching Factor/Degree of that tree.
Btrees will always have sorted keys.

- Branching Factor(B) / Degree (D):
  If B = n, n <= Children per Node < 2(n), n-1 <= Keys per Node < 2(n) - 1

__Properties__
* Worst/Average case performance for all operations	O(log n)
* Space complexity	O(n)

__Sources to read:__
* [Busying Oneself with B-Trees](https://medium.com/basecs/busying-oneself-with-b-trees-78bbf10522e7)
* [Geeksforgeeks](https://www.geeksforgeeks.org/introduction-of-b-tree-2/)
* [Rust API Docs](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
* [Keon Algorithms](https://github.com/keon/algorithms)
* [MIT Open Course](https://www.youtube.com/watch?v=TOb1tuEZ2X4)

### [AVL Tree](./avl_tree.rs)

An AVL Tree is a self-balancing binary search tree. The heights of any two sibling
nodes must differ by at most one; the tree may rebalance itself after insertion or
deletion to uphold this property.

__Properties__
* Worst/Average time complexity for basic operations: O(log n)
* Worst/Average space complexity: O(n)

__Sources to read:__
* [Wikipedia](https://en.wikipedia.org/wiki/AVL_tree)
* Geeksforgeeks
([Insertion](https://www.geeksforgeeks.org/avl-tree-set-1-insertion),
[Deletion](https://www.geeksforgeeks.org/avl-tree-set-2-deletion))
