## Sort Algorithms

### [Bogo-sort](./bogo_sort.rs)
![alt text][bogo-image]

From [Wikipedia][bogo-wiki]: In computer science, bogosort is a sorting algorithm based on the generate and test paradigm. The function successively generates permutations of its input until it finds one that is sorted. It is not considered useful for sorting, but may be used for educational purposes, to contrast it with more efficient algorithms.

__Properties__
* Worst case performance (unbounded in randomized version)
* Best case performance O(n)
* Average case performance O((n+1)!)


### [Bubble](./bubble_sort.rs)
![alt text][bubble-image]

From [Wikipedia][bubble-wiki]: Bubble sort, sometimes referred to as sinking sort, is a simple sorting algorithm that repeatedly steps through the list to be sorted, compares each pair of adjacent items and swaps them if they are in the wrong order. The pass through the list is repeated until no swaps are needed, which indicates that the list is sorted.

__Properties__
* Worst case performance	O(n^2)
* Best case performance	O(n)
* Average case performance	O(n^2)

###### View the algorithm in [action][bubble-toptal]



### [Cocktail-Shaker](./cocktail_shaker_sort.rs)
![alt text][shaker-image]

From [Wikipedia][shaker-wiki]: Cocktail shaker sort, also known as bidirectional bubble sort, cocktail sort, shaker sort (which can also refer to a variant of selection sort), ripple sort, shuffle sort, or shuttle sort, is an extension of bubble sort. The algorithm extends bubble sort by operating in two directions. While it improves on bubble sort by more quickly moving items to the beginning of the list, it provides only marginal performance improvements.

__Properties__
* Worst case performance	O(n^2)
* Best case performance	O(n)
* Average case performance	O(n^2)



### [Comb-sort](./comb_sort.rs)
![comb sort][comb-sort]

From [wikipedia][comb-sort-wiki]: Comb sort is a relatively simple sorting algorithm and improves on bubble sort in the same way that shell sort improves on insertion sort. The basic idea of comb sort is that the gap(distance from two compared elements) can be much more than 1. And the inner loop of bubble sort, which does actual `swap`, is modified such that the gap between swapped elements goes down in steps of a `shrink factor k: [n/k, n/k^2, ..., 1]`. And the gap is divided by the shrink factor in every loop, and the process repeats until the gap is 1. At this point, comb sort continues using a gap of 1 until the list is fully sorted. The final stage of the sort is thus equivalent to a bubble sort, but this time most turtles have been dealt with, so a bubble sort will be efficient. And the shrink factor has a great effect on the efficiency of comb sort and `k=1.3` has been suggested as an ideal value.

__Properties__
* Worst case performance   O(n^2)
* Best case performance    O(n log n)
* Average case performance O(n^2/2^p)

where `p` is the number of increments.



### [Counting](./counting_sort.rs)

From [Wikipedia][counting-wiki]: In computer science, counting sort is an algorithm for sorting a collection of objects according to keys that are small integers; that is, it is an integer sorting algorithm. It operates by counting the number of objects that have each distinct key value, and using arithmetic on those counts to determine the positions of each key value in the output sequence. Its running time is linear in the number of items and the difference between the maximum and minimum key values, so it is only suitable for direct use in situations where the variation in keys is not significantly greater than the number of items. However, it is often used as a subroutine in another sorting algorithm, radix sort, that can handle larger keys more efficiently.

__Properties__
* Worst case performance	O(n+k)
* Best case performance	O(n+k)
* Average case performance	O(n+k),

where n is the number of integers to sort and k is the difference between the largest and smallest integer in our list.



### [Insertion](./insertion_sort.rs)
![alt text][insertion-image]

From [Wikipedia][insertion-wiki]: Insertion sort is a simple sorting algorithm that builds the final sorted array (or list) one item at a time. It is much less efficient on large lists than more advanced algorithms such as quicksort, heapsort, or merge sort.

__Properties__
* Worst case performance	O(n^2)
* Best case performance	O(n)
* Average case performance	O(n^2)

