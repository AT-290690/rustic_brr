type Item = i8;
type SignedSize = i32;

/*  Empty / Null value for -0 index left[0] -> -1 */
fn empty() -> Item {
    -1
}

/*
  consists of 2 branches
  left -> items added to the start
  right -> items added to the end
*/
pub struct BinaryArray {
    pub left: Vec<Item>,
    pub right: Vec<Item>,
}
/*
  create new entity
  with initial state
*/
pub fn new() -> BinaryArray {
    return BinaryArray {
        left: vec![empty()],
        right: Vec::new(),
    };
}

/*
  empty the entity
  reset to initial state
*/
pub fn clear(entity: &mut BinaryArray) {
    entity.left.clear();
    entity.right.clear();
    entity.left.push(empty());
}

/* get the total amount of items */
pub fn length(entity: &BinaryArray) -> usize {
    entity.left.len() + entity.right.len() - 1
}

/* get total amout of items isnerted at the start */
pub fn offset_left(entity: &BinaryArray) -> usize {
    entity.left.len() - 1
}

/* get total amout of items isnerted at the end */
pub fn offset_right(entity: &BinaryArray) -> usize {
    entity.right.len()
}

/* get item at any index in O(1) */
pub fn get(entity: &BinaryArray, idx: usize) -> Item {
    let len = length(entity);
    if idx < len {
        let offset_index = idx as SignedSize + offset_left(entity) as SignedSize * -1;
        let index = if offset_index < 0 {
            offset_index * -1
        } else {
            offset_index
        } as usize;
        if offset_index >= 0 {
            return entity.right[index];
        } else {
            return entity.left[index];
        }
    } else {
        return empty();
    }
}

/* overwrite item at any index in O(1) */
pub fn set(entity: &mut BinaryArray, idx: usize, val: Item) -> Item {
    let len = length(entity);
    if idx < len {
        let offset_index = idx as SignedSize + offset_left(entity) as SignedSize * -1;
        let index = if offset_index < 0 {
            offset_index * -1
        } else {
            offset_index
        } as usize;
        if offset_index >= 0 {
            entity.right[index] = val;
            return val;
        } else {
            entity.left[index] = val;
            return val;
        }
    } else {
        return empty();
    }
}

/* insert item at from start in O(1) */
pub fn append(entity: &mut BinaryArray, item: Item) -> &mut BinaryArray {
    entity.right.push(item);
    return entity;
}

/* insert item at from start in O(1) */
pub fn prepend(entity: &mut BinaryArray, item: Item) -> &mut BinaryArray {
    entity.left.push(item);
    return entity;
}

/* pull it by the tail - remove item from end in O(1) armortized */
pub fn tail(entity: &mut BinaryArray) -> &mut BinaryArray {
    {
        let len = length(&entity);
        if len <= 0 {
            return entity;
        }
        if len == 1 {
            clear(entity)
        } else if entity.left.len() > 0 {
            entity.left.pop();
        }
    };
    if offset_left(entity) == 0 {
        return balance(entity);
    }
    return entity;
}

/* pull it by the head - remove item from start in O(1) armortized */
pub fn head(entity: &mut BinaryArray) -> &mut BinaryArray {
    {
        let len = length(&entity);
        if len <= 0 {
            return entity;
        }
        if len == 1 {
            clear(entity)
        } else if entity.right.len() > 0 {
            entity.right.pop();
        }
    };
    if offset_right(entity) == 0 {
        return balance(entity);
    }
    return entity;
}

/* convert to vec */
pub fn to_vec(entity: &BinaryArray) -> Vec<Item> {
    let mut out: Vec<Item> = Vec::new();
    let len = length(entity);
    if len == 0 {
        return out;
    }
    for idx in 0..len {
        out.push(get(entity, idx as usize))
    }
    return out;
}

/* balance O(N) */
pub fn balance(entity: &mut BinaryArray) -> &mut BinaryArray {
    let items = to_vec(entity);
    return fill(entity, items);
}

/*
  fill up the entity with starting values
  this will first clear the entity
*/
pub fn fill(entity: &mut BinaryArray, items: Vec<Item>) -> &mut BinaryArray {
    clear(entity);
    let len = items.len();
    let half = ((len / 2) as f64).floor() as usize;
    if half == 0 {
        return entity;
    }
    let mut left = half - 1;
    let mut right = half;

    loop {
        {
            let item = items[left];
            entity.left.push(item)
        };
        if left == 0 {
            break;
        } else {
            left -= 1
        }
    }

    loop {
        {
            let item = items[right];
            entity.right.push(item)
        };
        right += 1;
        if right == len {
            break entity;
        }
    }
}