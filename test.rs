#[cfg(test)]
mod tests {
    use crate::brr;
    #[test]
    fn getters() {
        let mut brr_arr = brr::Brr::new();
        brr_arr.from_vec(Vec::from([1, 2, 3, 4, 5]));
        assert!(*brr_arr.get(0).unwrap() == 1);
        assert!(*brr_arr.get(1).unwrap() == 2);
        assert!(*brr_arr.get(2).unwrap() == 3);
        assert!(*brr_arr.get(3).unwrap() == 4);
        assert!(*brr_arr.get(4).unwrap() == 5);
        assert_eq!(brr_arr.to_vec(), Vec::from([1, 2, 3, 4, 5]));
        brr_arr.head();
        brr_arr.tail();
        assert!(*brr_arr.get(2).unwrap() == 4);
        brr_arr.set(2, 100);
        assert!(*brr_arr.get(2).unwrap() == 100);
        brr_arr.tail().tail().tail().tail();
        brr_arr.from_vec(Vec::from([1, 2, 3, 4, 5, 6, 7, 8, 9]));
        brr_arr.head().head().head().head().head().head();
        brr_arr.from_vec(Vec::from([1, 2, 3, 4, 5, 6, 7, 8, 9]));
        brr_arr.tail().tail().tail().tail().tail().tail().tail();
        brr_arr.prepend(42);
        assert!(*brr_arr.get(0).unwrap() == 42);
        let mut my_arr: brr::Brr<i32> = brr::Brr::new();
        my_arr.from_vec(Vec::from([1; 1000]));
        my_arr.set(512, 9);
    }
    #[test]
    fn map() {
        let mut brr_arr = brr::Brr::new();
        brr_arr.from_vec(Vec::from([1, 2, 3, 4, 5]));
        let brr_map = brr_arr.map(|x, i| x * 2 + i);
        assert_eq!(brr_map.to_vec(), Vec::from([2, 5, 8, 11, 14]));
    }
    #[test]
    fn filter() {
        let mut brr_arr = brr::Brr::new();
        brr_arr.from_vec(Vec::from([1, 2, 3, 4, 5]));
        let brr_filter = brr_arr.filter(|x, _i| x % 2 == 0);
        assert_eq!(brr_filter.to_vec(), Vec::from([2, 4]));
    }
    #[test]
    fn rotate() {
        {
            let mut brr_arr = brr::Brr::new();
            brr_arr.from_vec(Vec::from([1, 2, 3])).rotate(2);
            assert_eq!(brr_arr.to_vec(), Vec::from([2, 3, 1]));
        }

        {
            let mut brr_arr = brr::Brr::new();
            brr_arr.from_vec(Vec::from([1, 2, 3])).rotate(-2);
            assert_eq!(brr_arr.to_vec(), Vec::from([3, 1, 2]));
        }
        {
            let mut brr_arr = brr::Brr::new();
            brr_arr.from_vec(Vec::from([1, 2, 3])).rotate(2 * 100 * -1);
            assert_eq!(brr_arr.to_vec(), Vec::from([3, 1, 2]));
        }

        {
            let mut brr_arr = brr::Brr::new();
            brr_arr.from_vec(Vec::from([1, 2, 3])).rotate(0);
            assert_eq!(brr_arr.to_vec(), Vec::from([1, 2, 3]));
        }

        {
            let mut brr_arr = brr::Brr::new();
            brr_arr
                .from_vec(Vec::from([1, 2, 3]))
                .rotate_left(3)
                .rotate_right(3);
            assert_eq!(brr_arr.to_vec(), Vec::from([1, 2, 3]));
        }
        {
            let mut brr_arr = brr::Brr::new();
            brr_arr.from_vec(Vec::from([1, 2, 3])).rotate(3);
            assert_eq!(brr_arr.to_vec(), Vec::from([1, 2, 3]));
        }
        {
            let mut brr_arr = brr::Brr::new();
            brr_arr.from_vec(Vec::from([1, 2, 3])).rotate(3);
            brr_arr.tail().tail().tail();
            assert!(brr_arr.is_empty() == true);
        }
    }
    #[test]
    fn transform_char() {
        fn validate_parens(str: &str) -> bool {
            let left = "(".chars().next().unwrap();
            let right = ")".chars().next().unwrap();
            return brr::Brr::new()
                .from_vec(str.to_string().chars().collect::<Vec<char>>())
                .filter(|s, _| *s == left || *s == right)
                .transform(|mut stack, paren, _| {
                    match *paren == left {
                        true => {
                            stack.prepend(*paren);
                        }
                        false => {
                            let first = stack.first();
                            match first {
                                Some(str) => {
                                    if *str == left {
                                        stack.tail();
                                    } else if *str == right {
                                        stack.append(*paren);
                                    }
                                }
                                None => {
                                    stack.append(*paren);
                                }
                            }
                        }
                    }

                    return stack;
                })
                .is_empty();
        }

        assert!(validate_parens("") == true);
        assert!(validate_parens("(())") == true);
        assert!(validate_parens("(()") == false);
        assert!(validate_parens("((()))()()(())()") == true);
        assert!(validate_parens("(((((") == false);
        assert!(validate_parens("((1 + 2) * 4) / 2 + 10") == true);
        assert!(validate_parens("((10 + 4) ^ 2) + (7 + 10) ^ 3") == true);
        assert!(validate_parens("(10 + 4) ^ 2) + (7 + 10) ^ 3") == false);
    }
    #[test]
    fn transform_str() {
        fn validate_parens(str: &str) -> bool {
            return brr::Brr::new()
                .from_vec(str.to_string().split("").collect::<Vec<&str>>())
                .filter(|s, _| *s == "(" || *s == ")")
                .transform(|mut stack, paren, _| {
                    match *paren == "(" {
                        true => {
                            stack.prepend(paren);
                        }
                        false => {
                            let first = stack.first();
                            match first {
                                Some(s) => {
                                    if *s == "(" {
                                        stack.tail();
                                    } else if *s == ")" {
                                        stack.append(paren);
                                    }
                                }
                                None => {
                                    stack.append(paren);
                                }
                            }
                        }
                    }

                    return stack;
                })
                .is_empty();
        }

        assert!(validate_parens("") == true);
        assert!(validate_parens("(())") == true);
        assert!(validate_parens("(()") == false);
        assert!(validate_parens("((()))()()(())()") == true);
        assert!(validate_parens("(((((") == false);
        assert!(validate_parens("((1 + 2) * 4) / 2 + 10") == true);
        assert!(validate_parens("((10 + 4) ^ 2) + (7 + 10) ^ 3") == true);
        assert!(validate_parens("(10 + 4) ^ 2) + (7 + 10) ^ 3") == false);
    }
    #[test]
    fn slice() {
        let mut brr = brr::Brr::new();
        let numbers = Vec::from([1, 2, 3, 4, 5]);
        brr.from_vec(numbers);
        let s1 = brr.slice(2, 0);
        assert_eq!(s1.to_vec(), [3, 4, 5]);
        let s2 = brr.slice(1, 3);
        assert_eq!(s2.to_vec(), [2, 3]);
        let s3 = brr.slice(0, 3);
        assert_eq!(s3.to_vec(), [1, 2, 3]);
    }
    #[test]
    fn insert_remove() {
        {
            let mut brr = brr::Brr::new();
            let numbers = Vec::from([1, 2, 3, 4, 5]);
            let a = brr.from_vec(numbers.clone());
            assert_eq!(
                a.insert_many(0, Vec::from([0, 0, 0])).to_vec(),
                Vec::from([0, 0, 0, 1, 2, 3, 4, 5])
            );
            let b = brr.from_vec(numbers.clone());
            assert_eq!(
                b.insert_many(3, Vec::from([10, 20, 30])).to_vec(),
                Vec::from([1, 2, 3, 10, 20, 30, 4, 5])
            );
            let c = brr.from_vec(numbers.clone());
            assert_eq!(
                c.insert_many(5, Vec::from([10, 20, 30])).to_vec(),
                Vec::from([1, 2, 3, 4, 5, 10, 20, 30])
            );
            let d = brr.from_vec(numbers.clone());
            assert_eq!(d.insert(2, 100).to_vec(), Vec::from([1, 2, 100, 3, 4, 5]));
            let e = brr.from_vec(numbers.clone());
            assert_eq!(e.insert(100, 100).to_vec(), Vec::from([1, 2, 3, 4, 5]));
        }

        {
            let mut brr = brr::Brr::new();
            let numbers = Vec::from([1, 2, 3, 4, 5]);
            let a = brr.from_vec(numbers.clone());
            assert_eq!(a.remove_many(0, 3).to_vec(), Vec::from([4, 5]));
            let b = brr.from_vec(numbers.clone());
            assert_eq!(b.remove_many(3, 1).to_vec(), Vec::from([1, 2, 3, 5]));
            let c = brr.from_vec(numbers.clone());
            assert_eq!(c.remove_many(4, 1).to_vec(), Vec::from([1, 2, 3, 4]));
            let d = brr.from_vec(numbers.clone());
            assert_eq!(d.remove(2).to_vec(), Vec::from([1, 2, 4, 5]));
            let e = brr.from_vec(numbers.clone());
            assert_eq!(e.remove(100).to_vec(), Vec::from([1, 2, 3, 4, 5]));
        }
    }
    #[test]
    fn at() {
        let mut brr_arr = brr::Brr::new();
        brr_arr.from_vec(Vec::from([1, 2, 3, 4, 5]));
        assert!(*brr_arr.at(0).unwrap() == 1);
        assert!(*brr_arr.at(1).unwrap() == 2);
        assert!(*brr_arr.at(2).unwrap() == 3);
        assert!(*brr_arr.at(3).unwrap() == 4);
        assert!(*brr_arr.at(4).unwrap() == 5);
        assert!(*brr_arr.at(-1).unwrap() == 5);
        assert!(*brr_arr.at(-2).unwrap() == 4);
        assert!(*brr_arr.at(-3).unwrap() == 3);
        assert!(*brr_arr.at(-4).unwrap() == 2);
        assert!(*brr_arr.at(-5).unwrap() == 1);
    }
    #[test]
    fn chop_cut() {
        let mut brr_arr = brr::Brr::new();
        brr_arr.from_vec(Vec::from([1, 2, 3, 4, 5]));
        for _ in 0..10 {
            brr_arr.chop();
        }
        brr_arr.from_vec(Vec::from([1, 2, 3, 4, 5]));
        for _ in 0..10 {
            brr_arr.cut();
        }
    }
    #[test]
    fn partition() {
        let mut b = brr::Brr::new();
        let out = b.from_vec(Vec::from([1, 2, 3, 4])).partition(2);
        assert_eq!(*out.get(0).unwrap().to_vec(), Vec::from([1, 2]));
        assert_eq!(*out.get(1).unwrap().to_vec(), Vec::from([3, 4]));
        assert_eq!(brr::flat(out).to_vec(), Vec::from([1, 2, 3, 4]));
    }
    #[test]
    fn for_each() {
        let mut b = brr::Brr::new();
        let mut z = 10;
        b.from_vec(Vec::from([1, 2, 3, 4])).for_each(|x, _| z += x);
        assert!(z == 20)
    }

    #[test]
    fn flat() {
        let mut b = brr::Brr::new();
        let input = Vec::from([1, 2, 3, 4, 5, 6, 7, 8]);
        let expected = input.clone();
        let deep = b.from_vec(input).partition(3);
        assert_eq!(brr::flat(deep).to_vec(), expected);
    }

    #[test]
    fn find() {
        assert!(
            brr::Brr::new()
                .from_vec(Vec::from([1, 2, 3, 4, 5]))
                .find(|x, _| *x == 3)
                .unwrap()
                == 3
        );
        assert!(
            brr::Brr::new()
                .from_vec(Vec::from([1, 2, 3, 4, 5]))
                .find(|x, _| *x == 5)
                .unwrap()
                == 5
        );
        assert!(
            brr::Brr::new()
                .from_vec(Vec::from([1, 2, 3, 4, 5]))
                .find(|x, _| *x == 15)
                == None
        );
    }
}
