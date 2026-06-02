trait Attack {
    fn choose_style(&self) -> String;
    fn choose_weapon(&self) -> String;
}

#[derive(Debug)]
enum Characters {
    Warrior,
    Archer,
    Wizard,
}

impl Attack for Characters {
    fn choose_style(&self) -> String {
        match self {
            Characters::Archer => "wing.chun".to_string(),
            Characters::Warrior => "kung fu".to_string(),
            Characters::Wizard => "thai chi".to_string(),
        }
    }

    fn choose_weapon(&self) -> String {
        match self {
            Characters::Archer => "bow arrow".to_string(),
            Characters::Warrior => "sword".to_string(),
            Characters::Wizard => "staff".to_string(),
        }
    }
}

#[derive(Debug)]
enum CarColour {
    Red,
    Yellow,
    Green,
    Blue,
    Black,
    Silver, 
}

impl CarColour {
    fn create_red_car() -> CarColour {
        CarColour::Red
    }
}

pub fn letrun() {
    let my_car = CarColour::create_red_car();
    println!("My car colour is: {:?}", my_car);

    let my_character: Characters = Characters::Warrior;
    let my_fighting_style = my_character.choose_style();    
    let my_weapon = my_character.choose_weapon();
    println!("My Style is: {} ", my_fighting_style);
    println!("My Weapon is: {} ", my_weapon);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_character_traits() {
        dbg!("hello!");
        let my_character: Characters = Characters::Warrior;
        let my_fighting_style = my_character.choose_style();
        let my_weapon = my_character.choose_weapon();

        dbg!("My Style is: {} ", my_fighting_style);
        dbg!("My Weapon is: {} ", my_weapon);
    }
}
