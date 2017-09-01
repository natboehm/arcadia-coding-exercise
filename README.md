##Arcadia Coding Exercise

I implemented the beginnings of the blogging Library in Rust. The library compiles,
however I did not have the time to complete much functionality or a main function
to demonstrate functionality.

Rust version: 1.19

To compile: `Cargo Build`

To run: `Cargo run`

(Running will only print 'hello world')

When compiled there will be numerous warnings as output, they're just letting
you know that the variables are not used. This is something I really like
about the Rust compiler, that it tells you where you can have dead code on
compilation. On crates.io these are enforced as compiler errors in order to
keep the code as clean as possible.

The library was designed to have three main structs, one representing the Blog,
one representing a post, another representing a comment. A blog has many posts,
a post has many comments. The blog should have been implemented in the `impl`
block, and called by a main function in `src/main.rs`.

The search functions would have been implemented alongside the blog, and would
have been useful in the CRUD functionality for the app. To update the author,
body, or labels, we would have had to search for the old and replace it with
the new.

Unit tests would have been written at the bottom of `src/lib.rs`. These would
have tested each function, ideally each function returning something to check
against.
