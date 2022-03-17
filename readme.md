# Wordle Clone in Rust

A very simple Wordle clone that runs as a CLI.

## Generating a Wordlist

`grep -o -w '\w\{5\}' /usr/share/dict/words > ./resources/words.txt`
