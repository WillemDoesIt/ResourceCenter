---
description: Docs of the lib.rs file
tags:
 - 
---

`lib.rs` relies on modules which have documentation here:
See [[Utils.md]] for details upon the `utils.rs` file

### File access

Because wasm doesn't have file system access, the db will have to be retrieved with a fetch, of which [this](https://rustwasm.github.io/wasm-bindgen/examples/fetch.html?highlight=fetch#the-fetch-api) was an incredible guide. Next was turning the buffer into a `Vec<u8>`. After [searching](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Response.html?search=Responce%20array) I found a few things I thought could help, and [another search](https://rustwasm.github.io/wasm-bindgen/api/web_sys/?search=ArrayBuffer) I found converting to `array_buffer` and then to `Uint8Array` to `Vec<u8>` works.

See [[Sqlite.md]] for details upon the `sqlite.rs` query function that is next used to get a output Table of a SQL query.

Next we had to dynamically load html into a div, [this guide](https://rustwasm.github.io/wasm-bindgen/examples/dom.html) was our starting point for that. The whole "`wasm-bindgen` Guide" was extremely useful. [This guide](https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html?highlight=dyn_into#srclibrs) as well showed how to get element by id, looking this up web_sys's crates page we found you can do a lot with the [Element Type](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html) Including what we needed `set_inner_html`

Then loading content was a pretty simple iteration through the table putting each item into the 
