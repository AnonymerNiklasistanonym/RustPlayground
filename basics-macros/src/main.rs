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

macro_rules! describe {
    // case with 2 arguments
    ($a:expr, $b:expr) => {
        println!("Two values: {} and {}", $a, $b);
    };
    // case with 1 argument
    ($s:expr) => {
        println!("One value: {}", $s);
    };
}

macro_rules! calculate {
    // case with a literal token 'keyword'
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
    // keywords can also be between arguments
    ($a:expr; add $b:expr) => {
        $a + $b + 1
    };
    ($a:expr; $b:expr; add) => {
        $a + $b + 2
    };
    (x $a:expr; add $b:expr) => {
        $a + $b + 3
    };
}

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
    println!("function: {}", add(2, 3));
    println!("macro:    {}", add!(2, 3));

    describe!(10, 20);
    describe!("hello");

    println!("add(10,5): {}", calculate!(add 10, 5));
    println!("sub(10,5): {}", calculate!(sub 10, 5));
    println!("mul(10,5): {}", calculate!(mul 10, 5));
    println!("div(10,5): {}", calculate!(div 10, 5));

    println!("cal!(10; add 5):   {}", calculate!(10; add 5));
    println!("cal!(x 10; add 5): {}", calculate!(x 10; add 5));
    println!("cal!(10; 5; add):  {}", calculate!(10; 5; add));

    foo();
    bar();

    println!("find_min: {}", find_min!(1, -99, 5, 2 * 3, 4, -10));
}
