mod m1_enums;

use std::fmt::Formatter;


const _OUR_COURSE: &str = "Rust with AutoGPT"; // stored in Static Memory

fn _basics() {
    println!("Welcome to our course in {}!", _OUR_COURSE);

    // Stack 
    let x: i32;
    x = 2;
    println!("x is {}", x);

    let y: i32 = 4;
    println!("y is {}", y);

    // For loops with y - 0, 1, 2, 3
    for i in 0..y {
        print!("{}, ", i);
    }

    println!("");

    // For loops with equals y - 0, 1, 2, 3, 4
    for i in 0..=y {
        print!("{}, ", i);
    }

    println!("");

    // mutability
    let mut z: i32 = 5;
    println!("z is {}.", z);
    z = 10;
    println!("z is {} now.", z);

    let freeze: f64 = -2.4;
    println!("freeze is {}.", freeze);

    let is_zero_remainder: bool = 10 % 4 != 0;
    println!("is_zero_remainder is {}.", is_zero_remainder);

    let my_char: char = 'z';
    println!("my_char is {}.", my_char);

    let my_emoji: char = '😎';
    println!("my_emoji is {}.", my_emoji);

    let my_ints: [i32; 10] = [1; 10]; // 10 i32 all initialized to 1
    println!("my_ints is {:?}.", my_ints);

    let my_floats: [f32; 10] = [0.0; 10]; // 10 f32 all initialized to 0.0
    println!("my_floats is {:?}.", my_floats);

    let my_floats_new: [f32; 10] = my_floats;
    println!("after assign my_floats is {:?}.", my_floats);
    println!("my_floats_new is {:?}.", my_floats_new);

    let my_floats_new1: [f32; 10] = my_floats.map(|n: f32| n + 2.0);
    println!("my_floats_new1 is {:?}", my_floats_new1);
}

fn _intermediate() {
    let name: &str = "Hello Dello";
    println!("name is {:?}", name); // dereference coersion
    println!("ptr to name is {:p}", name); // pointer of reference on Static Memory

    let dyn_name_1: String = "Hello Dello1".to_string();
    let dyn_name_2: String = String::from("Hello Dello2");
    println!("dyn_name_1 is {:?}", dyn_name_1);
    println!("dyn_name_2 is {:?}", dyn_name_2);
    // println!("pointer to dyn_name_1 is {:p}", dyn_name_1); // panics! - trait `std::fmt::Pointer` is not implemented for `String`
    // println!("pointer to dyn_name_2 is {:p}", dyn_name_2); // panics! - trait `std::fmt::Pointer` is not implemented for `String`
    println!("pointer to dyn_name_1 is {:p}", &dyn_name_1);
    println!("pointer to dyn_name_2 is {:p}", &dyn_name_2);

    println!("ptr to name is {:p}", name); // pointer of reference on Static Memory

    let str_slice: &str = &dyn_name_1[0..5];
    println!("str_slice is {:?}", str_slice);

    // Vectors
    let mut chars: Vec<char> = Vec::new();
    chars.insert(0, 'h');
    chars.insert(1, 'e');
    chars.insert(2, 'l');
    chars.push('l');
    chars.push('o');
    chars.push('!');
    println!("chars is {:?}", chars);
    // dbg!(chars); // here the value gets moved!
    // println!(chars); // so here it errors! instead send &chars

   dbg!(&chars);
   println!("chars is {:?}", chars);

   let removed_chars: Option<char> = chars.pop();
   println!("removed_chars is {:?}",removed_chars);

   let removed_chars: char = chars.pop().unwrap();
   println!("removed_chars is {:?}",removed_chars);

   println!("chars is {:?}", chars);

   // Iterator
   chars.iter().for_each(|c| println!("c is {}", c));

}

fn _basic_collections() {
    // Vectors
    let mut chars: Vec<char> = Vec::new();
    chars.insert(0, 'h');
    chars.insert(1, 'e');
    chars.insert(2, 'l');
    chars.push('l');
    chars.push('o');
    chars.push('!');
    println!("chars is {:?}", chars);

    let cc:char = chars.pop().unwrap();
    println!("cc is {:?}", cc);

    println!("chars is {:?}", chars);

    // Iterator & Closure
    chars.iter().for_each(|c| println!("c is {}", c));

    // Another vector
    let chars_again: Vec<char> = vec!('h','e','l','l','o','!');
    println!("chars_again is {:?}", chars_again);

    let collected: String = chars_again.iter().collect();
    println!("collected is {}", collected);

    for c in chars_again {
        print!("{}", c);
        if c == 'o' {
            print!(", world");
        }
    }

}

fn _closures() {
    let num: i32 = 25;
    let add_num = |x: i32| x + num;
    println!("add_num of 25 is {}", add_num(25));
}

fn _literals() {
    println!("big_number is {}",9_800_900);
    println!("hex number is {}",0xff);
    println!("octal numb is {}", 0o77);
    println!("binary num is {}", 0b1111_0000);
    println!("bytes 'A'  is {}", b'A');

    // Raw String Literal
    let text: &str = r#"message"#; // Raw String
    dbg!(text);
}

fn bits_n_bytes() {
    let a: u8 = 0b_1010_1010;
    let b: u8 = 0b_1111_1010;
    println!("a's val is {}", a);
    println!("b's val is {}", b);

    println!("a in binary {:08b}", a);
    println!("b in binary {:16b}", b);

    // Logic Gates
    println!("a AND b  {:08b}", a & b);
    println!("a OR b  {:08b}", a | b);
    println!("a XOR b  {:08b}", a ^ b);
    println!("NOT b  {:08b}", !b);

    // Bitwise Operations
    println!("a << 1: {:08b}", a << 1); // display in binary
    println!("a << 1: {}", a << 1);
    println!("a >> 1: {:08b}", a >> 1); // display in binary
    println!("a >> 1: {}", a >> 1);

    // Endian
    let n: u16 = 0x1234;
    println!("n is: {:?}", n);
    println!("big endian: {}", n.to_be());
    println!("lil endian: {}", n.to_le());
    let big_endian: [u8; _] = n.to_be_bytes();
    let sml_endian: [u8; _] = n.to_le_bytes();
    println!("big endian: {:02x}{:02x}", big_endian[0], big_endian[1]);
    println!("lil endian: {:02x}{:02x}", sml_endian[0], sml_endian[1]);
}

enum Status {
    Active,
    Inactive,
    Pending
}

// impl ToString for Status {
//     fn to_string(&self) -> String {
//         match self {
//             Status::Active => "Active".to_string(),
//             Status::Inactive => "Inactive".to_string(),
//             Status::Pending => "Pending".to_string()
//         }
//     }
// }

use core::fmt::{Display, Error};

impl Display for Status {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Status::Active => write!(f, "Active"),
            Status::Inactive => write!(f, "Inactive"),
            Status::Pending => write!(f, "Pending")
        }
    }
}

fn main() {
    // basics();
    // intermediate();
    //basic_collections();
    // closures();
    // literals();
    bits_n_bytes();
}
