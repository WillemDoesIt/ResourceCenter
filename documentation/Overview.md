---
description: The main doc page
tags:
 - 
---

This project has no back-end. Rather all of the SQL queries are done client side through Web Assembly (WASM). This wasm is compiled from rust. The database is SQLite. The website is designed in plain HTML/CSS, and just 3 lines of javascript.

Making the website static is to solve serious scalability and security problems, World Relief wouldn't have a server capable of the traffic this website needs to manage. Other than that willem personally loves the philosophy of the JAMstack, and we both just don't like JavaScript. But SQL wasm is a niche that would come with it's own challenges, so we went with a language most capable language for wasm: rust.

See [[Project_Structure.md]] for more detail on the why we choose the file structure we did and what each directory and file is for.
See [[Schema.md]] for details upon the database structure
See [[Lib.md]] for details upon the `lib.rs` file, which is essentially the "main" rust file.
See [[Web.md]] for details upon the `web/` development
See [[Misc.md]] for all else