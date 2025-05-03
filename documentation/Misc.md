---
description: All misc docs, probably will be mostly things from `utils.rs`
tags:
 - 
---

I reviewed [method syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html) and [traits](https://doc.rust-lang.org/book/ch10-02-traits.html) to create the `.expect_log()` method that works on Options and Result types, given that I wanted to be able to use expect like usually, but that doesn't work with wasm because things get logged to console, not to the terminal.

`simplify_url` was just really basic string manipulation.