---
description: Documentation of `sqlite.rs`
tags:
 - 
---

We wanted to use a database that would be lightweight and open source. The alternative would have been PostgreSQL, but that seemed like more trouble than it was worth to learn given the time constraints.

The [`sqite-wasm-rs` crate page](https://crates.io/crates/sqlite-wasm-rs) gives a few details on how it works. But from what I can see there is a serious lack of documentation. Luckily it appears to be a direct port of sqlite, or a interface to it. Which means whilst not all methods have support yet, the ones that do have parity. They follow the exact same documentation on [sqlite.org](https://sqlite.org/).

### Opening

For example, the crate page shows how to implement `ffi::sqlite3_open_v2` and searching `sqlite3_open_v2` on sqlite.org gives you this page, [Flags For File Operations](https://sqlite.org/c3ref/c_open_autoproxy.html) which links to this page [Opening a New Database Connection](https://sqlite.org/c3ref/open.html) which shows the v_2 option with really good documentation.

And the implementation in the crates page is really useful, as for example the filename that is documented as a `const char *filename` on sqlite.org, is achieved by doing `c"mem.db".as_ptr().cast()`, which means now if we see `const char *` again, we will know how to convert a type to that.

It also uses v_2 which requires the "Name of virtual file system (VFS) module to use", wherein `std::ptr::null()` is used, which selects "the default sqlite3_vfs object", this links to [OS Interface Object](https://sqlite.org/c3ref/vfs.html) which further explains vfs with sqlite. 

### Deserialization

I believe this is important because wasm doesn't run on the os, and doesn't have access to the file system according to [a blog](https://www.jamesfmackenzie.com/2019/12/08/webassembly-loading-files/). So, `mem.db` isn't accessing a file, but creating one in a vfs, or ("as an in-memory", not sure the difference yet) and opening it. Which means we need to load data into it, which appears to be called deserialization as [discussed on a sqlite.org forum](https://sqlite.org/forum/info/6df5220c4645510a). Sqlite documents [Deserializing a database](https://sqlite.org/c3ref/deserialize.html) which shares functionality in the [crates documentation](https://docs.rs/sqlite-wasm-rs/0.3.4/sqlite_wasm_rs/fn.sqlite3_deserialize.html). What wasn't clear was what zSchema was, cause "which db to open" could mean filename, or a bunch of other things, but it said "It is not possible to deserialized into the TEMP database", so after [searching "temp database"](https://sqlite.org/search?s=d&q=temp+database") I found different options, I tried c"main", and the `$core::panicking::assert_failed::ha62580e55d7391b5` error finally went away. 

### Prepare statement

`sqlite-wasm-rs` only had [prepare_v2 and prepare_v3](https://docs.rs/sqlite-wasm-rs/0.3.2/sqlite_wasm_rs/?search=prepare) and "sqlite3_prepare_v3() has an extra "prepFlags" option that is used for special purposes." - [sqlite docs](https://sqlite.org/c3ref/prepare.html), So we went with prepare_v2 assuming it would be simpler.

The attribute `zSql` was yet again a `*const c_char` but I think `c"{query}"` would just return a CString of `{query}` rather than the contents, so instead we found these [CString docs](https://doc.rust-lang.org/std/ffi/c_str/struct.CString.html#), which details how to convert to CString. Next, [sqlite.org](https://sqlite.org/c3ref/prepare.html) said "If the nByte argument is negative, then zSql is read up to the first zero terminator." This appears to be something around performance and I think -1 is either default or +1 is, just another I can test later in debugging. Next, "ppStmt is left pointing to a compiled prepared statement that can be executed using sqlite3_step()" so we made a variable called `statement_handler` that starts as a `std::ptr::null_mut();` just to get the types to match, and then get's loaded with the handler so we can do `sqlite3_step()` on it later. Next, is pzTail, "If pzTail is not NULL then *pzTail is made to point to the first byte past the end of the first SQL statement in zSql." which basically means that pzTail is only used in case you are preparing multiple statements, but we don't plan to create that functionality for this function, so we just give it the value of `std::ptr::null_mut()`.

### Loading the table

We define a table `struct` to be attributes, a one dimensional vector of strings, and contents, a vector of the records (rows), where each record is a hashmap.

This means accessing contents looks like this:
```rust
let output = sqlite::query(db, "SELECT `website`, `email` FROM Contacts");
log(&output.contents[0]["email"]);
```

There is a for loop to iterate through the columns getting there name with [ffi::sqlite3_column_name](https://sqlite.org/c3ref/column_name.html) which is converted to a String and pushed onto a attributes vector that will be put into the table. Next is contents, `ffi::sqlite3_step(statement_handler)` does a step in the query, and returns a value where if it's `ffi::SQLITE_ROW` means that it "has another row ready" [Result Codes](https://sqlite.org/c3ref/c_abort.html) which is why it's the condition in the while loop. From there we can get the message by converting back form CString, which requires an unsafe block because it's pointer manipulation given strings in c are just char iterators. We get [Result Values From A Query](`https://sqlite.org/c3ref/column_blob.html`) we step through the result and reocrd values with `ffi::sqlite3_column_text`, each record get's it's own hashmap, and all the hashmaps are put in a vector, named contents. Then a table loaded with attributes and contents is returned.


##### logging the table

`print_table` is a simple function that iterates through the table and prints it out in a way that respects multiple columns into the terminal. This also relies on a helper function called `force_length` that forces a string to be printed at a uniform length, by either truncating for too long of strings, or adding invisible whitespace.
