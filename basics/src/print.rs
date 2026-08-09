#[macro_export]
macro_rules! print_demo {
    () => {
        // {} will be replaced with any argument (which is evaluated and stringified in the process)
        println!("5 + 10 = {} days", 5 + 10);

        // Arguments can also be used with an index {index}
        println!("My name is {0}, {1} {0}", "Bond", "James");

        // Arguments can have a name
        println!("{subject} {verb} {object}",
                object="the lazy dog",
                subject="the quick brown fox",
                verb="jumps over");

        // Number formatting (integers)
        println!("Base 10 (default):     {}",   69420); // 69420
        println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
        println!("Base 8 (octal):        {:o}", 69420); // 207454
        println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c

        // Number formatting (floats)
        let pi: f64 = 3.141592653589793;
        println!("Default:                                     {}", pi);
        println!("N=2 decimal places:                          {:.2}", pi);
        println!("Scientific notation:                         {:e}", pi);
        println!("Scientific notation with N=3 decimal places: {:.3e}", pi);

        // Align left (5 characters meaning 1 has 4 leading spaces)
        println!("'12345'");
        println!("'{number:>5}' (right align | 5)", number=1);
        println!("'{number:>3}' (right align | 3)", number=1);
        // if number is longer this will override the align
        println!("'{number:>1}' (right align | 1)", number=10);

        // Select the padding character
        println!("'{number:0>5}' (right align | 5 | padding 0)", number=1); // 00001
        // and left-adjust by flipping the sign. This will output "10000".
        println!("{number:0<5}", number=1); // 10000

        // Use named arguments in the e.g. format specifier by appending a $ (can't be done for the padding!)
        println!("{number:>width$}", number=1, width=5);

        // Only types that implement fmt::Display can be formatted with `{}`
        // Needs to be implemented for custom defined types!
        use std::fmt;

        #[derive(Debug)] // adding this enables debug printing with {:?}
        struct Person {
            name: String,
            age: u32,
        }
        impl fmt::Display for Person {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{name} ({age})", name=self.name, age=self.age)
            }
        }

        let person = Person{name: "Alice".into(), age: 24};
        // local variables can automatically be used in format strings (are automatically captured)
        println!("Person (user type):    {person} [{person:?}]");
        println!("Pretty debug printing: {:#?}", person);
    };
}
