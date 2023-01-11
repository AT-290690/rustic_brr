mod brr;

use std::fmt;

use crate::brr::Brr;
impl fmt::Debug for brr::Brr<i32> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Brr")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}
 
fn main() {
    let items = vec![1, 2, 3, 4, 5];
    let mut brr_arr = Brr::new();
    brr_arr.fill(items);
    println!("{:?}", brr_arr.right.len());
    println!("{:?}", brr_arr.left.len() - 1);
    println!("{:?}", brr_arr);
    assert!(*brr_arr.get(0) == 1);
    assert!(*brr_arr.get(1) == 2);
    assert!(*brr_arr.get(2) == 3);
    assert!(*brr_arr.get(3) == 4);
    println!("{:?}", *brr_arr.get(0));
    println!("{:?}", *brr_arr.get(1));
    println!("{:?}", *brr_arr.get(2));
    println!("{:?}", *brr_arr.get(3));
    println!("{:?}", *brr_arr.get(4));
    println!("{:?}", brr_arr.to_vec());
    brr_arr.head();
    brr_arr.tail();
    println!("{:?}", brr_arr);
    println!("{:?}", *brr_arr.get(2));
    assert!(*brr_arr.get(2) == 4);
    brr_arr.set(2, 100);
    println!("{:?}", brr_arr);
    println!("{:?}", *brr_arr.get(2));
    assert!(*brr_arr.get(2) == 100);
    println!("{:?}", brr_arr.to_vec());
    println!("{:?}", *brr_arr.get(0));
    println!("{:?}", *brr_arr.get(1));
    println!("{:?}", *brr_arr.get(2));
    brr_arr.tail();
    brr_arr.tail();
    brr_arr.tail();
    brr_arr.tail();
    println!("{:?}", brr_arr);
    brr_arr.fill(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    println!("{:?}", brr_arr);
    println!("{:?}", brr_arr.to_vec());
    brr_arr.head();
    brr_arr.head();
    brr_arr.head();
    brr_arr.head();
    brr_arr.head();
    brr_arr.head();
    println!("{:?}", brr_arr);
    println!("{:?}", brr_arr.to_vec());
    brr_arr.fill(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    println!("{:?}", brr_arr);
    brr_arr.tail();
    brr_arr.tail();
    brr_arr.tail();
    brr_arr.tail();
    brr_arr.tail();
    brr_arr.tail();
    brr_arr.tail();
    println!("{:?}", brr_arr);
    println!("{:?}", brr_arr.to_vec());
    println!("{:?}", *brr_arr.get(0));
    println!("{:?}", *brr_arr.get(1));
    brr_arr.prepend(42);
    println!("{:?}", brr_arr);
    println!("{:?}", *brr_arr.get(0));
    assert!(*brr_arr.get(0) == 42);
    println!("{:?}", brr_arr.to_vec());

    let mut my_arr: Brr<i32> = Brr::new();
    my_arr.fill(vec![1; 1000]);
    my_arr.set(512, 9);
    print!("{:?}", my_arr.get(512));

    let mut bench_arr: Brr<i32> = Brr::new();
    let mut idx: usize = 0;
    let amount = 100000;
    loop {
        bench_arr.append( 1);
        idx += 1;
        if idx == amount {
            break;
        }
    }
    bench_arr.balance();
    println!("binary array of size {:?}", bench_arr.length());
    let mid = ((bench_arr.length() / 2) as f64).floor() as usize;
    println!(
        "set element at the middle {:?}",
        bench_arr.set(mid, 42)
    );
    println!(
        "get element from the middle {:?}",
        bench_arr.get(mid)
    );
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
    println!("{:?}", brr_arr.length());
    println!("{:?}", bench_arr.left.len() - 1);
    println!("{:?}", bench_arr.right.len());
    assert!(
        bench_arr.length() == 0
            && bench_arr.left.len() - 1 == 0
            && bench_arr.right.len() == 0
    );

    idx_bench = 0;
    let mut poor_vec = Vec::new();
    loop {
        poor_vec.push(1);
        idx_bench += 1;
        if idx_bench == amount {
            break;
        }
    }
    let mut idx_bench: usize = 0;
    println!("vector of size {}", poor_vec.len());
    loop {
        idx_bench += 1;
        if poor_vec.len() > 0 {
            poor_vec.remove(0);
        }
        if idx_bench == amount {
            break;
        }
    }
    println!("{:?}", poor_vec);
    println!("{:?}", poor_vec.len());
}