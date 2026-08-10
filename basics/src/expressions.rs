#[macro_export]
macro_rules! expressions_demo {
    () => {
        // Program is a list of statements e.g.
        // statement
        // statement
        // statement

        // Statements can be
        // 1) variable declarations
        let x;
        // 2) variable bindings
        x = 5;
        // 3) expressions
        x;
        x + 1;
        15;

        // Blocks are expressions too
        let y = {
            // in a block there are statements (ending with a semicolon [;])
            let x_squared = x * x;
            let x_cube = x_squared * x;
            // if the final statement does not end with a semicolon [;] its the result of the expression
            x_cube + x_squared + x
        };

        let z = {
            // if no final statement with a semicolon [;] exists the result is '()'
            2 * x;
        };

        println!("x is {:?}", x);
        println!("y is {:?}", y);
        println!("z is {:?}", z);
    };
}