###### View the algorithm in [action][insertion-toptal]


### [Gnome](./gnome_sort.rs)
![alt text][gnome-image]

From [Wikipedia][gnome-wiki]: The gnome sort is a sorting algorithm which is similar to insertion sort in that it works with one item at a time but gets the item to the proper place by a series of swaps, similar to a bubble sort. It is conceptually simple, requiring no nested loops. The average running time is O(n^2) but tends towards O(n) if the list is initially almost sorted

__Properties__
* Worst case performance	O(n^2)
* Best case performance	O(n)
* Average case performance	O(n^2)



### [Merge](./merge_sort.rs)
![alt text][merge-image]

From [Wikipedia][merge-wiki]: In computer science, merge sort (also commonly spelled mergesort) is an efficient, general-purpose, comparison-based sorting algorithm. Most implementations produce a stable sort, which means that the implementation preserves the input order of equal elements in the sorted output. Mergesort is a divide and conquer algorithm that was invented by John von Neumann in 1945.

__Properties__
* Worst case performance	O(n log n)
* Best case performance	O(n log n)
* Average case performance	O(n log n)


###### View the algorithm in [action][merge-toptal]

### [Odd-even](./odd_even_sort.rs)
![alt text][odd-even-image]

From [Wikipedia][odd-even-wiki]: In computing, an odd–even sort or odd–even transposition sort (also known as brick sort or parity sort) is a relatively simple sorting algorithm, developed originally for use on parallel processors with local interconnections. It is a comparison sort related to bubble sort, with which it shares many characteristics. It functions by comparing all odd/even indexed pairs of adjacent elements in the list and, if a pair is in the wrong order (the first is larger than the second) the elements are switched. The next step repeats this for even/odd indexed pairs (of adjacent elements). Then it alternates between odd/even and even/odd steps until the list is sorted. 

NOTE: The implementation is an adaptation of the algorithm for a single-processor machine, while the original algorithm was devised to be executed on many processors simultaneously.
__Properties__
* Worst case performance	O(n^2)
* Best case performance	O(n)
* Average case performance	O(n^2)


### [Pancake](./pancake_sort.rs)
![alt text][pancake-image]

From [Wikipedia][pancake-wiki]: All sorting methods require pairs of elements to be compared. For the traditional sorting problem, the usual problem studied is to minimize the number of comparisons required to sort a list. The number of actual operations, such as swapping two elements, is then irrelevant. For pancake sorting problems, in contrast, the aim is to minimize the number of operations, where the only allowed operations are reversals of the elements of some prefix of the sequence. Now, the number of comparisons is irrelevant.


### [Quick](./quick_sort.rs)
![alt text][quick-image]

From [Wikipedia][quick-wiki]: Quicksort (sometimes called partition-exchange sort) is an efficient sorting algorithm, serving as a systematic method for placing the elements of an array in order.

__Properties__
* Worst case performance	O(n^2)
* Best case performance	O(n log n) or O(n) with three-way partition
* Average case performance	O(n log n)

###### View the algorithm in [action][quick-toptal]

### [Radix](./radix_sort.rs)
![alt text][radix-image]

From [Wikipedia][radix-wiki]: Radix sort is a non-comparative sorting algorithm. It avoids comparison by creating and distributing elements into buckets according to their radix. For elements with more than one significant digit, this bucketing process is repeated for each digit, while preserving the ordering of the prior step, until all digits have been considered.

__Properties__
* Worst case performance O(w*n)

where w is the number of bits required to store each key.

### [Selection](./selection_sort.rs)
![alt text][selection-image]

From [Wikipedia][selection-wiki]: The algorithm divides the input list into two parts: the sublist of items already sorted, which is built up from left to right at the front (left) of the list, and the sublist of items remaining to be sorted that occupy the rest of the list. Initially, the sorted sublist is empty and the unsorted sublist is the entire input list. The algorithm proceeds by finding the smallest (or largest, depending on sorting order) element in the unsorted sublist, exchanging (swapping) it with the leftmost unsorted element (putting it in sorted order), and moving the sublist boundaries one element to the right.

