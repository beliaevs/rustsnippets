# Creating and Building cargo project
0. `cargo new <project>` creates new directory with your project. 
1. `cargo build` or `cargo build -r` if you want optimized release build.
2. `cargo clippy` finds warnings, especially if you add `#[warn(clippy::pedantic)]` into the source.
3. `cargo run` or `cargo run -r` for program launch.
