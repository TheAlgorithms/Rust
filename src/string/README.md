## String Algorithms

### [Knuth Morris Pratt](./knuth_morris_pratt.rs)
From [Wikipedia][kmp-wiki]: searches for occurrences of a "word" W within a main "text string" S by employing the observation that when a mismatch occurs, the word itself embodies sufficient information to determine where the next match could begin, thus bypassing re-examination of previously matched characters.
  Knuth Morris Pratt search runs in linear time in the length of W and S.

__Properties__
* Case performance  O(s + w)
* Case space complexity  O(w)

### [Aho–Corasick algorithm](./aho_corasick.rs)
From [Wikipedia][ac-algo-wiki]: In computer science, the Aho–Corasick algorithm is a string-searching algorithm invented by Alfred V. Aho and Margaret J. Corasick. It is a kind of dictionary-matching algorithm that locates elements of a finite set of strings (the "dictionary") within an input text. It matches all strings simultaneously. The complexity of the algorithm is linear in the length of the strings plus the length of the searched text plus the number of output matches. Note that because all matches are found, there can be a quadratic number of matches if every substring matches (e.g. dictionary = a, aa, aaa, aaaa and input string is aaaa).


[kmp-wiki]: https://en.wikipedia.org/wiki/Knuth–Morris–Pratt_algorithm
[ac-algo-wiki]: https://en.wikipedia.org/wiki/Aho–Corasick_algorithm
