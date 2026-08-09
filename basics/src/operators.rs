#[macro_export]
macro_rules! operators_demo {
    () => {
        println!("1 + 2 = {}", 1u32 + 2);
        println!("1 - 2 = {}", 1i32 - 2);

        // Short-circuiting boolean logic
        println!("true AND false is {}", true && false);
        println!("true OR false is {}", true || false);
        println!("NOT true is {}", !true);

        // Bitwise operations
        println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
        println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
        println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
        println!("10 << 5 is {} ({:0>10b} -> {0:0>10b})", 10u32 << 5, 10);
        println!(
            "0x80 >> 2 is 0x{:x} ({:0>10b} -> {0:0>10b})",
            0x80u32 >> 2,
            0x80u32
        );
    };
}
