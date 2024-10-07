LC3b
---

A rusty version of the LC3b computer. Workspace includes these crates:

  * lc3b: emulator which takes a program in assembly format, places the program in memory, and allows stepping through the program all while inspecting the internal state of the computer
  * lc3b-assembler: conversion of assembly to a suitable abstract in-memory representation
  * lc3b-isa: captures the LC-3b spec with rust data types defined in such a way that it is impossible to build an invalid LC-3b program
  * lc3b-web: HTTP server and HTML/CSS/JavaScript for running the emulator in a user-friendly way

Key technologies:

  * [Rust](https://doc.rust-lang.org/book/)
  * Rust code in to WASM enabled by [wasm-pack](https://github.com/rustwasm/wasm-pack)
  * [Pest ](https://pest.rs/) parsing expression grammar (no lexer+parser)

Running locally
---

If you're new to Rust you'll need it's compiler. Install it with [rustup](https://rustup.rs/)

    rustup default stable

Once that is done, clone the codebase to your computer:

    git clone https://github.com/tureus/lc3b-rs.git
    cd lc3b-web/
    cargo run

Wait for things to compile and for the web server to bind to a port. Then hit that web server with your browser. Just go to `http://localhost:3000`

Docs
---

    http://users.ece.utexas.edu/~patt/07s.360N/handouts/360n.appC.pdf
    http://users.ece.utexas.edu/~patt/21s.460n/handouts/appA.pdf
