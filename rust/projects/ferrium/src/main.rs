use std::collections::HashMap;

fn count_occurrences(text: &str) -> HashMap<char, usize> {
    let mut occurrences = HashMap::new();
    for c in text.chars() {
        *occurrences.entry(c).or_insert(0) += 1;
    }
    occurrences
}

fn main() {
    let text = "TV quiz jock Mr.Lynx, bags few PhD"; // perfect pangrams // bag fry nymph vex'd quicks   
    let occurrences = count_occurrences(text);
    println!("{:?}", occurrences);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_count_occurrences() {
        let text = "hello";
        let occurrences = count_occurrences(text);
        assert_eq!(occurrences.get(&'h'), Some(&1));
        assert_eq!(occurrences.get(&'e'), Some(&1));
        assert_eq!(occurrences.get(&'l'), Some(&2));
        assert_eq!(occurrences.get(&'o'), Some(&1));
    }   
}