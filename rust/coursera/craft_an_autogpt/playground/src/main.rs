mod my_funcs;
use crate::my_funcs::{add_five, sub_five};

mod utils;
use crate::utils::sub_funcs::{add_nine, sub_nine};

/**
 * This is documentation for rust progs!
 * 
 * This is a test!
 */

// Understanding rust Memory types & when, what is stored in them
// Stack vs. Heap vs. Static

// stored in Static
const _MY_INTEGER: u8 = 10; 

fn _main2() {
    let x:u8 = 50; // stored in stack, due to fixed size
    println!("x is {}", x);
    
    // Heap
    let arr:Vec<u8> = vec![1,2,3,4,5];
    println!("vec1 is {:?}", arr);
    
    // Heap
    let mut arr1:Vec<u8> = vec![2,3,4,5,6];
    arr1.push(10);
    println!("vec2 is {:?}", arr1);
    
    // Stack - reference is created on the stack pointing to value on Heap
    let arr2 = &arr1[0..3];
    println!("vec3 is {:?}", arr2);

    // Heap
    let s1:String = String::from("hello");
    println!("s1 is {:?}", s1);
    
    // Heap
    let mut s2:String = String::from("hello");
    s2.push(' ');
    s2.push('!');
    println!("s2 is {:?}", s2);
    
    // Stack - reference is created on the stack pointing to value on Heap
    let s3 = &s1[0..5];
    println!("s3 is {:?}", s3);
    
    println!("MY_INTEGER is {:?}", _MY_INTEGER);
}

fn _main3() {
    let num1: u8 = 50;
    let num2: u8 = num1;
    println!("num1 is: {}", num1);
    println!("num2 is: {}", num2); // no issues here

    let s1: String = String::from("Hello World!");
    let s2: String = s1;   // s2 is now Owner of the data in s1
    // println!("{}", s1); // s1 no longer has the ownership, so panics
    println!("{}", s2);

    // What do we do to remedy the Ownership?
    let s1: String = String::from("Hello World!");
    let s2: String = s1.clone();   // s2 has a deep copy of s1 & s1 continues to have its data
    println!("{}", s1); // s1 no longer panics
    println!("{}", s2);
}

// Dangling References

fn make_dangling2() -> String {
    let s: String = String::from("Hello Hello!");
    s
} // here s has lost ownership & hence r is not pointing to anything

pub fn _main4() {
    let x: u8 = 50;
    let y: u8 = x;
    println!("x is {}", x);
    println!("y is {}", y);
    
    // let s1: String = String::from("Hello World");
    // let s2: String = s1;
    // println!("s1 is {}", s1); // panics!
    // println!("s2 is {}", s2);
    
    let s1: String = String::from("Hello World");
    let s2: String = s1.clone(); // equivalent to = String::from("Hello World");
    println!("s1 is {}", s1); // no more panics!
    println!("s2 is {}", s2);
    
    let s3: String = String::from("Hello World");
    let s4: &String = &s3;      // reference to s3, borrowing the value
    println!("s3 is {}", s3);   // no more panics!
    println!("s4 is {}", s4);

    // Wont compile, as s has lost the value & hence ref to that varible has 
    // nothing for it to point to, hence panics
    // make_dangling();
    let s5: String = make_dangling2();
    println!("s5 is {}", s5);
}

// pub fn main() {
//   let mut xs = vec![1, 2, 3];

//   let xs_mut = &mut xs;
//   let x_ref = &xs[0];

//   xs_mut.clear(); 

//   dbg!(x_ref); // dangling pointer
// }

// fn _make_dangling() -> &String {
//     let s: String = String::from("Hello Hello!");
//     let r: &String = &s;
//     r
// } // here s has lost ownership & hence r is not pointing to anything


// pub fn main() {
//     // let mut s: String = String::from("Hello Hello");
//     // let t: &String = &s; // this is an immutable borrow - panics!
//     // s.push(' ');
//     // s.push('?');
//     // println!("s is {}", s);
//     // //println!("t is {}", t);
    
//     // let mut s1: String = String::from("Hello Dello");
//     // let t1: &mut String = &mut s1;    // this is mutable borrow
//     // s1.push('?');        
//     // println!("s1 is {}", s1); // immutable borrow happens here
//     // println!("t1 is {}", t1); // mutable borrow used here, but later
    
//     // let mut s1: String = String::from("Hello Dello");
//     // let t1 = &mut s1; // this is mutable borrow
//     // t1.push('?');
//     // println!("t1 is {}", t1); // mutable borrow used here - works!
//     // println!("s1 is {}", s1); // immutable borrow occurs here
//     // println!("t1 is {}", t1); // this panics!
    
