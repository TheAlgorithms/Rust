## String Algorithms

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
