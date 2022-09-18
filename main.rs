mod binary_array;
use binary_array as Barr;

use std::fmt;
impl fmt::Debug for Barr::BinaryArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BinaryArray")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}
fn main() {
    let items = vec![1, 2, 3, 4, 5];
    let mut bin_arr = Barr::new();
    Barr::fill(&mut bin_arr, items);
    println!("{:?}", bin_arr.right.len());
    println!("{:?}", bin_arr.left.len() - 1);
    println!("{:?}", bin_arr);
    assert!(Barr::get(&bin_arr, 0) == 1);
    assert!(Barr::get(&bin_arr, 1) == 2);
    assert!(Barr::get(&bin_arr, 2) == 3);
    assert!(Barr::get(&bin_arr, 3) == 4);
    println!("{:?}", Barr::get(&bin_arr, 0));
    println!("{:?}", Barr::get(&bin_arr, 1));
    println!("{:?}", Barr::get(&bin_arr, 2));
    println!("{:?}", Barr::get(&bin_arr, 3));
    println!("{:?}", Barr::get(&bin_arr, 4));
    println!("{:?}", Barr::to_vec(&bin_arr));
    Barr::head(&mut bin_arr);
    Barr::tail(&mut bin_arr);
    println!("{:?}", bin_arr);
    println!("{:?}", Barr::get(&bin_arr, 2));
    assert!(Barr::get(&bin_arr, 2) == 4);
    Barr::set(&mut bin_arr, 2, 100);
    println!("{:?}", bin_arr);
    println!("{:?}", Barr::get(&bin_arr, 2));
    assert!(Barr::get(&bin_arr, 2) == 100);
    println!("{:?}", Barr::to_vec(&bin_arr));
    println!("{:?}", Barr::get(&bin_arr, 0));
    println!("{:?}", Barr::get(&bin_arr, 1));
    println!("{:?}", Barr::get(&bin_arr, 2));
    Barr::tail(&mut bin_arr);
    Barr::tail(&mut bin_arr);
    Barr::tail(&mut bin_arr);
    Barr::tail(&mut bin_arr);
    println!("{:?}", bin_arr);
    Barr::fill(&mut bin_arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    println!("{:?}", bin_arr);
    println!("{:?}", Barr::to_vec(&bin_arr));
    Barr::head(&mut bin_arr);
    Barr::head(&mut bin_arr);
    Barr::head(&mut bin_arr);
    Barr::head(&mut bin_arr);
    Barr::head(&mut bin_arr);
    Barr::head(&mut bin_arr);
    println!("{:?}", bin_arr);
    println!("{:?}", Barr::to_vec(&bin_arr));
    Barr::fill(&mut bin_arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    println!("{:?}", bin_arr);
    Barr::tail(&mut bin_arr);
    Barr::tail(&mut bin_arr);
    Barr::tail(&mut bin_arr);
    Barr::tail(&mut bin_arr);
    Barr::tail(&mut bin_arr);
    Barr::tail(&mut bin_arr);
    Barr::tail(&mut bin_arr);
    println!("{:?}", bin_arr);
    println!("{:?}", Barr::to_vec(&bin_arr));
    println!("{:?}", Barr::get(&bin_arr, 0));
    println!("{:?}", Barr::get(&bin_arr, 1));
    Barr::prepend(&mut bin_arr, 42);
    println!("{:?}", bin_arr);
    println!("{:?}", Barr::get(&bin_arr, 0));
    assert!(Barr::get(&bin_arr, 0) == 42);
    println!("{:?}", Barr::to_vec(&bin_arr));

    let mut my_arr = Barr::new();
    Barr::fill(&mut my_arr, vec![1; 1000]);
    Barr::set(&mut my_arr, 512, 9);
    print!("{:?}", Barr::get(&my_arr, 512));

    let mut bench_arr = Barr::new();
    let mut idx: usize = 0;
    let amount = 100000;
    loop {
        Barr::append(&mut bench_arr, 1);
        idx += 1;
        if idx == amount {
            break;
        }
    }
    Barr::balance(&mut bench_arr);
    println!("binary array of size {:?}", Barr::length(&bench_arr));
    let mid = ((Barr::length(&bench_arr) / 2) as f64).floor() as usize;
    println!(
        "set element at the middle {:?}",
        Barr::set(&mut bench_arr, mid, 42)
    );
    println!(
        "get element from the middle {:?}",
        Barr::get(&bench_arr, mid)
    );
    let mut idx_bench: usize = 0;
    loop {
        Barr::head(&mut bench_arr);
        idx_bench += 1;
        if idx_bench == amount {
            break;
        }
    }
    println!("perform {} removal operations at the start", amount);
    println!("{:?}", bench_arr);
    println!("{:?}", Barr::length(&bench_arr));
    println!("{:?}", bench_arr.left.len() - 1);
    println!("{:?}", bench_arr.right.len());
    assert!(
        Barr::length(&bench_arr) == 0
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
