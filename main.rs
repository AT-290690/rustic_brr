mod binary_array;
use std::fmt;
impl fmt::Debug for binary_array::BinaryArray {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("BinaryArray")
     .field("left", &self.left)
     .field("right", &self.right)
     .finish()
  }
}
fn main() {
  let items = vec![1, 2, 3, 4, 5];
  let mut bin_arr = binary_array::create_binary_array();
  binary_array::fill(&mut bin_arr, items);
  println!("{:?}", binary_array::offset_right(&bin_arr));
  println!("{:?}", binary_array::offset_left(&bin_arr));
  println!("{:?}", bin_arr);
  println!("{:?}", binary_array::get(&bin_arr, 0));
  println!("{:?}", binary_array::get(&bin_arr, 1));
  println!("{:?}", binary_array::get(&bin_arr, 2));
  println!("{:?}", binary_array::get(&bin_arr, 3));
  println!("{:?}", binary_array::get(&bin_arr, 4));
  println!("{:?}", binary_array::to_vec(&bin_arr));
  binary_array::head (&mut bin_arr);
  binary_array::tail (&mut bin_arr);
  println!("{:?}", bin_arr);
  println!("{:?}", binary_array::get(&bin_arr, 2));
  binary_array::set(&mut bin_arr, 2, 100);
  println!("{:?}", bin_arr);
  println!("{:?}", binary_array::get(&bin_arr, 2));
  println!("{:?}", binary_array::to_vec(&bin_arr));
  println!("{:?}", binary_array::get(&bin_arr, 0));
  println!("{:?}", binary_array::get(&bin_arr, 1));
  println!("{:?}", binary_array::get(&bin_arr, 2));
  binary_array::tail(&mut bin_arr);
  binary_array::tail(&mut bin_arr);
  binary_array::tail(&mut bin_arr);
  binary_array::tail(&mut bin_arr);
  println!("{:?}", bin_arr);
  binary_array::fill (&mut bin_arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
  println!("{:?}", bin_arr);
  println!("{:?}", binary_array::to_vec(&bin_arr));
  binary_array::head (&mut bin_arr);  
  binary_array::head (&mut bin_arr);
  binary_array::head (&mut bin_arr);
  binary_array::head (&mut bin_arr);
  binary_array::head (&mut bin_arr);
  binary_array::head (&mut bin_arr);
  println!("{:?}", bin_arr);
  println!("{:?}", binary_array::to_vec(&bin_arr));
  binary_array::fill (&mut bin_arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
  println!("{:?}", bin_arr);
  binary_array::tail (&mut bin_arr);  
  binary_array::tail (&mut bin_arr);
  binary_array::tail (&mut bin_arr);
  binary_array::tail (&mut bin_arr);
  binary_array::tail (&mut bin_arr);
  binary_array::tail (&mut bin_arr);
  binary_array::tail (&mut bin_arr);
  println!("{:?}", bin_arr);
  println!("{:?}", binary_array::to_vec(&bin_arr));
  println!("{:?}", binary_array::get(&bin_arr, 0));
  println!("{:?}", binary_array::get(&bin_arr, 1));
  println!("{:?}", binary_array::get(&bin_arr, 2));
  println!("{:?}", binary_array::get(&bin_arr, 3));
  println!("{:?}", binary_array::get(&bin_arr, 4));
  binary_array::prepend(&mut bin_arr, 42);
  println!("{:?}", bin_arr);
  println!("{:?}", binary_array::get(&bin_arr, 0));
  println!("{:?}", binary_array::to_vec(&bin_arr));
  
// binary_array
let mut bench_arr = binary_array::create_binary_array();
let mut idx:usize = 0;
let amount = 300000;
loop {
  binary_array::append(&mut bench_arr, 1);
  idx += 1;
  if idx == amount {
    break
  }
}
binary_array::balance(&mut bench_arr);
println!("binary array of size {:?}", binary_array::length(&bench_arr));
let mid = ((binary_array::length(&bench_arr) / 2) as f64).floor() as usize;
println!("set element at the middle {:?}",binary_array::set(&mut bench_arr, mid, 42));
println!("get element from the middle {:?}",binary_array::get(&bench_arr, mid));
let mut idx_bench:usize = 0;
loop {
  binary_array::head(&mut bench_arr);
  idx_bench += 1;
  if idx_bench == amount {
    break
  }
}
println!("perform {} removal operations at the start", amount);
println!("{:?}", bench_arr);
println!("{:?}", binary_array::length(&bench_arr));
println!("{:?}", binary_array::offset_left(&bench_arr));
println!("{:?}", binary_array::offset_right(&bench_arr));

// vector

// idx_bench = 0;
// let mut poor_vec = Vec::new();
// loop {
//   poor_vec.push(1);
//   idx_bench += 1;
//   if idx_bench == amount {
//     break
//   }
// }
// let mut idx_bench:usize = 0;
// println!("vector of size {}", poor_vec.len());
// loop {
//   idx_bench += 1;
//   if poor_vec.len() > 0{
//     poor_vec.remove(0);
//   }
//   if idx_bench == amount {
//     break
//   }
// }
// println!("{:?}", poor_vec);
// println!("{:?}", poor_vec.len());
}