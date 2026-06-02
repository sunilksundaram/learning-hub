mod m1_enum;
mod m2_structs;
mod m3_traits;
mod m4_polymorphism;
mod p4_trait;
mod p4_traits;

#[allow(dead_code)]
const PI: f64 = 3.14; // constant, cannot be changed

#[allow(dead_code)]
const HELLO_WORLD: &str = "Hello, World!"; // String Lietral & COnstants are stored in Static Memory

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
fn memory() {
    let a: i32 = 50; // stored in stack
    let b: String = String::from("Hello"); // stored in heap (String is a struct that contains a pointer to the heap, the length, and the capacity)
    let c: String = "World".to_string(); // stored in a heap
    let d: &String = &b; // stored in stack (reference to the heap)
    let e: &str = "Hello"; // stored in stack (string literal)

    let f: &str = &b[0..5]; // stored in stack (reference to the heap)
    println!("f is {:?}", f);
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
fn mut_immut_ref() {
    let s: String = String::from("Hello World");
    let mut t: String = s.clone(); // clone creates a deep copy of the string, stored in heap

    let s1: &String = &s; //immutable reference to s, immutable data
    let s2: &mut String = &mut t; //immutable reference to t, mutable data
    let mut s3: &String = &s; //mutable reference to s, immutable data
    let mut s4: &mut String = &mut t; //mutable reference to t, mutable data
}

#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_mut)]
fn ownership() {
    let s: String = String::from("Hello World"); // s is the owner of the string
    let t: String = s; // ownership of the string is moved to t, s is no longer valid
    // println!("s is {}", s); // this will cause a compile error because s is no longer valid
    println!("t is {}", t); // this will work because t is the owner of the string

    {
        let s1: String = t; // ownership of the string is moved to s1, t is no longer valid
        // println!("t is {}", t); // this will cause a compile error because t is no longer valid
        println!("s1 is {}", s1); // this will work because s
    }

    // println!("s1 is {}", s1); // this will cause a compile error because s1 is no longer valid
    // println!("t is {}", t); // this will cause a compile error because t is no longer valid
}

// sending value to a function - ownership is moved to the function, and the value is dropped when the function ends
#[allow(dead_code)]
fn send_value(s: String) {
    println!("s is {}", s);
} // s is dropped here

#[allow(dead_code)]

// dangling reference - the reference is returned from the function, but the value it points to is dropped when the function ends
// fn dangling_reference(s: String) -> &String {
//     &s // this will cause a compile error because s is dropped when the function ends
// }

// Types for return Functions
// fn abg(s: String) -> &String {
//     &s // return immutable reference to s, dangling reference - s is dropped when the function ends
// }

// fn abh(mut s: String) -> &String {
//     s.push_str(" World"); // modify the string
//     &s // return immutable reference to s, dangling reference - s is dropped when the function ends
// }

// fn abi(s: String) -> &mut String {
//     &mut s // cannot mutate immutable variable
// }

// fn abj(mut s: String) -> &mut String {
//     s.push_str(" World"); // modify the string
//     &mut s // return mutable reference to s, dangling reference - s is dropped when the function ends
// }

// remedy for dangling reference - return the value instead of a reference, so that the ownership is moved to the caller
fn remedy_dangling_ref(mut s: String) -> String {
    s.push_str(" World"); // modify the string
    s // return the string, ownership is moved to the caller
}

// dereference operator - allows us to access the value that a reference points to
#[allow(dead_code)]
fn deref_operator() {
    let s: String = String::from("Hello World");
    let t: &String = &s; // t is a reference to s
    println!("t is {}", t); // its printing *t under the hood
    println!("prt to t is {:p}", t); // this will print the memory address of t
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn trials() {
    let x = {
        let y = 10;
        y + 20
    };
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn rust_types() {
    let a: i32 = 10; // integer - i8, u8, i16, u16, i32, u32, i64, u64, i128, u128
    let b: f64 = 3.14; // floating point - f32, f64
    let c: bool = true; // boolean - true or false
    let d: char = 'A'; // character - single Unicode scalar value
    let e: &str = "Hello"; // string slice - reference to a string literal

    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // array of integers
    let arr1: [i32; 5] = [0; 5]; // array of integers initialized to 0
    let tup: (i32, f64, bool) = (10, 3.14, true); // tuple of different types
    let (x, y, z) = tup; // destructuring the tuple

    println!("x is {}, y is {}, z is {}, tup is {:?}", x, y, z, tup);
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn shadowing() {
    let x = 10; // x is an integer
    let x = "Hello"; // x is now a string, shadows the previous x
    let x = 20; // x is now an integer again, shadows the previous x
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn loops() {
    for i in 0..5 {
        println!("i is {}", i);
    }

    let mut j = 0;
    while j < 4 {
        println!("j is {}", j);
        j += 1;
    }

    let myvars: Vec<String> = vec!["Hello".to_string(), "World".to_string()];
    for myvar in myvars.iter() {
        println!("myvar is {}", myvar);
    }
}

#[allow(dead_code)]
fn basic_collection() {
    let mut chars: Vec<char> = Vec::new(); // create an empty vector of characters
    chars.push('H'); // add characters to the vector
    chars.push('e');
    chars.push('l');
    chars.push('l');
    chars.push('o');
    println!("chars is {:?}", chars); // print the vector

    let c: char = chars.pop().unwrap(); // remove the last character from the vector
    dbg!("c is {}", c); // print the removed character, also note the c cannot be used post this line because it has been moved out of the vector

    chars.iter().for_each(|c| println!("c is {}", c));

    let vec_again: Vec<char> = vec!['W', 'o', 'r', 'l', 'd']; // create a vector of characters
    println!("vec_again is {:?}", vec_again); // print the vector

    let collected: String = chars.iter().collect(); // collect the characters into a string
    println!("collected is {}", collected); // print the collected string

    for c in chars {
        println!("c is {}", c);
        if c == 'o' {
            println!(", world!");
        }
    }
}

#[allow(dead_code)]
fn closures() {
    let num: i32 = 10;
    let add_num = |x: i32| x + num; // closure that adds num to its argument
    let result = add_num(5); // call the closure with an argument
    println!("result is {}", result); // print the result
}

pub fn add_five(num: i32) -> i32 {
    num + 5
}

pub fn sub_five(num: i32) -> i32 {
    num - 5
}

#[cfg(test)]
mod test {
    use super::*; // inherits all the libs/mods from above

    #[test]
    fn adds_five_test() {
        let x: i32 = 100;
        let y = add_five(x);

        println!("{} plus five is: {}", x, y);
        assert_eq!(y, 105);
    }

    #[test]
    fn sub_five_test() {
        let x: i32 = 100;
        let y = sub_five(x);

        println!("{} minus five is: {}", x, y);
        assert_eq!(y, 95);
    }
}

fn main() {
    println!("Hello, world!");
    m3_traits::letrun();
    p4_polymorph::create_user();
}
