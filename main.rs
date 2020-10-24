// main.rs
// Jordan Campbell
// 24.10.2020
// https://learning-rust.github.io/docs/a3.hello_world.html
// ❯ rustc main.rs
// ❯ ./main


fn log_demo() {
    println!("Hello, world!");          // Hello, world!
    println!("{} {}", "a", "b");        // a b
    println!("{0} {1}", "a", "b");      // a b
    println!("{1} {0}", "a", "b");      // b a
    println!("{greeting}, {name}!", greeting="Hello", name="world"); // Hello, world!
    println!("{:?}", [1,2,3]); // [1, 2, 3]
    println!("{:#?}", [1,2,3]);
    /*
        [
            1,
            2,
            3
        ]
    */
    let x = format!("{}, {}!", "Hello", "world");
    println!("{}", x); // Hello, world!

    // 💡 Rust has a print!() macro as well
    print!("Hello, world!"); // Without new line
    println!(); // A new line

    print!("Hello, world!\n"); // With new line
}

// println! is a macro -- "generates code" rather than being a strict function
fn main() {
    log_demo();
}