### [B-Trees](./b_tree.rs)

B-Trees are version of 2-3 trees, which are self-balancing. They are used to improve Disk reads and have a complexity of
O(log(n)), for every tree operations.The number of Childrens/Keys a particular node has, is
determined by the Branching Factor/Degree of that tree.
B-Trees will always have sorted keys.

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


### [Doubly linked list](./linked_list.rs)
![alt text][doubly-linked-list]

A linked list is also a `linear` data structure, and each element in the linked list is actually a separate object while all the objects are `linked together by the reference filed` in each element. In a `doubly linked list`, each node contains, besides the `next` node link, a second link field pointing to the `previous` node in the sequence. The two links may be called `next` and `prev`. And many modern operating systems use doubly linked lists to maintain references to active processes, threads and other dynamic objects.

__Properties__
* Indexing O(n)
* Insertion O(1)
  * Beginning O(1)
  * Middle (Indexing time+O(1))
  * End O(n)
* Deletion O(1)
  * Beginning O(1)
  * Middle (Indexing time+O(1))
  * End O(n)
* Search O(n)

__Source to read:__
* [Wikipedia](https://en.wikipedia.org/wiki/Linked_list)
* [LeetCode](https://leetcode.com/explore/learn/card/linked-list/)
* [Brilliant](https://brilliant.org/wiki/linked-lists/)
* [Rust API Docs](https://doc.rust-lang.org/std/collections/struct.LinkedList.html)


### [Stack Using Singly Linked List](./stack_using_singly_linked_list.rs)
![][stack]

From Wikipedia, a stack is an abstract data type that serves as a collection of elements, with two main principal operations, `Push` and `Pop`.

__Properties__
* Push O(1)
* Pop head.data O(1) tail.data O(n)
* Peek O(1)


__Source to read:__
* [Wikipedia](https://en.wikipedia.org/wiki/Linked_list)
* [rust-unofficial](https://rust-unofficial.github.io/too-many-lists/index.html)
* [Stack Implementation and complexity](https://medium.com/@kaichimomose/stack-implementation-and-complexity-c176924e6a6b)



[doubly-linked-list]: https://upload.wikimedia.org/wikipedia/commons/thumb/5/5e/Doubly-linked-list.svg/610px-Doubly-linked-list.svg.png

[stack]: https://upload.wikimedia.org/wikipedia/commons/thumb/b/b4/Lifo_stack.png/700px-Lifo_stack.png