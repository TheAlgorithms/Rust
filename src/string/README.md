## String Algorithms

### [Knuth Morris Pratt](./knuth_morris_pratt.rs)
From [Wikipedia][kmp-wiki]: searches for occurrences of a "word" W within a main "text string" S by employing the observation that when a mismatch occurs, the word itself embodies sufficient information to determine where the next match could begin, thus bypassing re-examination of previously matched characters.
  Knuth Morris Pratt search runs in linear time in the length of W and S.

__Properties__
* Case performance  O(s + w)
* Case space complexity  O(w)

### [Rabin Karp](./rabin_karp.rs)
From [Wikipedia][]In computer science, the Rabin–Karp algorithm or Karp–Rabin algorithm is a string-searching algorithm created by Richard M. Karp and Michael O. Rabin (1987) that uses hashing to find an exact match of a pattern string in a text. It uses a rolling hash to quickly filter out positions of the text that cannot match the pattern, and then checks for a match at the remaining positions. Generalizations of the same idea can be used to find more than one match of a single pattern, or to find matches for more than one pattern.

__Properties__
* Average time complexit  O(m + n)

[kmp-wiki]: https://en.wikipedia.org/wiki/Rabin%E2%80%93Karp_algorithm
