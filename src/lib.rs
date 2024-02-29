pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}
impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        // this evalueate to bool which is perfect for testing with !assert
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}
pub fn greeting(_name: &str) -> String {
    String::from("Hello!")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {length: 8, width: 7};
        let smaller = Rectangle {length: 5, width: 1};

        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {length: 8, width: 7};
        let smaller = Rectangle {length: 5, width: 1};

        assert!(!smaller.can_hold(&larger));
    }
    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_adds_two() {
        assert_ne!(4, add_two(3));
    }
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
        result.contains("Hello"),
        "Greeting did not contain name, value was `{}`", result
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two_2nd(2));
    }
    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two_2nd(3));
    }
    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two_2nd(100));
    }
    

    // #[test]
    // fn another() {
    //     panic!("make this test fail");
    // }
}

// should panic test, testing the correctness of error handling in our code
//
pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.", value);
        }
        Guess {
            value
        }
    }
}
pub fn add_two_2nd(a: i32) -> i32 {
    a + 2
}
