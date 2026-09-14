// Exercises for Lesson 1: Ownership & Borrowing.
//
// This file does NOT compile yet -- on purpose. Wrestling with real
// compiler errors is the fastest way to build intuition for the borrow
// checker. Work through the TODOs in order, then run:
//
//     cargo test -p ownership
//
// Fix each function so the whole crate compiles and every test passes.
// Don't change the tests at the bottom -- only the functions above them.

// TODO 1: `a` is moved into `b`, then used again on the next line.
// Fix it without changing what the function returns (still "hello hello").
// Hint: you want two independent, equal Strings. Look at `.clone()`.
pub fn exercise1_move() -> String {
    let a = String::from("hello");
    let b = a;
    format!("{a} {b}")
}

// TODO 2: `r1` borrows `s` immutably, then `s.push_str(..)` tries to
// borrow it mutably while `r1` is still alive later in the function.
// Fix it by reordering so the mutation happens before the immutable
// borrow is taken -- the returned string should still be
// "hello world / hello world".
pub fn exercise2_borrow_conflict() -> String {
    let mut s = String::from("hello");
    let r1 = &s;
    s.push_str(" world");
    format!("{r1} / {s}")
}

// TODO 3: this function returns a reference into `local`, a String owned
// by the function itself. `local` is dropped when the function returns,
// so the reference would dangle -- the borrow checker rejects this.
// Fix it by returning an owned `String` instead of a borrowed `&str`
// (change the return type, and return `local` itself instead of `&local`).
pub fn exercise3_dangling() -> &'static str {
    let local = String::from("dangling");
    &local
}

// TODO 4: `sum` takes ownership of the Vec, so the second call fails --
// `numbers` was already moved into the first call. Fix `sum`'s
// parameter type so it borrows a slice (`&[i32]`) instead of taking
// ownership, and update both call sites to pass a reference.
pub fn exercise4_sum_twice(numbers: Vec<i32>) -> i32 {
    let first = sum(numbers);
    let second = sum(numbers);
    first + second
}

fn sum(numbers: Vec<i32>) -> i32 {
    numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise1_move() {
        assert_eq!(exercise1_move(), "hello hello");
    }

    #[test]
    fn test_exercise2_borrow_conflict() {
        assert_eq!(exercise2_borrow_conflict(), "hello world / hello world");
    }

    #[test]
    fn test_exercise3_dangling() {
        assert_eq!(exercise3_dangling(), "dangling");
    }

    #[test]
    fn test_exercise4_sum_twice() {
        assert_eq!(exercise4_sum_twice(vec![1, 2, 3]), 12);
    }
}
