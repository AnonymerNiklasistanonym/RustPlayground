#[macro_export]
macro_rules! flow_control_demo {
    () => {
        let n = 5;

        // if/else similar to other languages but no paranthesis need to be added arount the statement
        if n < 0 {
            println!("n < 0");
        } else if n > 0 {
            println!("n > 0");
        } else {
            println!("n == 0");
        }

        // the whole block can also represent a return value by using the expression block rules
        let big_n =
            if n < 10 && n > -10 {
                // meaning the last non semicolon statement in the block is the return value
                10 * n
            } else {
                // using such a statement requires all branches to also return a value!
                n / 2
            };
        //   ^ All `let` bindings need this final semicolon!
        println!("big_n: {}", big_n);

        let big_nothing =
            if n < 10 && n > -10 {
                10 * n;
            } else {
                n / 2;
            };
        println!("big_nothing: {:?}", big_nothing);

        // Infinite loop
        let mut count = 0u32;
        loop {
            count += 1;
            if count == 3 {
                println!("three");
                // skip to next loop
                continue;
            }
            if count == 5 {
                println!("stop at 5");
                // exit loop
                break;
            }
        }

        // Different loop scopes can be labled in order to break/continue different loops
        let mut count = 0u32;
        'outer: loop {
            count += 1;
            println!("Entered the outer loop (count={count})");

            'inner: loop {
                count += 10;
                println!("Entered the inner loop (count={count})");

                if count >= 30 {
                    println!(">=30 -> break outer loop");
                    // continue outer loop
                    break 'outer;
                }
                if count >= 20 {
                    println!(">=20 -> continue outer loop");
                    // continue outer loop
                    continue 'outer;
                }

                // continue inner loop
                continue;
            }
        }
        println!("Exited both loops");

        // Loops can also be used to return a value
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                // instead of no semicolon the magic is to add it after the break
                break counter * 2;
            }
        };
        println!("Counted to {} with a loop", counter);

        // Loops can also be done e.g. 100 times
        let mut n = 0;
        while n < 100 {
            n += 1;
        }
        println!("Counted to {} with a while loop", n);

        // Or a specific range can be provided (the last number is excluded)
        for n in 1..5 {
            println!("for n in 1..5: n={}", n);
        }
        // Adding an equal = sign will not exclude the value
        for n in 1..=5 {
            println!("for n in 1..=5: n={}", n);
        }
        // Reverse counting is not possible (meaning the loop will not be run a single time)
        for n in 5..1 {
            println!("for n in 5..1: n={}", n);
        }
        // if not using rev
        for n in (1..=5).rev() {
            println!("for n in (1..=5).rev(): n={}", n);
        }

        // Iterate over a vector (one memory block of data)
        let names = vec!["Bob", "Frank", "Ferris"];
        for name in names.iter() {
            match name {
                &"Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }
        // This only borrows the vector so it can be safley used after:
        println!("names: {:?}", names);

        for name in names.into_iter() {
            match name {
                "Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }
        // using into_iter actually moves the ownership meaning names was consumed and is not available any more!
        //println!("names: {:?}", names);

        // using iter_mut its possible to mutate values
        let mut names = vec!["Bob", "Frank", "Ferris"];
        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!",
                _ => "??",
            }
        }
        println!("names: {:?}", names);

        // using match a value can easily be matched against possible cases (switch/case)
        // (one difference is that only once case will be selected, no fall through possible)
        let number = 13;
        match number {
            // a single value
            1 => println!("One!"),
            // several values
            2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
            // range
            13..=19 => println!("A teen"),
            // default/other cases
            _ => println!("Ain't special"),
        }

        let boolean = true;
        let binary = match boolean {
            // it automatically is an expression with return values:
            false => 0,
            // as soon as its an expression it requires all branches to return something!
            true => 1,
        };
        println!("{} -> {}", boolean, binary);

        // Tuples can even be matched with their specific values
        let triple = (0, -2, 3);
        match triple {
            (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
            // .. means it ignores those parts of the tuple
            (1, ..)  => println!("First is `1` and the rest doesn't matter"),
            (.., 2)  => println!("last is `2` and the rest doesn't matter"),
            (3, .., 4)  => println!("First is `3`, last is `4`, and the rest doesn't matter"),
            _      => println!("It doesn't matter what they are"),
        }

        // For arrays the same is possible
        let array = [1, -2, 6];
        match array {
            [0, second, third] =>
                println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
            // _ will ignore a specific value
            [1, _, third] => println!(
                "array[0] = 1, array[2] = {} and array[1] was ignored",
                third
            ),
            [-1, second, ..] => println!(
                "array[0] = -1, array[1] = {} and all the other ones were ignored",
                second
            ),
            // Using @ .. a specific slice can be created
            [3, second, tail @ ..] => println!(
                "array[0] = 3, array[1] = {} and the other elements were {:?}",
                second, tail
            ),
            // This case matches everything
            [first, middle @ .., last] => println!(
                "array[0] = {}, middle = {:?}, array[2] = {}",
                first, middle, last
            ),
        }
    };
}
