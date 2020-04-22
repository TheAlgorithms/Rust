## String Algorithms

### [Knuth Morris Pratt](./knuth_morris_pratt.rs)
From [Wikipedia][kmp-wiki]: searches for occurrences of a "word" W within a main "text string" S by employing the observation that when a mismatch occurs, the word itself embodies sufficient information to determine where the next match could begin, thus bypassing re-examination of previously matched characters.
  Knuth Morris Pratt search runs in linear time in the length of W and S.

__Properties__
* Case performance  O(s + w)
* Case space complexity  O(w)

[kmp-wiki]: https://en.wikipedia.org/wiki/Knuth–Morris–Pratt_algorithm

### [Palindromic Tree](./src/string/palindromic_tree.rs)
An algorithm that solves the maximum length of palindromic substrings, the number of palindromic substrings and some other palindromic problems.