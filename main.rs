mod ba;
use std::fmt;
impl fmt::Debug for ba::BinaryArray {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("BinaryArray")
     .field("left", &self.left)
     .field("right", &self.right)
     .finish()
  }
}
fn main() {
  let items = vec![1, 2, 3, 4, 5];
  let mut bin_arr = ba::make();
  ba::fill(&mut bin_arr, items);
  println!("{:?}", ba::offset_right(&bin_arr));
  println!("{:?}", ba::offset_left(&bin_arr));
  println!("{:?}", bin_arr);
  println!("{:?}", ba::get(&bin_arr, 0));
  println!("{:?}", ba::get(&bin_arr, 1));
  println!("{:?}", ba::get(&bin_arr, 2));
  println!("{:?}", ba::get(&bin_arr, 3));
  println!("{:?}", ba::get(&bin_arr, 4));
  println!("{:?}", ba::to_vec(&bin_arr));
  ba::head (&mut bin_arr);
  ba::tail (&mut bin_arr);
  println!("{:?}", bin_arr);
  println!("{:?}", ba::get(&bin_arr, 2));
  ba::set(&mut bin_arr, 2, 100);
  println!("{:?}", bin_arr);
  println!("{:?}", ba::get(&bin_arr, 2));
  println!("{:?}", ba::to_vec(&bin_arr));
  println!("{:?}", ba::get(&bin_arr, 0));
  println!("{:?}", ba::get(&bin_arr, 1));
  println!("{:?}", ba::get(&bin_arr, 2));
  ba::tail(&mut bin_arr);
  ba::tail(&mut bin_arr);
  ba::tail(&mut bin_arr);
  ba::tail(&mut bin_arr);
  println!("{:?}", bin_arr);
  ba::fill (&mut bin_arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
  println!("{:?}", bin_arr);
  println!("{:?}", ba::to_vec(&bin_arr));
  ba::head (&mut bin_arr);  
  ba::head (&mut bin_arr);
  ba::head (&mut bin_arr);
  ba::head (&mut bin_arr);
  ba::head (&mut bin_arr);
  ba::head (&mut bin_arr);
  println!("{:?}", bin_arr);
  println!("{:?}", ba::to_vec(&bin_arr));
  ba::fill (&mut bin_arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
  println!("{:?}", bin_arr);
  ba::tail (&mut bin_arr);  
  ba::tail (&mut bin_arr);
  ba::tail (&mut bin_arr);
  ba::tail (&mut bin_arr);
  ba::tail (&mut bin_arr);
  ba::tail (&mut bin_arr);
  ba::tail (&mut bin_arr);
  println!("{:?}", bin_arr);
  println!("{:?}", ba::to_vec(&bin_arr));
  println!("{:?}", ba::get(&bin_arr, 0));
  println!("{:?}", ba::get(&bin_arr, 1));
  println!("{:?}", ba::get(&bin_arr, 2));
  println!("{:?}", ba::get(&bin_arr, 3));
  println!("{:?}", ba::get(&bin_arr, 4));
  ba::prepend(&mut bin_arr, 42);
  println!("{:?}", bin_arr);
  println!("{:?}", ba::get(&bin_arr, 0));
  println!("{:?}", ba::to_vec(&bin_arr));
  
  let mut my_arr = ba::make();
  ba::fill (&mut my_arr, vec![1; 1000]);
  ba::set(&mut my_arr, 512, 9);
  print!("{:?}", ba::get(&my_arr, 512));

let mut bench_arr = ba::make();
let mut idx:usize = 0;
let amount = 300000;
loop {
  ba::append(&mut bench_arr, 1);
  idx += 1;
  if idx == amount {
    break
  }
}
ba::balance(&mut bench_arr);
println!("binary array of size {:?}", ba::length(&bench_arr));
let mid = ((ba::length(&bench_arr) / 2) as f64).floor() as usize;
println!("set element at the middle {:?}",ba::set(&mut bench_arr, mid, 42));
println!("get element from the middle {:?}",ba::get(&bench_arr, mid));
let mut idx_bench:usize = 0;
loop {
  ba::head(&mut bench_arr);
  idx_bench += 1;
  if idx_bench == amount {
    break
  }
}
println!("perform {} removal operations at the start", amount);
println!("{:?}", bench_arr);
println!("{:?}", ba::length(&bench_arr));
println!("{:?}", ba::offset_left(&bench_arr));
println!("{:?}", ba::offset_right(&bench_arr));

idx_bench = 0;
let mut poor_vec = Vec::new();
loop {
  poor_vec.push(1);
  idx_bench += 1;
  if idx_bench == amount {
    break
  }
}
let mut idx_bench:usize = 0;
println!("vector of size {}", poor_vec.len());
loop {
  idx_bench += 1;
  if poor_vec.len() > 0{
    poor_vec.remove(0);
  }
  if idx_bench == amount {
    break
  }
}
println!("{:?}", poor_vec);
println!("{:?}", poor_vec.len());
}