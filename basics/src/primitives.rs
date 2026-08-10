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

        // adding mut (mutable) will allow to change the value of a variable
        let mut mutable = 12;
        mutable = 21;
        println!("mutable: {} (inital)", mutable);

        // Variables can be overwritten with shadowing but not changed!
        let mutable = true;
        println!("mutable: {} (shadowed in same scope)", mutable);

        {
            // Shadows variable name while in scope
            let mutable = 67;
            println!("mutable: {}  (shadowed in new scope)", mutable);
        }
        println!("mutable: {} (after previous scope is closed)", mutable);

        // Array signatures [T; length] have a Type T and a length
        let my_array: [i32; 5] = [1, 2, 3, 4, 5];

        // Tuples are supported by default
        let my_tuple = (5u32, 1u8, true, -5.04f32);

        // Use underscores to improve readability
        let million = 1_000_000u32;
        println!("One million: {}", million);

        // Use scientific notation to improve readability (floating point numbers)
        println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

        // a variable can be declared first BUT MUST be bound before being used!
        let binding;
        // This is not possible! (no default value will be assigned!)
        //println!("binding: {}", binding);
        binding = 1;
        println!("binding: {}", binding);

        // Casting
        // 1) Implicit casting only possible if its safe!
        let decimal = 65.4321_f32;
        // Error! No conversion from floating (f32) to integer (u8)
        //let integer: u8 = decimal;
        let small_int = 8u8;
        let big_integer: u16 = small_int.into();
        let bigger_integer: u32 = small_int.into();
        let even_bigger_integer: u64 = small_int.into();
        let even_bigger_integer_with_sign: i64 = small_int.into();
        println!("Explicit casting: {} -> {} -> {}", small_int, big_integer, bigger_integer);

        // 2) Explicit casting
        let decimal = 65.4321_f32;
        let integer = decimal as u8;
        let character = integer as char;
        println!("Explicit casting: {} -> {} -> {}", decimal, integer, character);
        // Error! A float cannot be safley converted to a char
        //let character = decimal as char;

        // Explicit casting can have weird effects if you aren't aware of binary representations
        println!("{0} ({0:b}) in i8 is: {1} ({1:b}) as u8", -1i8, (-1i8) as u8);

        // For everything that is not trivially castable you should double check the edge cases:
        println!("{} (f32) as u8 is : {}", 300.0_f32, 300.0_f32 as u8);
        println!("{} (f32) as u8 is : {}", -100.0_f32, -100.0_f32 as u8);
        println!("{} (f32) as u8 is : {}", f32::NAN, f32::NAN as u8);

        // Depending on the type the memory usage is different and can be checked via std::mem::size_of_val
        let x1 = 1u8;
        let x2 = 1u32;
        let x3 = 1f32;
        let x4 = 1f64;
        let x5 = 1u128;
        println!("size of x1 in bytes: {}", std::mem::size_of_val(&x1));
        println!("size of x2 in bytes: {}", std::mem::size_of_val(&x2));
        println!("size of x3 in bytes: {}", std::mem::size_of_val(&x3));
        println!("size of x4 in bytes: {}", std::mem::size_of_val(&x4));
        println!("size of x5 in bytes: {}", std::mem::size_of_val(&x5));

        // Type aliases
        // (Don't provide additional saftey, just a literal alias)
        type NanoSecond = u64;
        type Inch = u64;
        let nanoseconds: NanoSecond = 5;
        let inches: Inch = 2;
        println!("{} nanoseconds + {} inches = {}?", nanoseconds, inches, nanoseconds + inches);
    };
}
