# Rust Introduction

A hands-on Rust course for someone coming from C++ (and some Python). Each
lesson is its own crate in this Cargo workspace: a `main.rs` you run to see
worked examples with commentary comparing Rust to C++/Python, plus an
`exercises.rs` that **won't compile until you fix it** -- fixing real
compiler/borrow-checker errors is how the intuition sticks.

## How to work through a lesson

```bash
cargo run -p <crate>     # read the worked examples, run them, see the output
cargo test -p <crate>    # this will FAIL TO COMPILE at first -- that's the exercise
```

Open `src/exercises.rs`, read the `TODO` comments, fix the functions one at a
time, and re-run `cargo test -p <crate>` until everything compiles and all
tests pass. Don't edit the `tests` module -- only the functions above it.

Build/test everything at once from the repo root with `cargo build` /
`cargo test`.

## Lessons

| # | Crate | Topic | Status |
|---|-------|-------|--------|
| 1 | [`01_ownership`](01_ownership) | Ownership, moves, borrowing, `&`/`&mut`, slices | ready |
| 2 | `02_structs_enums` | Structs, enums, pattern matching, `Option`/`Result` | planned |
| 3 | `03_error_handling` | `Result`, `?`, `panic!` vs recoverable errors | planned |
| 4 | `04_collections_iterators` | `Vec`, `HashMap`, iterator combinators | planned |
| 5 | `05_traits_generics` | Traits, generics, trait objects vs C++ templates/virtuals | planned |
| 6 | `06_lifetimes` | Explicit lifetimes, why the compiler needed them | planned |
| 7 | `07_smart_pointers` | `Box`, `Rc`, `RefCell`, interior mutability | planned |
| 8 | `08_concurrency` | Threads, channels, `Arc<Mutex<T>>`, `Send`/`Sync` | planned |

We'll add each lesson's crate to the workspace `Cargo.toml` as we build it.

## Why this format

Coming from C++, most Rust syntax (functions, structs, `match` vs `switch`,
generics) will feel familiar fast. The genuinely new idea is **ownership**:
the compiler statically tracks who owns each value and who's allowed to
read or mutate it, which is why lesson 1 leads with it. Everything else
(lifetimes, smart pointers, the borrow checker's stricter rules around
concurrency) is really just consequences of that one idea, so getting
comfortable with it early pays off for the rest of the course.
