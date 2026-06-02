#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

mod utils;
use crate::utils::fives::{add_five, sub_five};
use crate::utils::nines::{add_nine, sub_nine};

fn main() {
    let x = 25;
    let y = add_five(x);
    let z = sub_five(x);

    println!("{}  plus five is: {}", x, y);
    println!("{} minus five is: {}", x, z);

    let a = 25;
    let b = add_nine(a);
    let c = sub_nine(a);

    println!("{}  plus five is: {}", a, b);
    println!("{} minus five is: {}", a, c);

    main1();
    mut_immut_references();
    memory();
}

fn change_string(text: &mut String) {
    text.push('!');
}

fn main1() {
    let mut s: String = String::from("Hello World");
    let t: &mut String = &mut s;
    println!("t is {}", t);
    change_string(t);
    println!("t now is {}", t);
}

const MY_INTEGER: i32 = 25; // Static Memory

fn memory() {
    let x: i32 = 25 + MY_INTEGER; // stored in Stack

    let s1: String = String::from("Hello Dello"); // stored in heap
    let s2: String = s1.clone(); // value same as above, but a deep clone of s1 and stored in heap
    let s3: &str = "Hello Yellow"; // String Literal stored in Static
    let mut s4: String = "Yellow Dello".to_string();

    let t: &String = &s1; // reference stored in Stack of s1 in Heap
    let u: &mut String = &mut s4;
}

fn mut_immut_references() {
    let mut x: String = String::from("Hello World");

    let y: &String = &x; // ref cannot change, data cannot change
    let y: &mut String = &mut x; // ref cannot change, data can change
    let mut y: &String = &x; // ref can change, data cannot change
    let mut y: &mut String = &mut x; // ref can change, data can change
}

// fn will_dangle(s: String) -> &'static String {
//     &s
// }
