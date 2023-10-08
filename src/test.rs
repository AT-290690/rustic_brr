#[cfg(test)]
mod tests {
    use crate::brr;
    #[test]
    fn getters() {
        let mut brr_arr = brr![1, 2, 3, 4, 5];
        assert_eq!(*brr_arr.get(0).unwrap(), 1);
        assert_eq!(*brr_arr.get(1).unwrap(), 2);
        assert_eq!(*brr_arr.get(2).unwrap(), 3);
        assert_eq!(*brr_arr.get(3).unwrap(), 4);
        assert_eq!(*brr_arr.get(4).unwrap(), 5);
        assert_eq!(brr_arr.to_vec(), vec![1, 2, 3, 4, 5]);
        brr_arr.head();
        brr_arr.tail();
        assert_eq!(*brr_arr.get(2).unwrap(), 4);
        brr_arr.set(2, 100);
        assert_eq!(*brr_arr.get(2).unwrap(), 100);
        brr_arr.tail().tail().tail().tail();
        brr_arr.from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        brr_arr.head().head().head().head().head().head();
        brr_arr.from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        brr_arr.tail().tail().tail().tail().tail().tail().tail();
        brr_arr.prepend(42);
        assert_eq!(*brr_arr.get(0).unwrap(), 42);
        let mut my_arr: brr::Brr<i32> = brr::Brr::new();
        my_arr.from_vec(vec![1; 1000]);
        my_arr.set(512, 9);
    }
    #[test]
    fn map() {
        let mut brr_arr = brr![1, 2, 3, 4, 5];
        let brr_map = brr_arr.iterate(|x, i| x * 2 + i);
        assert_eq!(brr_map.to_vec(), vec![2, 5, 8, 11, 14]);
    }
    #[test]
    fn filter() {
        assert_eq!(
            brr![1, 2, 3, 4, 5].filter(|x, _| x % 2 == 0).to_vec(),
            vec![2, 4]
        );
        assert_eq!(
            brr![1, 2, 3, 4, 5].select(|x| x % 2 == 0).to_vec(),
            vec![2, 4]
        );
        assert_eq!(
            brr![1, 2, 3, 4, 5].except(|x| x % 2 == 0).to_vec(),
            vec![1, 3, 5]
        )
    }
    #[test]
    fn swap() {
        assert_eq!(brr![1, 2, 3].swap(0, 1).to_vec(), vec![2, 1, 3]);
        {
            let mut brr = brr![1, 2, 3, 4, 5];
            assert_eq!(brr.swap_remove_left(2).to_vec(), vec![2, 3, 1, 5]);
            assert_eq!(brr.swap_remove_right(2).to_vec(), vec![2, 3, 5])
        }
    }
    #[test]
    fn rotate() {
        assert_eq!(brr![1, 2, 3].rotate(2).to_vec(), vec![2, 3, 1]);
        assert_eq!(brr![1, 2, 3].rotate(-2).to_vec(), vec![3, 1, 2]);
        assert_eq!(brr![1, 2, 3].rotate(2 * 100 * -1).to_vec(), vec![3, 1, 2]);
        assert_eq!(brr![1, 2, 3].rotate(0).to_vec(), vec![1, 2, 3]);
        assert_eq!(
            brr![1, 2, 3].rotate_left(3).rotate_right(3).to_vec(),
            vec![1, 2, 3]
        );
        assert_eq!(brr![1, 2, 3].rotate(3).to_vec(), vec![1, 2, 3]);

        let mut brr_arr = brr![1, 2, 3];
        brr_arr.rotate(3).tail().tail().tail();
        assert_eq!(brr_arr.is_empty(), true);
    }
    #[test]
    fn transform_char() {
        fn validate_parens(str: &str) -> bool {
            let left = "(".chars().next().unwrap();
            let right = ")".chars().next().unwrap();
            return brr::Brr::new()
                .from_vec(str.to_string().chars().collect::<Vec<char>>())
                .select(|s| *s == left || *s == right)
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

        assert_eq!(validate_parens(""), true);
        assert_eq!(validate_parens("(())"), true);
        assert_eq!(validate_parens("(()"), false);
        assert_eq!(validate_parens("((()))()()(())()"), true);
        assert_eq!(validate_parens("((((("), false);
        assert_eq!(validate_parens("((1 + 2) * 4) / 2 + 10"), true);
        assert_eq!(validate_parens("((10 + 4) ^ 2) + (7 + 10) ^ 3"), true);
        assert_eq!(validate_parens("(10 + 4) ^ 2) + (7 + 10) ^ 3"), false);
    }
    #[test]
    fn transform_str() {
        fn validate_parens(str: &str) -> bool {
            return brr::Brr::new()
                .from_vec(str.to_string().split("").collect::<Vec<&str>>())
                .select(|s| *s == "(" || *s == ")")
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

        assert_eq!(validate_parens(""), true);
        assert_eq!(validate_parens("(())"), true);
        assert_eq!(validate_parens("(()"), false);
        assert_eq!(validate_parens("((()))()()(())()"), true);
        assert_eq!(validate_parens("((((("), false);
        assert_eq!(validate_parens("((1 + 2) * 4) / 2 + 10"), true);
        assert_eq!(validate_parens("((10 + 4) ^ 2) + (7 + 10) ^ 3"), true);
        assert_eq!(validate_parens("(10 + 4) ^ 2) + (7 + 10) ^ 3"), false);
    }
    #[test]
    fn slice() {
        let brr = brr![1, 2, 3, 4, 5];
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
            let numbers = vec![1, 2, 3, 4, 5];
            let a = brr.from_vec(numbers.clone());
            assert_eq!(
                a.insert_many(0, vec![0, 0, 0]).to_vec(),
                vec![0, 0, 0, 1, 2, 3, 4, 5]
            );
            let b = brr.from_vec(numbers.clone());
            assert_eq!(
                b.insert_many(3, vec![10, 20, 30]).to_vec(),
                vec![1, 2, 3, 10, 20, 30, 4, 5]
            );
            let c = brr.from_vec(numbers.clone());
            assert_eq!(
                c.insert_many(5, vec![10, 20, 30]).to_vec(),
                vec![1, 2, 3, 4, 5, 10, 20, 30]
            );
            let d = brr.from_vec(numbers.clone());
            assert_eq!(d.insert(2, 100).to_vec(), vec![1, 2, 100, 3, 4, 5]);
            let e = brr.from_vec(numbers.clone());
            assert_eq!(e.insert(100, 100).to_vec(), vec![1, 2, 3, 4, 5]);
        }

        {
            let mut brr = brr::Brr::new();
            let numbers = vec![1, 2, 3, 4, 5];
            let a = brr.from_vec(numbers.clone());
            assert_eq!(a.remove_many(0, 3).to_vec(), vec![4, 5]);
            let b = brr.from_vec(numbers.clone());
            assert_eq!(b.remove_many(3, 1).to_vec(), vec![1, 2, 3, 5]);
            let c = brr.from_vec(numbers.clone());
            assert_eq!(c.remove_many(4, 1).to_vec(), vec![1, 2, 3, 4]);
            let d = brr.from_vec(numbers.clone());
            assert_eq!(d.remove(2).to_vec(), vec![1, 2, 4, 5]);
            let e = brr.from_vec(numbers.clone());
            assert_eq!(e.remove(100).to_vec(), vec![1, 2, 3, 4, 5]);
        }
    }
    #[test]
    fn at() {
        let brr_arr = brr![1, 2, 3, 4, 5];
        assert_eq!(*brr_arr.at(0).unwrap(), 1);
        assert_eq!(*brr_arr.at(1).unwrap(), 2);
        assert_eq!(*brr_arr.at(2).unwrap(), 3);
        assert_eq!(*brr_arr.at(3).unwrap(), 4);
        assert_eq!(*brr_arr.at(4).unwrap(), 5);
        assert_eq!(*brr_arr.at(-1).unwrap(), 5);
        assert_eq!(*brr_arr.at(-2).unwrap(), 4);
        assert_eq!(*brr_arr.at(-3).unwrap(), 3);
        assert_eq!(*brr_arr.at(-4).unwrap(), 2);
        assert_eq!(*brr_arr.at(-5).unwrap(), 1);
    }
    #[test]
    fn chop_cut() {
        let mut brr_arr = brr![1, 2, 3, 4, 5];
        for _ in 0..10 {
            brr_arr.chop();
        }
        brr_arr.from_vec(vec![1, 2, 3, 4, 5]);
        for _ in 0..10 {
            brr_arr.cut();
        }
    }
    #[test]
    fn partition() {
        let out = brr![1, 2, 3, 4].partition(2);
        assert_eq!(*out.get(0).unwrap().to_vec(), vec![1, 2]);
        assert_eq!(*out.get(1).unwrap().to_vec(), vec![3, 4]);
        assert_eq!(brr::flat(out).to_vec(), vec![1, 2, 3, 4]);
    }
    #[test]
    fn partition_if() {
        {
            let out = brr![1, 2, 3, 4, 5, 6].partition_if(|_, i| i % 2 == 1);
            assert_eq!(
                out.fold(Vec::new(), |mut a, x, i| {
                    a.insert(i, x.to_vec());
                    a
                },),
                [[2, 4, 6], [1, 3, 5]]
            );
        }
        {
            assert_eq!(
                brr::flat(brr![1, 2, 3, 4, 5, 6].partition_if(|x, _| x % 2 != 0)).to_vec(),
                [1, 3, 5, 2, 4, 6]
            )
        }
        {
            assert_eq!(
                brr![1, 2, 3, 4, 5, 6]
                    .partition_if(|x, _| x % 2 != 0)
                    .to(brr::Brr::new(), |mut a, x| {
                        a.concat(x);
                        return a;
                    })
                    .to_vec(),
                [1, 3, 5, 2, 4, 6]
            )
        }
    }
    #[test]
    fn range() {
        assert_eq!(brr::range(1, 5).to_vec(), vec![1, 2, 3, 4, 5]);
        assert_eq!(brr::range(5, 10).to_vec(), vec![5, 6, 7, 8, 9, 10]);
        assert_eq!(brr::range(1, 5).map(|x| x * 10).to_vec(), vec![10, 20, 30, 40, 50]);
        assert_eq!(brr::range(1, 5).map(|x| x * 10).to(0, |a, b| a + b), 150);
    }
    #[test]
    fn for_each() {
        let mut z = 10;
        brr![1, 2, 3, 4].for_each(|x, _| z += x);
        assert_eq!(z, 20)
    }

    #[test]
    fn flat() {
        let input = brr![1, 2, 3, 4, 5, 6, 7, 8];
        let expected = input.clone();
        let deep = input.partition(3);
        assert_eq!(brr::flat(deep).to_vec(), expected.to_vec());
    }

    #[test]
    fn find() {
        assert_eq!(*brr![1, 2, 3, 4, 5].find(|x| *x == 3).unwrap(), 3);
        assert_eq!(*brr![1, 2, 3, 4, 5].find(|x| *x == 5).unwrap(), 5);
        assert_eq!(brr![1, 2, 3, 4, 5].find(|x| *x == 15), None);
        assert_eq!(brr![1, 2, 3, 4, 5].find_index(|x| *x == 3).unwrap(), 2);
        assert_eq!(brr![1, 2, 3, 4, 5].find_index(|x| *x == 5).unwrap(), 4);
        assert_eq!(brr![1, 2, 3, 4, 5].find_index(|x| *x == 15), None);
    }
    #[test]
    fn concat() {
        assert_eq!(
            brr![1, 2, 3, 4, 5]
                .concat(&brr![10, 20, 30, 40, 50])
                .to_vec(),
            [1, 2, 3, 4, 5, 10, 20, 30, 40, 50]
        );
        assert_eq!(
            brr![1, 2, 3, 4, 5]
                .concat(&brr![10, 20, 30, 40, 50])
                .concat(&brr![6])
                .concat(&brr![32, -2, 12])
                .to_vec(),
            [1, 2, 3, 4, 5, 10, 20, 30, 40, 50, 6, 32, -2, 12]
        );
        assert_eq!(
            brr![1, 2, 3, 4, 5].concat(&brr::Brr::new()).to_vec(),
            [1, 2, 3, 4, 5]
        );

        assert_eq!(
            brr::concat(vec![brr![1, 2, 3], brr![4, 5, 6], brr![7, 8, 9]]).to_vec(),
            brr![1, 2, 3, 4, 5, 6, 7, 8, 9].to_vec()
        )
    }
    #[test]
    fn reverse() {
        assert_eq!(brr![1, 2, 3, 4, 5].reverse().to_vec(), [5, 4, 3, 2, 1])
    }
    #[test]
    fn some_every() {
        assert_eq!(brr![1, 2, 3, 4, 5].some(|x| *x == 2), true);
        assert_eq!(brr![1, 2, 3, 4, 5].some(|x| *x == 10), false);
        assert_eq!(brr![1, 2, 3, 4, 5].every(|x| *x == 2), false);
        assert_eq!(brr![1, 2, 3, 4, 5].every(|x| *x < 10), true);
        assert_eq!(brr![2, 4, 6, 8].every(|x| *x % 2 == 0), true)
    }
    #[test]
    fn to() {
        assert_eq!(brr![1, 2, 3, 4, 5].to(0, |a, b| a + b), 15);
    }
    #[test]
    fn common() {
        assert_eq!(
            brr![1, 2, 3, 4, 5, 6, 7, 8]
                .select(|x| x % 2 == 0)
                .map(|x| x * 3)
                .rotate(-2)
                .slice(1, 4)
                .to_vec(),
            [24, 6, 12]
        )
    }
    #[test]
    fn adjacent() {
        assert_eq!(
            brr![1, 2, 3, 4].adjacent_difference(|a, b| a + b).to_vec(),
            [1, 3, 5, 7]
        );
        assert_eq!(
            brr![1, 2, 3, 4].adjacent_find(|a, b| a + b == 5).unwrap(),
            3
        );
        assert_eq!(
            brr![1, 2, 3, 4]
                .adjacent_find_index(|a, b| a + b == 5)
                .unwrap(),
            2
        );
    }
    #[test]
    fn iterator() {
        {
            let mut iter = brr![1, 2, 3, 4, 5].into_iter();
            assert_eq!(iter.next().unwrap(), 1);
            assert_eq!(iter.next().unwrap(), 2);
            assert_eq!(iter.next().unwrap(), 3);
            assert_eq!(iter.next().unwrap(), 4);
            assert_eq!(iter.next().unwrap(), 5);
        }
        {
            let mut iter = brr::Brr::from_iter(brr![1, 2, 3, 4, 5].into_iter()).into_iter();
            assert_eq!(iter.next().unwrap(), 1);
            assert_eq!(iter.next().unwrap(), 2);
            assert_eq!(iter.next().unwrap(), 3);
            assert_eq!(iter.next().unwrap(), 4);
            assert_eq!(iter.next().unwrap(), 5);
        }
    }
    #[test]
    fn zip() {
        {
            let mut arr: Vec<Vec<i32>> = Vec::new();
            let brr = brr![1, 2, 3, 4].zip(brr![-1, -2, -3, -4]);
            for i in 0..brr.length() {
                arr.push(brr.get(i).unwrap().to_vec())
            }
            assert_eq!(arr, [[1, -1], [2, -2], [3, -3], [4, -4]]);
        }
    }

    #[test]
    fn count() {
        fn maximum_count(arr: Vec<i32>) -> usize {
            let mut instance = brr::Brr::new();
            instance.from_vec(arr);
            return std::cmp::max(instance.count(|x| x < &0), instance.count(|x| x > &0));
        }
        assert_eq!(maximum_count(vec![-2, -1, -1, 1, 2, 3]), 3);
    }
}
