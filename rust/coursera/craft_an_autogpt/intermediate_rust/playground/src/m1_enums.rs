#[derive(Debug)] // now we can Debug it if needed
enum CarColour {
    Red,
    Green, 
    Blue, 
    Silver
}

fn create_car_colour_blue() -> CarColour {
    let my_car_colour: CarColour = CarColour::Green;
    my_car_colour
}

#[derive(Debug)]
enum GivenResult<T, E> {
    Ok(T),
    Err(E)
}

fn check_five_old(num_check: u8) -> bool {
    if num_check < 5 {
        true
    } else {
       false 
    }
}

fn check_five(num_check: u8) -> GivenResult<u8, String> {
    if num_check < 5 {
        GivenResult::Ok(num_check)
    } else {
        GivenResult::Err("Not under 5!".to_string())
    }
}

fn check_five_built_in(num_check: u8) -> Result<u8, String> {
    if num_check < 5 {
        Ok(num_check)
    } else {
        Err("Not under 5!".to_string())
    }
}

#[derive(Debug)]
enum GivenOption<T> {
    None,
    Some(T)
}

fn remainder_zero(num: f32) -> GivenOption<f32> {
    let val: f32 = num % 10.0; 
    if  val == 0.0 {
        GivenOption::None
    } else {
        GivenOption::Some(val)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_enums(){
        let car_colour = create_car_colour_blue();
        // dbg!("Hello");
        dbg!(car_colour);

        let is_under_five_res = check_five_old(5);
        dbg!(is_under_five_res);

        let is_under_five_res_2 = check_five(7);
        dbg!(is_under_five_res_2);

        let is_under_five_res_3 = check_five_built_in(7)?;
        dbg!(is_under_five_res_3);

        let remainder = remainder_zero(32.0);
        dbg!(remainder);
    }
}