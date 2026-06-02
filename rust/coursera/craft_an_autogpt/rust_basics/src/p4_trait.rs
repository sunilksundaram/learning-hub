enum Character {
    Wizard,
    Knight,
    Archer,
    Thief,
}

trait Attack {
    fn attack(&self) -> String;
}

trait Fight {
    fn fight(&self) -> String;
}

impl Attack for Character {
    fn attack(&self) -> String {
        match self {
            Character::Archer => "Attacking with a bow and arrow".to_string(),
            Character::Knight => "Attacking with a sword".to_string(),
            Character::Thief => "Attacking with a dagger".to_string(),
            Character::Wizard => "Attacking with a spell".to_string(),
        }
    }
}

impl Fight for Character {
    fn fight(&self) -> String {
        match self {
            Character::Archer => "Fighting from a distance".to_string(),
            Character::Knight => "Fighting up close".to_string(),
            Character::Thief => "Fighting with stealth".to_string(),
            Character::Wizard => "Fighting with magic".to_string(),
        }
    }
}

// ================== Structs =================
#[derive(Debug)]
struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: u64,
}

impl User {
    fn new(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    fn incr_signin_count(&mut self) {
        self.sign_in_count += 1;
    }
}


pub fn create_user() {
    let mut user1: User = User::new(String::from("user1@example.com"), String::from("user1"));
    println!("User1: {:?}", user1);

    user1.incr_signin_count();
    println!("User1 after incr sign-in count: {:?}", user1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user() {
        create_user();
    }
}