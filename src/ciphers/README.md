## Ciphers

### [Caesar](./caesar.rs)
![alt text][caesar]
In cryptography, a **Caesar cipher**, also known as Caesar's cipher, the shift cipher, Caesar's code or Caesar shift, is one of the simplest and most widely known encryption techniques.<br>
It is **a type of substitution cipher** in which each letter in the plaintext is replaced by a letter some fixed number of positions down the alphabet. For example, with a left shift of 3, D would be replaced by A, E would become B, and so on. <br>
The method is named after **Julius Caesar**, who used it in his private correspondence.<br>
The encryption step performed by a Caesar cipher is often incorporated as part of more complex schemes, such as the Vigenère cipher, and still has modern application in the ROT13 system. As with all single-alphabet substitution ciphers, the Caesar cipher is easily broken and in modern practice offers essentially no communication security.
###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Caesar_cipher)

### [Polybius](./polybius.rs)
The **Polybius square**, also known as the Polybius checkerboard, is a device invented by the ancient Greeks Cleoxenus and Democleitus, and made famous by the historian and scholar Polybius.<br>
The device is used for fractionating plaintext characters so that they can be represented by a smaller set of symbols, which is useful for telegraphy, steganography, and cryptography.<br>
The **Polybius square** is also used as a basic cipher called the Polybius cipher. This cipher is a **substitution cipher** with characters being substituted for pairs of digits.

#### Example cipher
 Δ | 1 | 2 | 3 |  4  | 5
---|---|---|---| --- |---
1  | a | b | c |  d  | e
2  | f | g | h | i/j | k
3  | l | m | n |  o  | p
4  | q | r | s |  t  | u
5  | v | w | x |  y  | z
###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Polybius_square)

### [Vigenère](./vigenere.rs)
The **Vigenère cipher** is a method of encrypting alphabetic text by using a series of **interwoven Caesar ciphers** based on the letters of a keyword. It is **a form of polyalphabetic substitution**.<br>
The Vigenère cipher has been reinvented many times. The method was originally described by Giovan Battista Bellaso in his 1553 book La cifra del. Sig. Giovan Battista Bellaso; however, the scheme was later misattributed to Blaise de Vigenère in the 19th century, and is now widely known as the "Vigenère cipher".<br>
Though the cipher is easy to understand and implement, for three centuries it resisted all attempts to break it; this earned it the description **le chiffre indéchiffrable**(French for 'the indecipherable cipher'). 
Many people have tried to implement encryption schemes that are essentially Vigenère ciphers. Friedrich Kasiski was the first to publish a general method of deciphering a Vigenère cipher in 1863.
###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher)

### [SHA-2](./sha256.rs)
SHA-2 (Secure Hash Algorithm 2) is a set of cryptographic hash functions designed by the United States National Security Agency (NSA) and first published in 2001. They are built using the Merkle–Damgård structure, from a one-way compression function itself built using the Davies–Meyer structure from a (classified) specialized block cipher. 
###### Source: [Wikipedia](https://en.wikipedia.org/wiki/SHA-2)

### [Transposition](./transposition.rs)
In cryptography, a **transposition cipher** is a method of encryption by which the positions held by units of plaintext (which are commonly characters or groups of characters) are shifted according to a regular system, so that the ciphertext constitutes a permutation of the plaintext. That is, the order of the units is changed (the plaintext is reordered).<br> 
Mathematically a bijective function is used on the characters' positions to encrypt and an inverse function to decrypt.
###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Transposition_cipher)

[caesar]: https://upload.wikimedia.org/wikipedia/commons/4/4a/Caesar_cipher_left_shift_of_3.svg

### [AES](./aes.rs)
The Advanced Encryption Standard (AES), also known by its original name Rijndael (Dutch pronunciation: [ˈrɛindaːl]), is a specification for the encryption of electronic data established by the U.S. National Institute of Standards and Technology (NIST) in 2001.

###### Source: [Wikipedia](https://en.wikipedia.org/wiki/Advanced_Encryption_Standard)

![aes](https://upload.wikimedia.org/wikipedia/commons/5/50/AES_%28Rijndael%29_Round_Function.png)