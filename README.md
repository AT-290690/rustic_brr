# Rustic Brr

<p align="center">
<img width="50" src="./logo.svg"/>
</p>

## Instant array operations

insert O(1)  
remove O(1)  
access O(1)  
memory O(N)

There is a simple benchmark vs vector in main.rs
cargo run
cargo test

Structure

```rust
let mut brray = Brr::new();
brray.prepend(-1).prepend(-2).append(100).append(1).append(2).append(3).append(4);
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
