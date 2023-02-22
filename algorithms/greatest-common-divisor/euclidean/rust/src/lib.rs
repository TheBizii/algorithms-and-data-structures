pub fn euclid(mut first: i32, mut second: i32) -> i32 {
    first = first.abs();
    second = second.abs();

    while second != 0 {
        let tmp = first % second;
        first = second;
        second = tmp;
    }

    first
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_input() {
        assert_eq!(euclid(230, 172), 2);
        assert_eq!(euclid(215, 256), 1);
        assert_eq!(euclid(234, 246), 6);
        assert_eq!(euclid(171, 187), 1);
        assert_eq!(euclid(67, 149), 1);
    }

    #[test]
    fn zero_input() {
        assert_eq!(euclid(0, 46), 46);
        assert_eq!(euclid(0, 175), 175);
        assert_eq!(euclid(118, 0), 118);
        assert_eq!(euclid(266, 0), 266);
        assert_eq!(euclid(0, 206), 206);
    }

    #[test]
    fn negative_input() {
        assert_eq!(euclid(-39, 333), 3);
        assert_eq!(euclid(39, -333), 3);
        assert_eq!(euclid(-311, 246), 1);
        assert_eq!(euclid(-206, -142), 2);
        assert_eq!(euclid(-67, 149), 1);
    }

    #[test]
    fn big_numbers() {
        assert_eq!(euclid(652017495, 1541617144), 1);
        assert_eq!(euclid(-1661344126, 1887766744), 2);
        assert_eq!(euclid(111111111, 999999999), 111111111);
    }

    #[test]
    fn multiple_numbers() {
        assert_eq!(euclid(euclid(euclid(30, 40), 60), 70), 10);
    }
}
