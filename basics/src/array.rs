#[macro_export]
macro_rules! array_demo {
    () => {
        fn array_information(arr: &[i32]) {
            if (arr.len() > 0) {
                println!("> First element of the array: {}", arr[0]);
            }
            println!("> The array has {} elements ({arr:?})", arr.len());
        }

        // Fixed-size array
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        // Initalize all elements with a specific value
        let arr2: [i32; 500] = [2; 500];

        println!("arr:  {:?}", arr);
        println!("arr2: {:?}", arr2);

        println!("arr[0]:    {}", arr[0]);
        println!("arr[1]:    {}", arr[1]);
        println!("arr[2]:    {}", arr[2]);
        println!("arr.len(): {}", arr.len());

        use std::mem;

        // Arrays are stack allocated
        println!("arr occupies  {} bytes", mem::size_of_val(&arr));
        println!("arr2 occupies {} bytes", mem::size_of_val(&arr2));

        // Slice the array
        println!("arr[1 .. 4]: {:?}", &arr[1 .. 4]);
        array_information(&arr[1 .. 4]);

        // Using get arrays can be safley accessed since that returns an Option(Some or None)
        for i in 0..arr.len() + 1 {
            match arr.get(i) {
                Some(value) => println!("{}: {}", i, value),
                None => println!("{} doesn't exist!", i),
            }
        }
    };
}