__Properties__
* Worst case performance	O(n^2)
* Best case performance	O(n^2)
* Average case performance	O(n^2)

###### View the algorithm in [action][selection-toptal]

### [Shell](./shell_sort.rs)
![alt text][shell-image]

From [Wikipedia][shell-wiki]:  Shellsort is a generalization of insertion sort that allows the exchange of items that are far apart.  The idea is to arrange the list of elements so that, starting anywhere, considering every nth element gives a sorted list.  Such a list is said to be h-sorted.  Equivalently, it can be thought of as h interleaved lists, each individually sorted.

__Properties__
* Worst case performance O(nlog2 2n)
* Best case performance O(n log n)
* Average case performance depends on gap sequence

###### View the algorithm in [action][shell-toptal]

### [Stooge](./stooge_sort.rs)
![alt text][stooge-image]

From [Wikipedia][stooge-wiki]:  Stooge sort is a recursive sorting algorithm. It is notable for its exceptionally bad time complexity of O(n^(log 3 / log 1.5)) = O(n^2.7095...). The running time of the algorithm is thus slower compared to reasonable sorting algorithms, and is slower than Bubble sort, a canonical example of a fairly inefficient sort. It is however more efficient than Slowsort. The name comes from The Three Stooges.

__Properties__
* Worst case performance O(n^(log(3) / log(1.5)))

### [Tim](./tim_sort.rs)
![alt text][tim-image]

From [Wikipedia][tim-wiki]:  Timsort is a hybrid stable sorting algorithm, derived from merge sort and insertion sort, designed to perform well on many kinds of real-world data. It was implemented by Tim Peters in 2002 for use in the Python programming language. The algorithm finds subsequences of the data that are already ordered (runs) and uses them to sort the remainder more efficiently. This is done by merging runs until certain criteria are fulfilled. Timsort has been Python's standard sorting algorithm since version 2.3. It is also used to sort arrays of non-primitive type in Java SE 7, on the Android platform, in GNU Octave, on V8, Swift, and Rust.

__Properties__
* Worst-case performance O(max element size(ms))
* Best-case performance	O(max element size(ms))

### [Sleep](./sleep_sort.rs)
![alt text][sleep-image]

From [Wikipedia][bucket-sort-wiki]: This is an idea that was originally posted on the message board 4chan, replacing the bucket in bucket sort with time instead of memory space.
It is actually possible to sort by "maximum of all elements x unit time to sleep". The only case where this would be useful would be in examples.

### [Patience](./patience_sort.rs)
[patience-video]


From [Wikipedia][patience-sort-wiki]: The algorithm's name derives from a simplified variant of the patience card game. The game begins with a shuffled deck of cards. The cards are dealt one by one into a sequence of piles on the table, according to the following rules.

1. Initially, there are no piles. The first card dealt forms a new pile consisting of the single card.
2. Each subsequent card is placed on the leftmost existing pile whose top card has a value greater than or equal to the new card's value, or to the right of all of the existing piles, thus forming a new pile.
3. When there are no more cards remaining to deal, the game ends.

This card game is turned into a two-phase sorting algorithm, as follows. Given an array of n elements from some totally ordered domain, consider this array as a collection of cards and simulate the patience sorting game. When the game is over, recover the sorted sequence by repeatedly picking off the minimum visible card; in other words, perform a k-way merge of the p piles, each of which is internally sorted.

__Properties__
* Worst case performance O(n log n)
* Best case performance	O(n)

[bogo-wiki]: https://en.wikipedia.org/wiki/Bogosort
[bogo-image]: https://upload.wikimedia.org/wikipedia/commons/7/7b/Bogo_sort_animation.gif

[bubble-toptal]: https://www.toptal.com/developers/sorting-algorithms/bubble-sort
[bubble-wiki]: https://en.wikipedia.org/wiki/Bubble_sort
[bubble-image]: https://upload.wikimedia.org/wikipedia/commons/thumb/8/83/Bubblesort-edited-color.svg/220px-Bubblesort-edited-color.svg.png "Bubble Sort"

