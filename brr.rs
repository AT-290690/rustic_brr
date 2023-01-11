type Size = i32;
#[derive(Default, Clone)]
pub struct Brr<T> {
    pub left: Vec<T>,
    pub right: Vec<T>,
}

impl<T:Clone + Default> Brr<T> {
    pub fn new() -> Self {
        return Brr {
            left:vec![<_>::default(); 1],
            right: Vec::new(),
        };
    }

pub fn clear(&mut self) {
    self.left.clear();
    self.right.clear();
    self.left.push(<_>::default())
}

pub fn length(&self) -> usize {
    return self.left.len() + self.right.len() - 1
}

pub fn get(&self, idx: usize) -> &T {
    let offset_index = idx as Size + (self.left.len() - 1) as Size * -1;
    if offset_index >= 0 {
        let index = offset_index as usize;
        return &self.right[index];
    } else {
        let index = (offset_index * -1) as usize;
        return &self.left[index];
    }
}

pub fn set(&mut self, idx: usize, val: T) {
    let len = self.length();
    if idx < len {
        let offset_index = idx as Size + (self.left.len() - 1) as Size * -1;
        if offset_index >= 0 {
            let index = offset_index as usize;
            self.right[index] = val;
        } else {
            let index = (offset_index * -1) as usize;
            self.left[index] = val;
        }
    } 
}

pub fn append(&mut self, item: T) {
    self.right.push(item);
}

pub fn prepend(&mut self, item: T) {
    self.left.push(item);
}

pub fn tail(&mut self) {
    {
        let len = self.length();
        if len <= 0 {
           return;
        } if len == 1 {
            self.clear()
        } else if self.left.len() > 0 {
            self.left.pop();
        }
    };
    if self.left.len() - 1 == 0 {
         self.balance();
    }
}

pub fn head(&mut self) {
    {
        let len = self.length();
        if len <= 0 {
        return;
        }
        if len == 1 {
            self.clear()
        } else if self.right.len() > 0 {
            self.right.pop();
        }
    };
    if self.right.len() == 0 {
        self.balance();
    }
}

pub fn to_vec(&self) -> Vec<T> {
    let mut out: Vec<T> = Vec::new();
    let len = self.length();
    if len == 0 {
        return out;
    }
    for idx in 0..len {
        out.push(self.get(idx).clone())
    }
    return out;
}

pub fn balance(&mut self) {
    let items = self.to_vec();
    self.fill(items);
}

pub fn fill(&mut self, items: Vec<T>) {
    self.clear();
    let len = items.len();
    let half = ((len / 2) as f64).floor() as usize;
    if half == 0 {
      return;
    }
    let mut left = half - 1;
    let mut right = half;

    loop {
        {
            let item = items[left].clone();
            self.left.push(item)
        };
        if left == 0 {
            break;
        } else {
            left -= 1
        }
    }
    
    loop {
        {
            let item = items[right].clone();
            self.right.push(item)
        };
        right += 1;
        if right == len {
            break;
        }
    }
}
}