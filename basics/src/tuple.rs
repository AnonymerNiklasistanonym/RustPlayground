#[macro_export]
macro_rules! tuple_demo {
    () => {
        // Typed tuples as function argument/return value
        fn reverse(pair: (i32, bool)) -> (bool, i32) {
            let (int_param, bool_param) = pair;
            // Return value
            (bool_param, int_param)
        }

        // Tuples can be tuple members.
        let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

        let tuple = (1, true);
        println!("Tuple is              {:?}", tuple);
        println!("The reversed tuple is {:?}", reverse(tuple));
        println!("Tuple first value is  {:?}", tuple.0);
        println!("Tuple second value is {:?}", tuple.1);

        let long_tuple = (
            1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true, 2,
        );
        //println!("Tuples longer than 12 can't be printed {:?}", long_tuple);
    };
}
