## Sort Algorithms


### [Bubble](./bubble_sort.rs)
![alt text][bubble-image]

From [Wikipedia][bubble-wiki]: Bubble sort, sometimes referred to as sinking sort, is a simple sorting algorithm that repeatedly steps through the list to be sorted, compares each pair of adjacent items and swaps them if they are in the wrong order. The pass through the list is repeated until no swaps are needed, which indicates that the list is sorted.

__Properties__
* Worst case performance	O(n^2)
* Best case performance	O(n)
* Average case performance	O(n^2)

###### View the algorithm in [action][bubble-toptal]



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

[bubble-toptal]: https://www.toptal.com/developers/sorting-algorithms/bubble-sort
[bubble-wiki]: https://en.wikipedia.org/wiki/Bubble_sort
[bubble-image]: https://upload.wikimedia.org/wikipedia/commons/thumb/8/83/Bubblesort-edited-color.svg/220px-Bubblesort-edited-color.svg.png "Bubble Sort"

[counting-wiki]: https://en.wikipedia.org/wiki/Counting_sort

[insertion-toptal]: https://www.toptal.com/developers/sorting-algorithms/insertion-sort
[insertion-wiki]: https://en.wikipedia.org/wiki/Insertion_sort
[insertion-image]: https://upload.wikimedia.org/wikipedia/commons/7/7e/Insertionsort-edited.png "Insertion Sort"

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