//     let mut s1: String = String::from("Hellow Yellow");
//     let mut s2: &mut String = &mut s2; // think of it like a backdoor
// }

const MY_INTEGER: i32 = 50; // Static memory

fn memory() {
    //Stack - all fixed length goes here - for e.g., datatypes
    let _x: u8 = 25;
    let _y: f32 = 2.5; // f16 is unstable
    
    // Heap - all arrays, vectors, Strings, etc.
    let mut s: String = String::from("Hello!");
    let _t: &String = &s; // Stack - all references are put in stack
    let _u: &mut String = &mut s; // Stack - reference in Stack
    
    // "Hello2 World" is a String Literal stored in Static
    let s3: &str = "Hello2 World"; // Stored in Stack as Reference to a Static
    println!("s3 is {:?}", s3);
    
    let s4: &str = &s3[0..6]; // Stored in Stack as a Reference to a Heap
    println!("s4 is {:?}", s4);
    
    let s5: String = "Hello3 World".to_string(); // Stored in a Heap as a String
    println!("s5 is {:?}", s5);
}

fn mut_immut() {
    let mut x = 10;         // should be mut, else it cannot be borrowed
    let _y = x;              // ownership moved to y
    let _y = &x;             // cannot change data (10) and where y points to (the reference)
    let _y = &mut x;         // can change data (10) but not where y points to (the reference)
    let _y = &x;         // cannot change data (10) but can change where y points to (the reference)
    let _y = &mut x;     // can change data (10) and change where y points to (the reference)
}

fn ownership() {
    // Ownership
    let s: String = String::from("Hello Hello");
    let t: String = s; // value of s moved to t; t now has the ownership
    // println!("s is: {}", s); // panics! - attempting value borrow after move
    
    { // this is similar to sending the data to a different mod/fn
        let _s1: String = String::from("Yellow Dellow");
        let t1: String = t; // ownership handed over to t1
        println!("t1 is {}", t1); // works!
        
    } // here t1 will be cleaned up - s -> t -> t1 -> cleaned up    
    // println!("t is {}", t); // panics! - attempting value borrow after move
    
} // typically all variables within will lose its ownership & cleaned up

// fn will_dangle(s: String) -> &String {
//     &s
// } // here s will have been deleted so reference &s will not be poiniting to anything

fn send_value_instead(s: String) -> String {
    s
}

fn decorator_send_string(s: &mut String) {
    s.push('!');
}

// fn test_dangle() {
//     let x: String = String::from("Hello");
//     will_dangle(x);
// }

fn test_dangle2() {
    let x: String = String::from("Hello");
    send_value_instead(x);
}

fn test_mut_no_dangle2() {
    let mut x: String = String::from("Hello");
    // let &mut y: &mut String = &mut x;
    decorator_send_string(&mut x);
}

fn deref_operator() {
    let s: String = String::from("Hello World");
    let t: &String = &s;
    println!("t is {}", t);
    println!("t ptr is {:p}", t); // deref is using * => under the hood, it *t
}

fn deref_use() {
    let mut s: String = "John".to_string();
    let t: &mut String   = &mut s;

    *t = String::from("Daunt");
    dbg!(&t);
    println!("t is {}", t);
    println!("s is {}", s);
}

fn deref_other() {
    let mut x: i32 = 50;
    x = 70;
    dbg!(x);
}

// New Scope + New Stack Frame (when called)
fn scope() {
    let a: i32 = 50;
    dbg!(a);

    {
        let a: i32 = 88;
        let b: i32 = 78;
        dbg!(a);
        dbg!(b);
    }

    dbg!(a);

    // Scope in for
    for _i in 0..10 {
        let _c: i32 = 34;
    }

    // Scope in if .. else if .. else
    if a < 50 {
        let _a: i32 = 35;
    } else if a >=50 {
        let _a: i32 = 36;
    } else {
        let _a: i32 = 37;
    }

    // Scope in match
    match a {
        30 => {
            let _a: i32 = 38;
        },
        _ => {
            let _a: i32 = 39;
        },
    }
    // Scope in Closures
    let closure_example = || {
        let _a: i32 = 40;
    };

    closure_example(); // new stack frame when closure is invoked

}

fn main() {
    println!("Hello, world!");

    let num = 34;
    println!("{} plus five is: {}", num, add_five(num));
    println!("{}  sub five is: {}", num, sub_five(num));

    let num = 45;
    println!("{} plus nine is: {}", num, add_nine(num));
    println!("{}  sub nine is: {}", num, sub_nine(num));

    memory();
    mut_immut();
    ownership();
    // test_dangle();
    test_dangle2();
    test_mut_no_dangle2();
    deref_operator();
    deref_use();
    deref_other();
    scope();
}