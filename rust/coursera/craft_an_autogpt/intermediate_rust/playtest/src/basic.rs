pub fn basics() {
    println!("Hello World!");
    let x: i32 = 50;
    let y: f32 = 9.0;
    println!("x is {} and y is {}", x, y);

    let s1: String = String::from("Hello World!");
    let s2: String = "Yello Dello!".to_string();
    let s3: String = s1; // Ownership of s1 transferred to s3
    let s4: &String = &s2; // Borrowed a Reference to value at memory pointed by pointer s4

    /* 
     * 
     * Data Types in Rust
     *      Scalar Datatypes:
     *          Integer Types: 
     *              u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize
     *              Decimal     : 98_222_000
     *              Hexadecimal : 0xffff
     *              Octal       : 0o7777
     *              Binary      : 0b1100_1100
     *              Byte        : b'A' (is u8 only)
     *          Floating Types:
     *              f32, f64
     *              Numeric Operations: +, -, /, *, %
     *          Boolean Type:
     *              true or false
     *          Character Type:
     *              4 byte unicode scalar value
     * 
     *      Compound Datatypes:
     *          Tuple Type:
     *              grouping different number of types
     *          Array Type:
     *              
     * 
     * 
     * 
    */

    // Tuple Type
    let tup: (i32, String, &str) = (25, s2, &s2);
    let (x, y, z) = &tup;
    let (a, b, c) = tup; // Ownership is shifted here

    // Array Types
    let arr: [i32; 10] = [0; 10]; // Create an array arr & initialize all to 0

    // println!("s1 is: {}", s1); // as s1 no longer is the owner of "Hello World!" 
    println!("s2 is: {}", s2);
    println!("s3 is: {}", s3);
    println!("s4 is: {}", s4);

    println!("Status is: {}", Status::Started.to_string());

    decorate(s3);

}

fn decorate(s: String) -> &String {
    &s
} // here s loses its lifetime, we need a lifetime specifier to ensure value survives

enum Status {
    Started,
    Stopped,
    Backfill
}

impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Started => "Started".to_string(),
            Status::Stopped => "Stopped".to_string(),
            Status::Backfill => "Backfill".to_string(),
        }
    }
}