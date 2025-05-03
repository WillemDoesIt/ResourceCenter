---
description: Overview of project structure
tags:
 - 
---

# Overview

Setting up the minimal wasm project was done by following [this tutorial](https://www.youtube.com/watch?v=y_nGGbM2eaU).

This is where the resulting file tree came from. A `src/` folder for the rust, a `web/` directory for what will be served, the compiled wasm being within `web/wasm/` and database that must be accessible via fetch is thus in web at `web/database/`. Documentation in `/documentation/` and other files like `cargo.toml` and `cargo.lock` are there just because it's required from rust projects. `deploy.sh` is there to automate the process of compilation and serving the web directory. The `target/` directory is also a consequence of choosing rust, this is where the compiled `.wasm` file goes before `wasm-bindgen-cli` creates js/ts bindings and puts it in `web/wasm/`

### Nix flake

We decided to setup a Nix flake, because they allow for unmatched reproducibility, debatably better than Docker containers. In a project this big, the amount of dependencies required can be a lot, and installation process can differ from one Windows machine to another. But with Nix flakes, it's a lot harder to make an error in the "Usage" part of the `README`, because no matter how many dependencies if `nix develop` works on one machine, it will work on any.

### web/ directory

From there we decided to keep all of the web pages, like `index.html` into a `web/` directory to keep things more organized. From there, the database needs to be accessible from the webpage, so we put it in the `web/` directory.
In this is a favicon and we choose to setup a `format.css` as well as a `style.css` to be separate as one decides the positions of divs, whilst the other decides colors, borders, shadows, rounding of edges, things like that. This directory also has a `favicon.ico`.

### documentation/ directory

This is where markdown files like this one will be kept to store information to explain what certain things do, and why we chose to make them do what they do. The documentation is not here to explain how things work, instead code should be written well enough to be clear on how it's doing what it's doing. Each file also cites it's references anyways, so the `documenation/` directory shouldn't have to play too much of a role.
