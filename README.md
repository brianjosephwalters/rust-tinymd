# A Rust markdown compiler
Following:
* [Getting Started with Rust by Building a Tiny Markdown Compiler](https://jesselawson.org/rust/getting-started-with-rust-by-building-a-tiny-markdown-compiler/)

### Take Aways:
* Ch 2: Functions, return/early return, expression-based languages.
* Ch 3: heap-alloctated/stack-allocated strings (String vs. &str), env! macro, the borrow checker
* Ch 4: commandline arguments, assignment moves the value not just copies it, match, passing arguments to functions, 
* Ch 5: Path/File, expect() unwraps Result type, panic!() respects return type of open(), Buffered Readers, ignoring the err path when unwrapping by unwrap(), Option (Some() and None()), Write

### Notes:
*  Really like this website.
*  The parsing logic seems unnecessarily complicated for these markup symbols.
*  Does Rust have a streaming/yield syntax where the input could be passed along and written before the entire file has even been read?
*  How do to unit testing in Rust?
