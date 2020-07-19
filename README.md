# Rust B-tree
During the spring of 2020, one of my CS classes introduced me to the Rust programming language. The courses started with a couple weeks of Ruby, then 8 weeks or so of Ocaml, and then some closing remarks on Rust. Since, I got such a brief introduction, I wanted to learn more by creating this project. 

# Notes

Naming conventions: https://doc.rust-lang.org/1.0.0/style/style/naming/README.html
Creating structs: https://doc.rust-lang.org/book/ch05-01-defining-structs.html

Functions that "don't" return a value, actually return the unit type `()`
```rust
 pub fn traverse() -> () { 
     ...
 }
```

## TODO

figure out traits:
https://blog.rust-lang.org/2015/05/11/traits.html