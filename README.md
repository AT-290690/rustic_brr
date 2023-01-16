# Rustic Brr

<p align="center">
<img width="200" src="./logo.svg"/>
</p>

## Instant array operations

insert O(1)  
remove O(1)  
access O(1)  
memory O(N)

This is Rust version of my JavaScript https://github.com/AT-290690/brrr 

There is a simple benchmark vs vector in main.rs
(bump up N - Brr goes first and vec second. Diff is noticible the bigger N)

cargo run

cargo test 

Structure

```rust
let brray = brr![-2, -1, 0, 1, 2, 3, 4];
{
  left: [0, -1, -2], // first item in left is unreachable and just there as an offset - defaults to 0 for i32
  right: [0, 1, 2, 3, 4]
}
brray.to_vec() => [-2, -1, 0, 1, 2, 3, 4]
```

Indexing is guaranteed without the need of reordering thanks to simple arithmetics:

![1_CJHj_FVbZ61iWSIevvMrsw](https://user-images.githubusercontent.com/88512646/189848001-5274f5bf-200d-46e3-80df-25c5718bfc4a.gif)

```rust
 -  [0, 3, 2, 1, 0] // left
 +  [4, 5, 6, 7, 8] // right

[0] -> 0 - 4 = -4 => 0 // -
[1] -> 1 - 4 = -3 => 1 // -
[2] -> 2 - 4 = -2 => 2 // -
[3] -> 3 - 4 = -1 -> 3 // -
[4] -> 4 - 4 =  0 => 4 // +
[5] -> 5 - 4 =  1 => 5 // +
[6] -> 6 - 4 =  2 => 6 // +
[7] -> 7 - 4 =  3 => 7 // +
[8] -> 8 - 4 =  4 => 8 // +

[0, 1, 2, 3, 4, 5, 6, 7, 8]
```
How it works? It never adds or removes from the front. 
Uses 2 vectors and only pushes and pops. 
And keeps constant indexing using the above logic.

Very, very, very rarly it balances the two vectors (Once on every COMPLETE removals of 1 branch vector or... never).

Original vector shifts and unshifts for operations at the start making insertions/deletions O(N) 
Brr has them in O(1) + keeping the access O(1).
