#[cfg(test)]
mod tests {
    use crate::brr::Brr;

    #[test]
    fn it_works() {
        {
            let mut brr_arr = Brr::new();
            brr_arr.fill(vec![1, 2, 3, 4, 5]);
            assert!(*brr_arr.get(0) == 1);
            assert!(*brr_arr.get(1) == 2);
            assert!(*brr_arr.get(2) == 3);
            assert!(*brr_arr.get(3) == 4);
            brr_arr.head();
            brr_arr.tail();
            assert!(*brr_arr.get(2) == 4);
            brr_arr.set(2, 100);
            assert!(*brr_arr.get(2) == 100);
            brr_arr.tail().tail().tail().tail();
            brr_arr.fill(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
            brr_arr.head().head().head().head().head().head();
            brr_arr.fill(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
            brr_arr.tail().tail().tail().tail().tail().tail().tail();
            brr_arr.prepend(42);
            assert!(*brr_arr.get(0) == 42);
            let mut my_arr: Brr<i32> = Brr::new();
            my_arr.fill(vec![1; 1000]);
            my_arr.set(512, 9);
        }

        {
            let mut brr_arr = Brr::new();
            brr_arr.fill(vec![1, 2, 3, 4, 5]);
            let brr_map = brr_arr.map(|x, i| x * 2 + i);
            assert!(*brr_map.get(0) == 2);
            assert!(*brr_map.get(1) == 5);
            assert!(*brr_map.get(2) == 8);
            assert!(*brr_map.get(3) == 11);
            assert!(*brr_map.get(4) == 14);
        }

        {
            let mut brr_arr = Brr::new();
            brr_arr.fill(vec![1, 2, 3, 4, 5]);
            let brr_filter = brr_arr.filter(|x, _i| x % 2 == 0);
            assert!(*brr_filter.get(0) == 2);
            assert!(*brr_filter.get(1) == 4);
        }

        {
            let mut brr_arr = Brr::new();
            brr_arr.fill(vec![1, 2, 3]).rotate(2, true);
            assert!(*brr_arr.get(0) == 2);
            assert!(*brr_arr.get(1) == 3);
            assert!(*brr_arr.get(2) == 1);
        }
        {
            let mut brr_arr = Brr::new();
            brr_arr.fill(vec![1, 2, 3]).rotate(2, false);
            assert!(*brr_arr.get(0) == 3);
            assert!(*brr_arr.get(1) == 1);
            assert!(*brr_arr.get(2) == 2);
        }
        {
            let mut brr_arr = Brr::new();
            brr_arr.fill(vec![1, 2, 3]).rotate(2 * 100, false);
            assert!(*brr_arr.get(0) == 3);
            assert!(*brr_arr.get(1) == 1);
            assert!(*brr_arr.get(2) == 2);
        }
        {
            let mut brr_arr = Brr::new();
            brr_arr.fill(vec![1, 2, 3]).rotate(0, false);
            assert!(*brr_arr.get(0) == 1);
            assert!(*brr_arr.get(1) == 2);
            assert!(*brr_arr.get(2) == 3);
        }
        {
            let mut brr_arr = Brr::new();
            brr_arr.fill(vec![1, 2, 3]).rotate(3, false).rotate(3, true);
            assert!(*brr_arr.get(0) == 1);
            assert!(*brr_arr.get(1) == 2);
            assert!(*brr_arr.get(2) == 3);
        }
    }
}
