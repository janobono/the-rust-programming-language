# the-rust-programming-language
My journey through [rust language](https://www.rust-lang.org).

## motivation
As a JAVA developer, I have to deal mentally with the exaggerated demands on the system resources of even relatively 
simple programs on a daily basis. 

A major turnaround occurred at a time when applications began to be delivered as Docker containers, despite the 
SpringBoot framework used, the results were not sufficient.

I've always tried to find a way so maybe RUST will be the way. As a source I will use 
[The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html) and 
[Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/). 

## install
[The Rust installation page](https://www.rust-lang.org/learn/get-started)
- Execute `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Modify `.bashrc` add `source ~/.cargo/env`
- To test call for example `cargo --version`

## doc
```
rustup doc --std
```

## content
- [basics](./basics/README.md)
- [advanced topics](./advanced/README.md)
