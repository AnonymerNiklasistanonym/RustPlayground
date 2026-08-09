#[macro_export]
macro_rules! structs_demo {
    () => {
        // Unit struct
        #[allow(dead_code)]
        struct Marker;
        // Useful when the type itself carries meaning like
        #[allow(dead_code)]
        struct Admin;
        #[allow(dead_code)]
        struct User;

        // Tuple struct
        #[derive(Debug)]
        struct Point(i32, i32);

        // Named-field struct
        #[derive(Debug)]
        struct Person2 {
            name: String,
            age: u8,
        }

        // Create struct:
        // 1)
        let name = String::from("Peter");
        let age = 27;
        let peter = Person2 { name, age };
        println!("{:?}", peter);
        // 2)
        let peter2 = Person2 {
            name: String::from("Peter"),
            age: 27,
        };
        println!("{:?}", peter2);

        // Structs can be reused as fields of another struct
        #[derive(Debug)]
        struct Rectangle {
            top_left: Point,
            bottom_right: Point,
        }

        let rectangle = Rectangle {
            // round paranthesis because its a tuple struct
            top_left: Point(5, 0),
            bottom_right: Point(10, 0),
        };
        println!("Rectangle {:?} [{:?}, {:?}]", rectangle, rectangle.top_left, rectangle.bottom_right);
    };
}
