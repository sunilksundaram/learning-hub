#[derive(Debug)]
#[allow(dead_code)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[allow(dead_code)]
fn change_username(user: &mut User, new_username: String) {
    user.username = new_username;
}

// ================== Tuple Structs =================

#[allow(dead_code)]
impl User {
    fn incr_signin_count(&mut self) {
        self.sign_in_count += 1;
    }

    fn change_email_id(&mut self, new_email: &str) {
        self.email = new_email.to_string();
    }
}

// ================== Unit Tests for Structs =================

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let s: String = String::from("Hello, world!");
        assert!(s.contains("world"));
        dbg!("s is {}", s);
    }

    #[test]
    fn test_build_user() {
        let user: User = User {
            email: String::from("hello@hello.com"),
            username: String::from("hello"),
            active: true,
            sign_in_count: 1,
        };

        dbg!("user is {:?}", user);
    }

    #[test]
    fn test_build_mut_user() {
        let mut user: User = User {
            email: String::from("hello@hello.com"),
            username: String::from("hello"),
            active: true,
            sign_in_count: 1,
        };

        user.username = String::from("world");
        dbg!("user is {:?}", user);
    }

    #[test]
    fn test_change_username() {
        let mut user: User = User {
            email: String::from("hello@hello.com"),
            username: String::from("hello"),
            active: true,
            sign_in_count: 1,
        };

        let new_username = String::from("dello");
        change_username(&mut user, new_username);
        dbg!("user is {:?}", user);
    }

    #[test]
    fn test_change_user_details() {
        let mut user: User = User {
            email: String::from("hello@hello.com"),
            username: String::from("hello"),
            active: true,
            sign_in_count: 1,
        };

        let new_email: String = String::from("mynew@hello.com");
        user.change_email_id(&new_email);
        user.incr_signin_count();

        dbg!("user: {}", user);
    }
}
