#[macro_export]
macro_rules! primitives_demo {
    () => {
        // Variables can be type annotated
        let logical: bool = true;
        // The type of a variable can't be changed!
        //logical = 1;

        let a_float: f64 = 1.0; // Regular annotation
        let an_integer = 5i32; // Suffix annotation

        // If no annoation is used it will default to a type
        let default_float = 3.0; // `f64`
        let default_integer = 7; // `i32`

        // the type default can even be inferred from another line
        let mut inferred_type = 12; // `i32` but overridden by the next line
        inferred_type = 4294967296i64; // `i64`
        // since the variable was declared mutable the value can be changed

        let mut mutable = 12;
        mutable = 21;
        // Variables can be overwritten with shadowing but not changed!
        let mutable = true;

        // Array signatures [T; length] have a Type T and a length
        let my_array: [i32; 5] = [1, 2, 3, 4, 5];

        // Tuples are supported by default
        let my_tuple = (5u32, 1u8, true, -5.04f32);

        // Use underscores to improve readability
        let million = 1_000_000u32;
        println!("One million: {}", million);

        // Use scientific notation to improve readability (floating point numbers)
        println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);
    };
}
