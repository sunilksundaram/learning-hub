use std::{f32, io, u8};

// convert deg C to deg F
fn convert_to_f(deg_c: f32) -> f32 {
    let deg_f = (deg_c * 9.0) / 5.0 + 32.0;
    deg_f
}

// convert deg F to deg C
fn convert_to_c(deg_f: f32) -> f32 {
    let deg_c = ((deg_f - 32.0) * 5.0) / 9.0;
    deg_c
}

fn main() {
    println!("Hello, world!");

    loop {
        println!("Input 1 for deg C to deg F, 2 for deg F to deg C, e for exit:");
        let mut conv_option = String::new();
        io::stdin()
            .read_line(&mut conv_option)
            .expect("Error while reading option input!");

        if conv_option.trim() == "e" {
            println!("Exiting - Bye!");
            break;
        }

        let conv_num: u8 = match conv_option.trim().parse::<u8>() {
            Ok(num) => {
                if num < 1 || num > 2 {
                    println!("Please enter a valid number - 1 or 2!");
                    continue;
                } else {
                    num
                }
            }
            Err(_) => {
                println!("Please enter a valid number - 1 or 2!");
                continue;
            }
        };

        match conv_num {
            1 => {
                //deg C to deg F
                println!("Enter deg C to convert: ");
                let mut deg_c = String::new();
                io::stdin()
                    .read_line(&mut deg_c)
                    .expect("Error while reading deg C input!");

                let deg_c_num: f32 = match deg_c.trim().parse::<f32>() {
                    Ok(c_num) => c_num,
                    Err(_) => {
                        println!("Please enter a valid deg C number!");
                        continue;
                    }
                };

                let converted_to_f: f32 = convert_to_f(deg_c_num);
                println!("{} deg C is {} deg F", deg_c_num, converted_to_f);
            }
            2 => {
                // deg F to deg C
                println!("Enter deg F to convert: ");
                let mut deg_f = String::new();
                io::stdin()
                    .read_line(&mut deg_f)
                    .expect("Error while reading deg F input!");

                let deg_f_num: f32 = match deg_f.trim().parse::<f32>() {
                    Ok(f_num) => f_num,
                    Err(_) => {
                        println!("Please enter a valid deg C number!");
                        continue;
                    }
                };

                let converted_to_c: f32 = convert_to_c(deg_f_num);
                println!("{} deg F is {} deg C", deg_f_num, converted_to_c);
            }
            _ => {
                println!("Please enter a valid number - 1 or 2!");
                continue;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_f() {
        let to_f: f32 = convert_to_f(0.0);
        assert_eq!(to_f, 0.0);
    }
}
