#[macro_export]
macro_rules! enums_demo {
    () => {
        // enum values can either be
        enum WebEvent {
            // 1) unit
            PageLoad,
            PageUnload,
            // 2) tuple
            KeyPress(char),
            Paste(String),
            // 3) struct
            Click { x: i64, y: i64 },
        }

        // The enum value can then be matched
        fn inspect(event: WebEvent) {
            match event {
                // 1) unit
                WebEvent::PageLoad => println!("page loaded"),
                WebEvent::PageUnload => println!("page unloaded"),
                // 2) tuple (destructuring)
                WebEvent::KeyPress(c) => println!("pressed '{c}'."),
                WebEvent::Paste(s) => println!("pasted \"{s}\"."),
                // 3) struct (destructuring)
                WebEvent::Click { x, y } => {
                    println!("clicked at x={x}, y={y}.");
                }
            }
        }

        inspect(WebEvent::KeyPress('x'));
        inspect(WebEvent::Paste("my text".to_owned()));
        inspect(WebEvent::Click { x: 20, y: 80 });
        inspect(WebEvent::PageLoad);
        inspect(WebEvent::PageUnload);

        // Its possible to create an alias for an enum
        #[derive(Debug)]
        enum VeryVerboseEnumOfThingsToDoWithNumbers {
            Add,
            Subtract,
        }
        type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

        let add = Operations::Add;
        println!("enum value: {:?}", add);
        let sub = Operations::Subtract;
        println!("enum value: {:?}", sub);

        enum Stage {
            Beginner,
            Advanced,
        }
        // Make specific enum values available without scope
        use Stage::{Advanced};

        let stage = Stage::Beginner;

        match stage {
            Stage::Beginner => println!("Beginners are starting their learning journey!"),
            Advanced => println!("Advanced learners are mastering their subjects..."),
        }

        enum Role {
            Student,
            Teacher,
        }
        // Make all enum values available without scope
        use Role::*;
        let role = Student;

        match role {
            Student => println!("Students are acquiring knowledge!"),
            Teacher => println!("Teachers are spreading knowledge!"),
        }

        // Like in e.g. C/C++ enums can also be interpreted as numbers (starting from 0)
        enum Number {
            Zero,
            One,
        }
        // or with explicit number values
        enum Color {
            Red = 0xff0000,
            Green = 0x00ff00,
            Blue = 0x0000ff,
        }
        // String values can only be applied by implementing fmt::Display (or using the debug comment)
        impl fmt::Display for Color {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                use Color::*;
                let name = match self {
                    Red => "red",
                    Green => "green",
                    Blue => "blue",
                };
                write!(f, "{}", name)
            }
        }

        println!("Number::Zero: {}", Number::Zero as i32);
        println!("Number::One:  {}", Number::One as i32);

        // {0:0>6x}
        // 0   -> argument 0
        // :   (separator)
        // 0>6 -> lead with zeros and make number at least 6 characters long, align right instead of the default left
        // x   -> instead of decimal rendering (0-9) use hexadecimal rendering (0-f)
        println!("Color::Red:   {}/x{0:0>6x} ({})", Color::Red as u32, Color::Red);
        println!("Color::Green: {}/x{0:0>6x} ({})", Color::Green as u32, Color::Green);
        println!("Color::Blue:  {}/x{0:0>6x} ({})", Color::Blue as u32, Color::Blue);
    };
}
