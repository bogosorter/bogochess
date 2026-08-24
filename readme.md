# bogochess

A simple chess engine whose goal is neither to be good nor to be efficient. Time will tell if that changes :)

## About

_bogochess_ is written in Rust (the code is still somewhat crappy, don't look at it!). The engine communicates using a subset of the [UCI protocol](https://publish.obsidian.md/modern-uci-doc/UCI+Docs/Intro), which interfaces with a GUI of your choice (I've been using [Cute Chess](https://github.com/cutechess/cutechess)). 

## Building & Testing

To test locally, you can use Rust's `cargo run`. To build a release binary, use `cargo build --release`.

The engine has three different test types. All of them use [Perft](https://chessprogramming.org/Perft) positions suggested in the [Chess Programming Wiki](https://chessprogramming.org/Perft_Results#initial-position), and can be run with `cargo test`:

- Perft tests
- Idempotency tests, which ensure that undoing moves always sets the board to the initial state
- Pruning tests, which ensure that pruning the search tree does not result in worse moves

Additionally, to test and compare different pruning methods, there is a benchmark that compares the number of nodes explored by different methods, which can be run using `cargo bench`.

## Sources

- Sebastian Lague's [Coding Adventure: Chess](https://www.youtube.com/watch?v=U4ogK0MIzqk&t=12s)
- Sebastian Lague's [Coding Adventure: Making a Better Chess Bot](https://www.youtube.com/watch?v=_vqlIPDR2TU)
- [UCI Docs](https://publish.obsidian.md/modern-uci-doc/UCI+Docs/Intro)
- [Chess Programming Wiki](https://chessprogramming.org/)
  - [Perft](https://chessprogramming.org/Perft)
  - [Perft Results](https://chessprogramming.org/Perft_Results#initial-position)
  - [Quiescence Search](https://chessprogramming.org/Quiescence_Search)
