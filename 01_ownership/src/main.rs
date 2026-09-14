// Lesson 1: Ownership & Borrowing
//
// Run this file with:      cargo run -p ownership
// Run the exercises with:  cargo test -p ownership
//
// You already know C++ and some Python, so skip straight to what's new:
// Rust tracks, at compile time, who owns each value and who's allowed to
// look at or modify it. No garbage collector, no manual free(), no
// use-after-free, no data races -- enforced by the compiler ("the borrow
// checker"), not by convention or runtime checks.

#[cfg(test)]
mod exercises;

fn main() {
    move_semantics();
    clone_vs_copy();
    borrowing();
    mutable_borrowing();
    slices();
}

fn move_semantics() {
    println!("\n-- move semantics --");

    // In C++, `std::string b = a;` copies (unless you std::move it).
    // In Python, `b = a` makes b and a refer to the SAME object.
    // In Rust, `let b = a;` for a heap type (String) MOVES ownership.
    // `a` becomes invalid -- the compiler forbids using it afterward.
    let a = String::from("hello");
    let b = a;
    // println!("{a}"); // <- uncomment this: compile error ("value borrowed after move")
    println!("b = {b}");

    // Passing a String into a function moves it too, same rule as `let b = a;`.
    let s = String::from("world");
    takes_ownership(s);
    // println!("{s}"); // <- would also fail to compile: s was moved into the function
}

fn takes_ownership(s: String) {
    println!("took ownership of: {s}");
    // s is dropped (its memory freed) right here, at the end of this scope.
}

fn clone_vs_copy() {
    println!("\n-- clone vs copy --");

    // Want C++-style deep copy semantics? Ask for it explicitly with .clone().
    let a = String::from("hello");
    let b = a.clone();
    println!("a = {a}, b = {b}"); // both valid, this is a real heap copy

    // Simple stack-only types (integers, bool, char, tuples of these...)
    // implement the `Copy` trait: assignment copies instead of moving,
    // just like `int b = a;` in C++.
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}"); // both still valid, no move happened
}

fn borrowing() {
    println!("\n-- borrowing (&T) --");

    // Instead of moving or cloning, you can *borrow* a reference, `&T`.
    // This is like passing `const T&` in C++: read-only access, no copy,
    // no ownership transfer. The original owner is still valid afterward.
    let s = String::from("hello");
    let len = string_length(&s);
    println!("'{s}' has length {len}"); // s is still usable here

    // You can have as many simultaneous immutable borrows as you want.
    let r1 = &s;
    let r2 = &s;
    println!("r1 = {r1}, r2 = {r2}");
}

fn string_length(s: &String) -> usize {
    s.len()
} // s goes out of scope here, but since it's a reference, nothing is dropped

fn mutable_borrowing() {
    println!("\n-- mutable borrowing (&mut T) --");

    // `&mut T` is like a non-const reference/pointer in C++ -- it lets the
    // function modify the caller's data in place.
    let mut s = String::from("hello");
    append_world(&mut s);
    println!("s = {s}");

    // THE KEY RULE, and the thing that's genuinely new coming from C++:
    // for any given value, at any given moment, you may have EITHER
    //   - any number of immutable borrows (&T)     OR
    //   - exactly one mutable borrow (&mut T)
    // never both at once. The compiler enforces this at compile time.
    // This is what makes data races on shared data impossible in safe Rust.
    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; // <- uncomment: compile error, can't mix with r1/r2 alive
    println!("r1 = {r1}, r2 = {r2}");
}

fn append_world(s: &mut String) {
    s.push_str(", world");
}

fn slices() {
    println!("\n-- slices --");

    // A slice `&[T]` (or `&str` for strings) is a borrowed *view* into
    // part of a collection -- like a (pointer, length) pair in C++, but
    // the borrow checker guarantees it can't outlive or outrun the data
    // it points into.
    let v = vec![1, 2, 3, 4, 5];
    let middle = &v[1..4]; // elements at indices 1, 2, 3
    println!("middle slice = {middle:?}");

    let s = String::from("hello world");
    let first_word = &s[0..5];
    println!("first word = {first_word}");
}
