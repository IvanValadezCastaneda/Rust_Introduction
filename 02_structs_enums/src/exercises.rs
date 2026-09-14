// Exercises for Lesson 2: Structs, Enums, Pattern Matching, and Option.
//
// This file does NOT compile yet -- on purpose, same as lesson 1. Work
// through the TODOs in order, then run:
//
//     cargo test -p structs_enums
//
// Fix each function/type so the whole crate compiles and every test
// passes. Don't change the tests at the bottom -- only the code above them.

struct Rectangle {
    width: u32,
    height: u32,
}

// TODO 1: `consume` takes `self` by value, which MOVES the Rectangle into
// the method -- same move rule as lesson 1, just applied to `self`. The
// exercise calls it twice on the same `rect`, so the second call uses a
// moved value. Fix `consume`'s signature so it *borrows* self instead of
// consuming it (you'll need to change how the field is accessed too,
// since `self` will now be a reference).
impl Rectangle {
    fn consume(self) -> u32 {
        self.width * self.height
    }
}

pub fn exercise1_area_twice() -> u32 {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let first = rect.consume();
    let second = rect.consume();
    first + second
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// TODO 2: this `match` only handles two of the three `TrafficLight`
// variants. Rust requires every `match` to be exhaustive, so this won't
// compile. Add the missing `TrafficLight::Yellow` arm -- have it return
// "slow down".
fn describe(light: TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "stop",
        TrafficLight::Green => "go",
    }
}

pub fn exercise2_describe_yellow() -> &'static str {
    describe(TrafficLight::Yellow)
}

fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

// TODO 3: `find_first_even` returns `Option<i32>`, not `i32` -- there
// might not be an even number in the slice. This function tries to use
// the Option directly as if it were a plain i32, which is a type
// mismatch. Fix it by unwrapping the Option: use `.unwrap_or(0)` so a
// missing even number counts as 0 instead of crashing.
pub fn exercise3_first_even_doubled(numbers: &[i32]) -> i32 {
    let found: i32 = find_first_even(numbers);
    found * 2
}

struct Point {
    x: i32,
    y: i32,
}

// TODO 4: `==` doesn't work on `Point` yet -- Rust doesn't generate
// equality comparisons for your types automatically (unlike C++, where
// you'd get an error too, but for a different reason: no operator==
// overload defined). Fix it by adding `#[derive(PartialEq)]` directly
// above `struct Point` -- this asks the compiler to generate a
// field-by-field equality check for you.
pub fn exercise4_points_equal() -> bool {
    let a = Point { x: 1, y: 2 };
    let b = Point { x: 1, y: 2 };
    a == b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exercise1_area_twice() {
        assert_eq!(exercise1_area_twice(), 3000);
    }

    #[test]
    fn test_exercise2_describe_yellow() {
        assert_eq!(exercise2_describe_yellow(), "slow down");
    }

    #[test]
    fn test_exercise3_first_even_doubled() {
        assert_eq!(exercise3_first_even_doubled(&[1, 3, 4, 5]), 8);
    }

    #[test]
    fn test_exercise4_points_equal() {
        assert!(exercise4_points_equal());
    }
}
