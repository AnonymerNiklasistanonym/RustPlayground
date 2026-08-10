//! Hint: main.rs is not included in the generated API documentation!

// do not warn if code is 'dead' since this is an introduction
#![allow(dead_code)]
// do not warn if variables are unused
#![allow(unused_variables)]
// do not warn if assigned values are not being read
#![allow(unused_assignments)]

// find the module called arrays.rs
mod arrays;
mod constants;
mod enums;
mod expressions;
mod flow_control;
mod operators;
mod pointers;
mod primitives;
mod print;
mod strings;
mod structs;
mod tuples;

/// Hint: anything in main.rs is usually not included in the generated API documentation!
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Single line comment

/*
 * Multiline comment
 */

fn main() {
    #[cfg(debug_assertions)]
    println!("debug");
    #[cfg(not(debug_assertions))]
    println!("release");

    println!("{}", add(2, 3));

    println!("-----------[     print_demo     ]-----------");
    print_demo!();
    println!("-----------[  primitives_demo   ]-----------");
    primitives_demo!();
    println!("-----------[  operators_demo    ]-----------");
    operators_demo!();
    println!("-----------[     tuples_demo    ]-----------");
    tuples_demo!();
    println!("-----------[     arrays_demo    ]-----------");
    arrays_demo!();
    println!("-----------[    structs_demo    ]-----------");
    structs_demo!();
    println!("-----------[     enums_demo     ]-----------");
    enums_demo!();
    println!("-----------[   constants_demo   ]-----------");
    constants_demo!();
    println!("-----------[    strings_demo    ]-----------");
    strings_demo!();
    println!("-----------[  expressions_demo  ]-----------");
    expressions_demo!();
    println!("-----------[ flow_control_demo  ]-----------");
    flow_control_demo!();
    println!("-----------[   pointers_demo    ]-----------");
    pointers_demo!();
}
