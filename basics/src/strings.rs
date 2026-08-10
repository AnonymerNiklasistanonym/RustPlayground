#[macro_export]
macro_rules! strings_demo {
    () => {
        fn get_type_of<T>(_: &T) -> &str {
            std::any::type_name::<T>()
        }

        // str > dynamically sized string slice (needs to be borrowed)
        // -> & means the string is being borrowed
        let language: &str = "English";
        // the default type of a string literal is also a &str
        let hello = "hello";

        println!("str:    {:?} ({}), {:?} ({})", language, get_type_of(&language), hello, get_type_of(&hello));

        // String > owned, growable string
        let mut my_string = String::from(language);
        println!("String: {:?} ({})", my_string, get_type_of(&my_string));
        my_string.push_str(" hello");
        println!("String: {:?} ({})", my_string, get_type_of(&my_string));

        // For function arguments &str is a good choice if the goal is to just read the value which will just borrow the string data
        fn print_string(x: &str) {
            println!("print_string: {}", x);
        }
        print_string(&my_string);

        // From trait:
        // Define how to create type **from** another type -> mechanism to convert between several types

        let string_from_str = String::from("abc");
        println!("String::from(\"abc\"):   {:?}", &string_from_str);

        use std::convert::From;
        #[derive(Debug)]
        struct CustomNumber {
            value: i32,
        }
        impl From<i32> for CustomNumber {
            fn from(item: i32) -> Self {
                CustomNumber { value: item }
            }
        }
        let num = CustomNumber::from(30);
        println!("CustomNumber::from(30): {:?}", num);

        // Into trait:
        // Define how to convert type **into** another type

        // Important: Defining from automatically defines into:
        let num: CustomNumber = 5.into();
        println!("CustomNumber = 5.into(): {:?}", num);

        // but this will not automatically define from!
        use std::convert::Into;
        #[derive(Debug)]
        struct CustomNumber2 {
            value: i32,
        }
        impl Into<CustomNumber2> for i32 {
            fn into(self) -> CustomNumber2 {
                CustomNumber2 { value: self }
            }
        }
        let int = 5;
        let num: CustomNumber2 = int.into();
        println!("CustomNumber2 = int.into(): {:?}", num);
        // ERROR:
        //let num = CustomNumber2::from(20);
        //println!("CustomNumber2::from(20): {:?}", num);

        // If the conversion is not always successful there is also TryFrom and TryInto:
        use std::convert::TryFrom;
        #[derive(Debug)]
        #[derive(PartialEq)]
        struct EvenNumber(i32);
        impl TryFrom<i32> for EvenNumber {
            type Error = ();
            fn try_from(value: i32) -> Result<Self, Self::Error> {
                if value % 2 == 0 {
                    Ok(EvenNumber(value))
                } else {
                    Err(())
                }
            }
        }
        assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
        assert_eq!(EvenNumber::try_from(5), Err(()));

        let result: Result<EvenNumber, ()> = 8i32.try_into();
        assert_eq!(result, Ok(EvenNumber(8)));
        let result: Result<EvenNumber, ()> = 5i32.try_into();
        assert_eq!(result, Err(()));
    };
}
