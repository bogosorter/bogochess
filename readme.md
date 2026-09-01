# bogochess

A simple chess engine whose goal is neither to be good nor to be efficient. Time will tell if that changes :)

## About

_bogochess_ is written in Rust. The engine communicates using a subset of the [UCI protocol](https://publish.obsidian.md/modern-uci-doc/UCI+Docs/Intro), which interfaces with a GUI of your choice (I've been using [Cute Chess](https://github.com/cutechess/cutechess)). 

## Building & Testing

To test locally, you can use Rust's `cargo run`. To build a release binary, use `cargo build --release`.

The engine has two test types: [Perft](https://chessprogramming.org/Perft) tests, which test that the correct number of moves is generated up to a given depth, and idempotency tests, which check that undoing moves always sets the board to the initial state. Tests can be run with `cargo test`. However, since some of the tests are rather intensive, I'd advise you to use `cargo test --release`.

Additionally, to compare different search algorithms, there is a benchmark that analyzes the number of nodes explored, nodes per second and other metrics on different initial states, which can be run using `cargo bench`.

## Sources

- Sebastian Lague's [Coding Adventure: Chess](https://www.youtube.com/watch?v=U4ogK0MIzqk&t=12s)
- Sebastian Lague's [Coding Adventure: Making a Better Chess Bot](https://www.youtube.com/watch?v=_vqlIPDR2TU)
- [UCI Docs](https://publish.obsidian.md/modern-uci-doc/UCI+Docs/Intro)
- [Chess Programming Wiki](https://chessprogramming.org/)
