type Item = i8;
type UnsignedSize = u32;
type SignedSize = i32;

fn empty () -> Item { -1 }
pub struct BinaryArray {
  pub left: Vec<Item>,
  pub right: Vec<Item>,
}
pub fn clear (entity: &mut BinaryArray) {
  entity.left.clear();
  entity.right.clear();
  entity.left.push(empty());
}
pub fn length (entity: &BinaryArray) -> usize { entity.left.len() + entity.right.len() - 1 }
pub fn offset_left (entity: &BinaryArray) -> usize { entity.left.len() - 1 }
pub fn offset_right (entity: &BinaryArray) -> usize { entity.right.len() }
fn check_bounds_and_get (idx: usize, entity: &BinaryArray) -> Option<Item> {
  let len = length(entity);
  if idx as UnsignedSize >= len as UnsignedSize  { return None }
  else {
    let offset_index = idx as SignedSize + offset_left(entity) as SignedSize * -1;
    let index = if offset_index < 0 { offset_index * -1 } else { offset_index } as usize;
    if offset_index >= 0 { Some (entity.right[index]) } else { Some (entity.left[index]) } 
  }
}
fn check_bounds_and_set (idx: usize, entity: &mut BinaryArray, val:Item) -> Option<Item> {
  let len = length(entity);
  if idx as UnsignedSize > len as UnsignedSize { return None }
  else {
    let offset_index = idx as SignedSize + offset_left(entity) as SignedSize * -1;
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
pub fn get (entity: &BinaryArray, idx: usize) -> Item {
  match check_bounds_and_get(idx, entity) {
    None => empty(),
    Some(item) => item
  }
}
pub fn set (entity: &mut BinaryArray, idx: usize, val: Item) -> Item {
  match check_bounds_and_set(idx, entity, val) {
    None => empty(),
    Some (item) => item
  }
}
fn add_to_left (entity: &mut BinaryArray, item: Item) { entity.left.push(item) }
fn add_to_right (entity: &mut BinaryArray, item: Item) { entity.right.push(item) }
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
pub fn to_vec (entity: &BinaryArray) -> Vec<Item> {
  let mut out:Vec<Item> = Vec::new();
  let len = length(entity);
  if len == 0 { return out }
  for idx in 0..len { out.push(get(entity, idx as usize)) }
  return out;
}
pub fn append (entity: &mut BinaryArray, item: Item) -> &mut BinaryArray {
  add_to_right(entity, item);
  return entity;
}
pub fn prepend (entity: &mut BinaryArray, item: Item)-> &mut BinaryArray {
  add_to_left (entity, item);
  return entity;
}
pub fn head (entity: &mut BinaryArray) -> &mut BinaryArray {
  remove_from_right(entity);
  if offset_right(entity) == 0  { return balance(entity) }
  return entity;
}
pub fn tail (entity: &mut BinaryArray) -> &mut BinaryArray {
  remove_from_left(entity);  
  if offset_left(entity) == 0 { return balance(entity) }
  return entity;
}
pub fn make () -> BinaryArray {
  return BinaryArray {
    left: vec![-1],
    right: Vec::new()
  }
}
pub fn balance (entity: &mut BinaryArray) -> &mut BinaryArray {
  let items = to_vec(entity);
  clear(entity);
  return fill(entity, items);
}
pub fn fill (entity: &mut BinaryArray, items:Vec<Item>) -> &mut BinaryArray {
  let len = items.len();
  let half = ((len / 2) as f64).floor() as usize;
  if half == 0 { return entity }
  let mut left = half - 1;
  let mut right = half;
  loop {
    add_to_left(entity, items[left]);
    if left == 0 { break } else { left -= 1 }
  }
  loop {
    add_to_right(entity, items[right]);
    right += 1;
    if right == len { break entity }
  }
}