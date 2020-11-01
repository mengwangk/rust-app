#[cfg(test)]
mod tests {
    #[test]
    fn larger_can_hold_smaller() {
        let larger = mylib3::Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = mylib3::Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = mylib3::Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = mylib3::Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, mylib3::add_two(2));
    }
}
