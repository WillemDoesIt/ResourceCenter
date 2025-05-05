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

Then loading content was a pretty simple iteration through the table putting each item into the div.

Next, was actually getting user input and turning that in a query. [This guide](https://rustwasm.github.io/wasm-bindgen/examples/weather_report.html) was the first thing I looked at. It showed me how to get input from an `<input type="text" />` box, but that was useless cause it happens right when the page is opened when it needs to be triggered by the search button. This is done in js by something called closures, which luckily the wasm-bindgen book had [a chapter on closures](https://rustwasm.github.io/wasm-bindgen/examples/closures.html) which gave all the info I needed to get `<input type="button" />` input to trigger a function that reads the `<input type="text" />`. This also means I was able to put all the dynamically loading div code into a function that is called once the button is triggered. 

Now that we are getting inputs we can start putting that into the sql, I am not sure how to safegaurd against injections, but given each word is split into a iterator I think that will do enough. For now it just checks if the first word is in the title of the org or service:
```rust
("...
WHERE O.name LIKE '%{0}%' OR S.name LIKE '%{0}%';", words.next().unwrap_or(""));
```