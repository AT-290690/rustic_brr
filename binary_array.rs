pub struct BinaryArray {
  pub left: Vec<i32>,
  pub right: Vec<i32>,
}
pub fn clear (entity: &mut BinaryArray) {
  entity.left.clear();
  entity.right.clear();
  entity.left.push(-1);
}
pub fn length (entity: &BinaryArray) -> u128 {
  return entity.left.len() as u128 + entity.right.len() as u128 - 1;
}
pub fn offset_left (entity: &BinaryArray) -> i32{
  return (entity.left.len() as i32 - 1) * -1;
}
pub fn offset_rigth (entity: &BinaryArray) -> i32{
  return entity.right.len() as i32;
}
fn check_bounds_and_get (idx: usize, entity: &BinaryArray) -> Option<i32> {
  let len = length(entity);
  if idx as u128 >= len  { return None }
  else {
    let offset_index = idx as i32 + offset_left(entity);
    let index = if offset_index < 0 { offset_index * -1 } else { offset_index } as usize;
    if offset_index >= 0 { 
      return Some (entity.right[index])
    } else { 
      return Some (entity.left[index])
    } 
  }
}
fn check_bounds_and_set (idx: usize, entity: &mut BinaryArray, val:i32) -> Option<i32> {
  let len = length(entity);
  if idx as u128 > len  { return None }
  else {
    let offset_index = idx as i32 + offset_left(entity);
    let index = if offset_index < 0 { offset_index * -1 } else { offset_index } as usize;

    if offset_index >= 0 { 
      entity.right[index] = val;
      return Some (entity.right[index])
    } else { 
      entity.left[index] = val;
      return Some (entity.left[index])
    } 
  }
}
pub fn get (entity: &BinaryArray, idx: usize) -> i32 {
  match check_bounds_and_get(idx, entity) {
    None => -1,
    Some(item) => item,
  }
}
pub fn set (entity: &mut BinaryArray, idx: usize, val: i32) {
  match check_bounds_and_set(idx, entity, val) {
    None => -1,
    Some(item) => item,
  };
}
fn add_to_left (entity: &mut BinaryArray, item: i32) {
  entity.left.push(item);
}
fn add_to_right (entity: &mut BinaryArray, item: i32)  {
  entity.right.push(item);
}
fn remove_from_left (entity: &mut BinaryArray) {
  let len = length(&entity);
  if len > 0 {
    if len == 1 { clear (entity) } 
    else if entity.left.len() > 0 { 
      entity.left.pop();
    }
  }
}
fn remove_from_right (entity: &mut BinaryArray) {
  let len = length(&entity);
  if len > 0 {
    if len == 1 { clear (entity) } 
    else if entity.right.len() > 0 { 
      entity.right.pop();
    }
  }
}
pub fn to_vec (entity: &BinaryArray) -> Vec<i32> {
  let mut out:Vec<i32> = Vec::new();
  let len = length(entity);
  if len == 0 { return out }
  for idx in 0..len {
    out.push(get(entity, idx as usize));
  }
  return out;
}
pub fn append (entity: &mut BinaryArray, item: i32) -> &mut BinaryArray {
  add_to_right(entity, item);
  return entity;
}
pub fn prepend (entity: &mut BinaryArray, item: i32)-> &mut BinaryArray {
  add_to_left (entity, item);
  return entity;
}
pub fn head (entity: &mut BinaryArray, ) -> &mut BinaryArray {
  remove_from_right(entity);
  if offset_rigth(entity) == 0  { return balance(entity) }
  return entity;
}
pub fn tail (entity: &mut BinaryArray, ) -> &mut BinaryArray {
  remove_from_left(entity);  
  if offset_left(entity) == 0  { return balance(entity) }
  return entity;
}
pub fn create_binary_array () -> BinaryArray {
  return BinaryArray {
    left: vec![0],
    right: Vec::new()
  };
}
pub fn balance (entity: &mut BinaryArray) -> &mut BinaryArray {
  let items = to_vec(entity);
  clear(entity);
  return fill(entity, items);
}
pub fn fill(entity: &mut BinaryArray, items:Vec<i32>) -> &mut BinaryArray {
  let len  = items.len();
  let half = ((len / 2) as f32).floor() as usize;
  if half == 0 { return entity }
  let mut left = half - 1;
  let mut right = half;
  loop {
    add_to_left(entity, items[left]);
    if left == 0 {
      break
    } else {
      left -= 1;
    }
  }
  loop {
    add_to_right(entity, items[right]);
    right += 1;
    if right == len {
      break entity
    }
  }
}