# Rust

## Functions and Macros

### Basics

Macros look like functions except that their name ends with a `!` (*bang*):

```rs
macro_rules! say_hello {
    // empty argument list
    () => {
        // in the braces is the content that the macro expands to
        println!("Hello!")
    };
}

fn main() {
    // This call will expand into `println!("Hello!")`
    say_hello!()
}
```

The arguments of a macro are prefixed by a `$` (*dollar sign*) and type annotated with a designator:

```rs
fn add(a: i32, b: i32) -> i32 {
    a + b
}

macro_rules! add {
    // argument list
    ($a: expr, $b: expr) => {
        // in the braces is the content that the macro expands to
        $a + $b
    };
}

fn main() {
    println!("function: {}", add(2, 3));
    println!("macro:    {}", add!(2, 3));
}
```

Designators:

- `expr` is used for expressions
- `ident` is used for variable/function names
- `literal` is used for literal constants
- ...

For example using the `ident` designator a function can be generated:

```rs
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            // The `stringify!` macro converts an `ident` into a string
            println!("You called {:?}()",
                     stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

fn main() {
    foo();
    bar();
}
```

They are expanded during compile time and allow meta programming.

```rs
macro_rules! describe {
    // two values
    ($a:expr, $b:expr) => {
        println!("Two values: {} and {}", $a, $b);
    };
    // one value
    ($s:expr) => {
        println!("One value: {}", $s);
    };
}

fn main() {
    describe!(10, 20);
    describe!("hello");
}
```

Using a literal token *keyword* multiple cases can be chosen directly:

```rs
macro_rules! calculate {
    (add $a:expr, $b:expr) => {
        $a + $b
    };
    (sub $a:expr, $b:expr) => {
        $a - $b
    };
    (mul $a:expr, $b:expr) => {
        $a * $b
    };
    (div $a:expr, $b:expr) => {
        $a / $b
    };
}

fn main() {
    println!("{}", calculate!(add 10, 5));
    println!("{}", calculate!(sub 10, 5));
    println!("{}", calculate!(mul 10, 5));
    println!("{}", calculate!(div 10, 5));
}
```

One disadvantage compared to functions is that there is no direct way to enforce the type in the pattern (like only allowing numbers).

### Variadic arguments:

```rs

macro_rules! find_min {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Tail call `$y`
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    println!("find_min: {}", find_min!(1, -99, 5, 2 * 3, 4, -10));
}
```

This variadic macro has a macro argument that is a set:

- Instead of `$y:expr`
- `$($y:expr),+` is being used

Given multiple values this will create effectivley the following code recursively applying the macro:

```rs
println!(
    "{}",
    std::cmp::min(
        1,
        std::cmp::min(
            -99,
            std::cmp::min(
                5,
                std::cmp::min(
                    2 * 3,
                    std::cmp::min(
                        4,
                        // Base case => no std::cmp::min
                        -10
                    )
                )
            )
        )
    )
);
```
