## Search Algorithms

### [Linear](./linear_search.rs)
![alt text][linear-image]

From [Wikipedia][linear-wiki]: linear search or sequential search is a method for finding a target value within a list. It sequentially checks each element of the list for the target value until a match is found or until all the elements have been searched.
  Linear search runs in at worst linear time and makes at most n comparisons, where n is the length of the list.

__Properties__
* Worst case performance	O(n)
* Best case performance	O(1)
* Average case performance	O(n)
* Worst case space complexity	O(1) iterative

### [Binary](./binary_search.rs)
![alt text][binary-image]

From [Wikipedia][binary-wiki]: Binary search, also known as half-interval search or logarithmic search, is a search algorithm that finds the position of a target value within a sorted array. It compares the target value to the middle element of the array; if they are unequal, the half in which the target cannot lie is eliminated and the search continues on the remaining half until it is successful.

__Properties__
* Worst case performance	O(log n)
* Best case performance	O(1)
* Average case performance	O(log n)
* Worst case space complexity	O(1)

[linear-wiki]: https://en.wikipedia.org/wiki/Linear_search
[linear-image]: http://www.tutorialspoint.com/data_structures_algorithms/images/linear_search.gif

[binary-wiki]: https://en.wikipedia.org/wiki/Binary_search_algorithm
[binary-image]: https://upload.wikimedia.org/wikipedia/commons/f/f7/Binary_search_into_array.png

### [Peak Finder](./peak_finder.rs)

From [MIT OpenCourseWare](peak-finder-youtube): PDF can be found [here](peak-finder-pdf)

Peak finder helps to find the highest point present in a slope. Following algorithms, helps
to find the first found peak, moving to top of positive slope direction.

**Brute force**

Simplest way to solve is to loop through all elements, checking the target element to it's nearby elements, in forward or backward direction, where if the element next to the target is small, then target is the peak element.

**Properties**

- Worst case performance O(n)
- Best case performance O(1)
- Average case performance O(n)

**Divide & Conquer**

1. Split the `array` from the `mid`,
2. If `array[mid] <= array[mid-1]`, repeat Step 1 for `array[mid+1]` to `array[last]`
3. Else If `array[mid] <= array[mid-1]`, repeat Step 1 for `array[first]` to `array[mid-1]`.
4. Else `array[mid]` is the Peak.

**Properties**

- Worst case performance O(log n)
- Best case performance O(1)
- Average case performance O(log n)

[peak-finder-youtube]: https://www.youtube.com/watch?v=HtSuA80QTyo&list=PLUl4u3cNGP61Oq3tWYp6V_F-5jb5L2iHb&index=2&t=1179s
[peak-finder-pdf]: https://ocw.mit.edu/courses/electrical-engineering-and-computer-science/6-006-introduction-to-algorithms-fall-2011/lecture-videos/MIT6_006F11_lec01.pdf
