#[allow(dead_code)]
#[derive(Debug)]
enum CarColour {
    Red,
    Green,
    Blue,
    Silver,
}

#[allow(dead_code)]
fn create_car_color_red() -> CarColour {
    let my_car_color: CarColour = CarColour::Red;
    my_car_color
}

#[allow(dead_code)]
#[derive(Debug)]
enum GivenResult<T, E> {
    Ok(T),
    Err(E),
}

#[allow(dead_code)]
fn check_under_five(num: i32) -> GivenResult<u8, String> {
    if num < 5 {
        GivenResult::Ok(num as u8)
    } else {
        GivenResult::Err(format!("{} is greater than or equal to five", num))
    }
}

#[allow(dead_code)]
fn num_check_1(num: u8) {
    let result = num % 2;
    if result == 0 {
        println!("num remainder is 0 => {} is even", num);
    } else {
        println!("num remainder is 1 => {} is odd", num);
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum GivenOption<T> {
    None,
    Some(T),
}

#[allow(dead_code)]
fn num_check(num: f32) -> GivenOption<f32> {
    let remainder: f32 = num % 2.0;
    if remainder == 0.0 {
        println!("num remainder is 0 => {} is even", num);
        GivenOption::None
    } else {
        println!("num remainder is 1 => {} is odd", num);
        GivenOption::Some(remainder)
    }
}

//================= Built In Equivalent of the above enums =================
#[allow(dead_code)]
fn check_under_five_builtin(num: i32) -> Result<i32, String> {
    if num < 5 {
        Ok(num)
    } else {
        Err(format!("{} is greater than or equal to five", num))
    }
}

#[allow(dead_code)]
fn num_check_builtin(num: f32) -> Option<f32> {
    let remainder: f32 = num % 2.0;
    if remainder == 0.0 {
        println!("num remainder is 0 => {} is even", num);
        None
    } else {
        println!("num remainder is 1 => {} is odd", num);
        Some(remainder)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_dangle() {
        let x: String = String::from("Hello");
        dbg!(x);
    }

    #[test]
    fn test_check_car_color() {
        let car_color: CarColour = create_car_color_red();
        dbg!("car_color is {:?}", car_color);
    }

    #[test]
    fn check_under_five_test() {
        let result: GivenResult<u8, String> = check_under_five(3);
        dbg!("result is {:?}", result);
    }

    #[test]
    fn check_under_five_test_err() {
        let result: GivenResult<u8, String> = check_under_five(5);
        dbg!("result is {:?}", result);
    }

    #[test]
    fn num_check_test() {
        let result: GivenOption<f32> = num_check(3.0);
        dbg!("result is {:?}", result);
    }

    #[test]
    fn num_check_test_even() {
        let result: GivenOption<f32> = num_check(4.0);
        dbg!("result is {:?}", result);
    }

    #[test]
    fn check_under_five_test_builtin() {
        let result: i32 = check_under_five_builtin(3).unwrap_or_else(|err| {
            println!("Error: {}", err);
            -1 // return a default value in case of error
        });
        dbg!("result is {:?}", result);
    }

    #[test]
    fn check_under_five_test_builtin_1() {
        let result: Result<i32, String> = check_under_five_builtin(3);
        dbg!("result is {:?}", result);
    }

    #[test]
    fn check_under_five_test_err_builtin() {
        let result = check_under_five_builtin(5).unwrap_or_else(|err| {
            println!("Error: {}", err);
            -1 // return a default value in case of error
        });
        dbg!("result is {:?}", result);
    }

    #[test]
    fn num_check_test_builtin() {
        let result: Option<f32> = num_check_builtin(3.0);
        dbg!("result is {:?}", result);
    }

    #[test]
    fn num_check_test_even_builtin() {
        let result: Option<f32> = num_check_builtin(4.0);
        dbg!("result is {:?}", result);
    }
}
