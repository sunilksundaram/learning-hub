/// Adds two 64-bit unsigned integers and returns the result.
///
/// # Arguments
/// 
/// # `left` - a usize to add
/// # `right` - a usize to add
/// 
/// # Example
///
/// ```
/// use my_crate::add;
///
/// let result = add(2, 2);
/// assert_eq!(result, 4);
/// ```
///
/// # Panics
/// 
/// This function will panic if the addition overflows in debug builds, 
/// or wrap around in release builds (unless compiled with overflow checks).


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// Allowing Dead Code - if necessary
// Method 1: Use _ (undescore)
fn _myfunc1(_num: i32) {
    let _x = 25;
    let _z = 33;
}

// Method 2: Use Procedural Macro - allow
#[allow(dead_code, unused_variables)]
fn myfunc2(num:i32) {
    let x = 25;
    let z = 33;
}