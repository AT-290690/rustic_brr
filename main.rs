use std::fmt;
#[path = "src/brr.rs"] mod brr;
#[path = "src/test.rs"] mod test;
use crate::brr::Brr;
impl fmt::Debug for brr::Brr<i32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return f
            .debug_struct("Brr")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish();
    }
}

fn main() {
    let amount = 100000;
    let mut bench_arr: Brr<i32> = brr![1; amount];
    println!("binary array of size {:?}", bench_arr.length());
    let mid = ((bench_arr.length() / 2) as f64).floor() as usize;
    println!("set element at the middle {:?}", bench_arr.set(mid, 42));
    println!("get element from the middle {:?}", bench_arr.get(mid));
    let mut idx_bench: usize = 0;
    loop {
        bench_arr.head();
        idx_bench += 1;
        if idx_bench == amount {
            break;
        }
    }
    println!("perform {} removal operations at the start", amount);
    println!("{:?}", bench_arr);
    println!("{:?}", bench_arr.length());
    println!("{:?}", bench_arr.left.len() - 1);
    println!("{:?}", bench_arr.right.len());
    println!(
        "{:?}",
        vec![
            bench_arr.length(),
            bench_arr.left.len() - 1,
            bench_arr.right.len()
        ]
    );
    assert!(bench_arr.length() == 0 && bench_arr.left.len() - 1 == 0 && bench_arr.right.is_empty());

    let mut poor_vec = vec![1; amount];
   
    let mut idx_bench: usize = 0;
    println!("vector of size {}", poor_vec.len());
    loop {
        idx_bench += 1;
        poor_vec.remove(0);
        if idx_bench == amount {
            break;
        }
    }
    println!("{:?}", poor_vec);
    println!("{:?}", poor_vec.len());
}