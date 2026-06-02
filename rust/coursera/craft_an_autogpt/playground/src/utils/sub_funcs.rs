pub fn add_nine(num: i32) -> i32 {
    num + 9
}

pub fn sub_nine(num: i32) -> i32 {
    num - 9
}

// Writing Unit Tests
#[cfg(test)]
mod test {
    use super::*; // inherits all the libs/mods from above

    #[test]
    fn adds_nine_test() {
        let x: i32 = 100;
        let y = add_nine(x);

        println!("{} plus nine is: {}", x, y);
        assert_eq!(y, 109);
    }
}