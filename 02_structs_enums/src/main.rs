// Lesson 2: Structs, Enums, Pattern Matching, and Option
//
// Run this file with:      cargo run -p structs_enums
// Run the exercises with:  cargo test -p structs_enums
//
// None of this is conceptually new coming from C++ -- structs, methods,
// tagged unions, switch-like dispatch, nullable values -- you've used all
// of these before. What's new is how much stricter and more explicit Rust
// is about each one. Ownership from lesson 1 still applies everywhere here:
// methods borrow or consume `self` using the exact same rules as any other
// value.

#[cfg(test)]
mod exercises;

fn main() {
    structs();
    methods();
    enums();
    pattern_matching();
    option_type();
}

// -- structs --

// Like a C++ struct/aggregate, or a Python class with only __init__ fields.
// No inheritance, no constructors by convention -- you build one with a
// struct literal.
struct Rectangle {
    width: u32,
    height: u32,
}

fn structs() {
    println!("\n-- structs --");

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    // Just like C++: dot syntax for field access.
    println!("rect is {} x {}", rect.width, rect.height);
}

// -- methods (impl blocks) --

// C++ puts methods inside the class body. Rust separates them into an
// `impl` block attached to the struct by name -- you can even have
// multiple `impl Rectangle` blocks, they all just add to the same type.
impl Rectangle {
    // `&self` is like an implicit `const Rectangle&` in C++ (the `this`
    // pointer, but read-only and explicit). Ownership rules from lesson 1
    // apply: `&self` borrows, `&mut self` borrows mutably, and plain
    // `self` (no &) *consumes* the value -- after calling a `self` method,
    // the original variable is moved and can't be used again.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // No `self` parameter at all = an "associated function", like a
    // C++ static member function. Called as `Rectangle::square(10)`,
    // not on an instance.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn methods() {
    println!("\n-- methods --");

    let mut rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("area = {}", rect.area()); // &self: just borrows, rect still usable

    rect.scale(2); // &mut self: needs `rect` to be `mut`
    println!("scaled area = {}", rect.area());

    let sq = Rectangle::square(10); // associated function, no instance needed
    println!("square area = {}", sq.area());
}

// -- enums --

// A C++ `enum class` can only hold one of a fixed set of bare labels.
// A Rust enum is far closer to `std::variant` (or a tagged union): each
// variant can carry its *own* data, of different shapes.
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

fn enums() {
    println!("\n-- enums --");

    let shapes = vec![
        Shape::Circle { radius: 2.0 },
        Shape::Rectangle {
            width: 3.0,
            height: 4.0,
        },
        Shape::Triangle {
            base: 5.0,
            height: 6.0,
        },
    ];

    for shape in &shapes {
        println!("area = {}", area(shape));
    }
}

// -- pattern matching --

// `match` is like a `switch`, but stricter in two ways that matter:
//   1. It must be EXHAUSTIVE -- every variant needs an arm, or it won't
//      compile. No silent "forgot a case" bugs.
//   2. There's no fallthrough. Each arm is its own isolated block.
// It also *destructures* -- each arm can pull the data straight out of
// the matched variant, instead of you manually unpacking a union by hand.
fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rectangle { width, height } => width * height,
        Shape::Triangle { base, height } => 0.5 * base * height,
    }
}

fn pattern_matching() {
    println!("\n-- pattern matching --");

    // match also works on plain values, like a switch over an int.
    let n = 4;
    let description = match n {
        0 => "zero",
        1 | 2 | 3 => "small", // multiple patterns per arm
        4..=9 => "medium",    // inclusive range pattern
        _ => "large",         // `_` is the required "everything else" arm
    };
    println!("{n} is {description}");
}

// -- Option<T> --

// C++ has nullptr (any pointer might silently be null -- the type system
// doesn't tell you). Python has None (any variable might silently be None).
// Both let you forget to check, and find out at runtime via a crash.
//
// Rust has no null at all. A value that might be absent is wrapped in
// `Option<T>`, an enum with two variants: `Some(T)` and `None`. A plain
// `T` can NEVER be missing -- if a function's return type is `i32`, you
// are guaranteed a real i32, not "an i32 or maybe secretly nothing."
// The compiler forces you to handle the "nothing" case before you can
// get at the value.
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

fn option_type() {
    println!("\n-- Option<T> --");

    let numbers = [1, 3, 5, 4, 7];

    // match handles both cases explicitly -- the compiler won't let you
    // forget the None arm.
    match find_first_even(&numbers) {
        Some(n) => println!("first even number: {n}"),
        None => println!("no even number found"),
    }

    // `if let` is shorthand for "I only care about the Some case right now".
    if let Some(n) = find_first_even(&numbers) {
        println!("(if let) first even number: {n}");
    }

    // .unwrap_or(default) unwraps or falls back to a default value --
    // handy when None isn't really exceptional.
    let n = find_first_even(&[1, 3, 5]).unwrap_or(0);
    println!("first even, or 0: {n}");

    // .unwrap() panics (crashes) if the value is None. Only reach for this
    // when you've already proven the value can't be missing -- it's the
    // same "trust me" risk as dereferencing an unchecked pointer in C++.
}
