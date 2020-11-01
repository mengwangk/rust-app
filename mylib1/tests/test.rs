#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2, mylib1::add_one(1));
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

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
}
