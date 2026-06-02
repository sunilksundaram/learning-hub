/**
 * Function: add_five
 * 
 * # Arguments: (num: i32)
 * 
 * # Returns: i32
 * 
 * # Example:
 * 
 * ```
 * let x = 5;
 * let y = add_five(x);
 * ```
 * 
 * 
 */
pub fn add_five(num: i32) -> i32 {
    num + 5
}

pub fn sub_five(num: i32) -> i32 {
    num - 5
}

// Writing Unit Tests
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
}