[shaker-wiki]: https://en.wikipedia.org/wiki/Cocktail_shaker_sort
[shaker-image]: https://upload.wikimedia.org/wikipedia/commons/e/ef/Sorting_shaker_sort_anim.gif

[counting-wiki]: https://en.wikipedia.org/wiki/Counting_sort

[insertion-toptal]: https://www.toptal.com/developers/sorting-algorithms/insertion-sort
[insertion-wiki]: https://en.wikipedia.org/wiki/Insertion_sort
[insertion-image]: https://upload.wikimedia.org/wikipedia/commons/7/7e/Insertionsort-edited.png "Insertion Sort"

[gnome-wiki]: https://en.wikipedia.org/wiki/Gnome_sort
[gnome-image]: https://upload.wikimedia.org/wikipedia/commons/3/37/Sorting_gnomesort_anim.gif "Insertion Sort"

[pancake-wiki]: https://en.wikipedia.org/wiki/Pancake_sorting
[pancake-image]: https://upload.wikimedia.org/wikipedia/commons/0/0f/Pancake_sort_operation.png

[quick-toptal]: https://www.toptal.com/developers/sorting-algorithms/quick-sort
[quick-wiki]: https://en.wikipedia.org/wiki/Quicksort
[quick-image]: https://upload.wikimedia.org/wikipedia/commons/6/6a/Sorting_quicksort_anim.gif "Quick Sort"

[merge-toptal]: https://www.toptal.com/developers/sorting-algorithms/merge-sort
[merge-wiki]: https://en.wikipedia.org/wiki/Merge_sort
[merge-image]: https://upload.wikimedia.org/wikipedia/commons/c/cc/Merge-sort-example-300px.gif "Merge Sort"

[odd-even-image]: https://upload.wikimedia.org/wikipedia/commons/1/1b/Odd_even_sort_animation.gif
[odd-even-wiki]: https://en.wikipedia.org/wiki/Odd%E2%80%93even_sort

[radix-wiki]: https://en.wikipedia.org/wiki/Radix_sort
[radix-image]: https://ds055uzetaobb.cloudfront.net/brioche/uploads/IEZs8xJML3-radixsort_ed.png?width=400 "Radix Sort"

[selection-toptal]: https://www.toptal.com/developers/sorting-algorithms/selection-sort
[selection-wiki]: https://en.wikipedia.org/wiki/Selection_sort
[selection-image]: https://upload.wikimedia.org/wikipedia/commons/thumb/b/b0/Selection_sort_animation.gif/250px-Selection_sort_animation.gif "Selection Sort Sort"

[shell-toptal]: https://www.toptal.com/developers/sorting-algorithms/shell-sort
[shell-wiki]: https://en.wikipedia.org/wiki/Shellsort
[shell-image]: https://upload.wikimedia.org/wikipedia/commons/d/d8/Sorting_shellsort_anim.gif "Shell Sort"

[stooge-image]: https://upload.wikimedia.org/wikipedia/commons/f/f8/Sorting_stoogesort_anim.gif
[stooge-wiki]: https://en.wikipedia.org/wiki/Stooge_sort

[tim-image]: https://thumbs.gfycat.com/BruisedFrigidBlackrhino-size_restricted.gif
[tim-wiki]: https://en.wikipedia.org/wiki/Timsort

[comb-sort]: https://upload.wikimedia.org/wikipedia/commons/4/46/Comb_sort_demo.gif
[comb-sort-wiki]: https://en.wikipedia.org/wiki/Comb_sort

[sleep-sort]: <no image>
[sleep-sort-wiki]: https://ja.m.wikipedia.org/wiki/バケットソート#.E3.82.B9.E3.83.AA.E3.83.BC.E3.83.97.E3.82.BD.E3.83.BC.E3.83.88

[patience-sort-wiki]: https://en.wikipedia.org/wiki/Patience_sorting
[patience-video]: https://user-images.githubusercontent.com/67539676/212542208-d3f7a824-60d8-467c-8097-841945514ae9.mp4
