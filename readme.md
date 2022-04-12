# Wordle Clone in Rust

![image](https://user-images.githubusercontent.com/45131445/163044263-cb84cb42-305e-4e9e-b26b-c61800c22329.png)

A very simple Wordle clone that runs as a CLI,

`cargo run` to start, `cargo build` to build a binary.

## Generating a Wordlist

`grep -o -w '\w\{5\}' /usr/share/dict/words > ./resources/words.txt`![Uploading image.png…]()

