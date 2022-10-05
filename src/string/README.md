## String Algorithms

### [Aho-Corasick Algorithm](./aho_corasick.rs)
From [Wikipedia][aho-corasick-wiki]: a string-searching algorithm invented by Alfred V. Aho and Margaret J. Corasick in 1975.[1] It is a kind of dictionary-matching algorithm that locates elements of a finite set of strings (the "dictionary") within an input text. It matches all strings simultaneously.

[aho-corasick-wiki]: https://en.wikipedia.org/wiki/Aho%E2%80%93Corasick_algorithm


### [Burrows-Wheeler transform](./burrows_wheeler_transform.rs)
From [Wikipedia][burrows-wheeler-wiki]: The Burrows–Wheeler transform (BWT, also called block-sorting compression) rearranges a character string into runs of similar characters. This is useful for compression, since it tends to be easy to compress a string that has runs of repeated characters by techniques such as move-to-front transform and run-length encoding. More importantly, the transformation is reversible, without needing to store any additional data except the position of the first original character. The BWT is thus a "free" method of improving the efficiency of text compression algorithms, costing only some extra computation. 

__Properties__
* Worst-case performance  O(n)

[burrows-wheeler-wiki]: https://en.wikipedia.org/wiki/Burrows%E2%80%93Wheeler_transform


### [Knuth Morris Pratt](./knuth_morris_pratt.rs)
From [Wikipedia][kmp-wiki]: searches for occurrences of a "word" W within a main "text string" S by employing the observation that when a mismatch occurs, the word itself embodies sufficient information to determine where the next match could begin, thus bypassing re-examination of previously matched characters.
  Knuth Morris Pratt search runs in linear time in the length of W and S.

__Properties__
* Case performance  O(s + w)
* Case space complexity  O(w)

[kmp-wiki]: https://en.wikipedia.org/wiki/Knuth–Morris–Pratt_algorithm



### [Manacher](./manacher.rs)
From [Wikipedia][manacher-wiki]: find a longest palindrome in a string in linear time.

__Properties__
* Worst-case time complexity is O(n)
* Worst-case space complexity is O(n)

[manacher-wiki]: https://en.wikipedia.org/wiki/Longest_palindromic_substring#Manacher's_algorithm


### [Rabin Karp](./rabin_karp.rs)
From [Wikipedia][rabin-karp-wiki]: a string-searching algorithm created by Richard M. Karp and Michael O. Rabin that uses hashing
to find an exact match of a pattern string in a text.

[rabin-karp-wiki]: https://en.wikipedia.org/wiki/Rabin%E2%80%93Karp_algorithm


### [Hamming Distance](./hamming_distance.rs)
From [Wikipedia][hamming-distance-wiki]: In information theory, the Hamming distance between two strings of equal length is the number of positions at which the corresponding symbols are different. In other words, it measures the minimum number of substitutions required to change one string into the other, or the minimum number of errors that could have transformed one string into the other. In a more general context, the Hamming distance is one of several string metrics for measuring the edit distance between two sequences. It is named after the American mathematician Richard Hamming.

[run-length-encoding-wiki]: https://en.wikipedia.org/wiki/Run-length_encoding

### [Run Length Encoding](./run_length_encoding.rs)
From [Wikipedia][run-length-encoding-wiki]: a form of lossless data compression in which runs of data (sequences in which the same data value occurs in many consecutive data elements) are stored as a single data value and count, rather than as the original run.

[hamming-distance-wiki]: https://en.wikipedia.org/wiki/Hamming_distance
