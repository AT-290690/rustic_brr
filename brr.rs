#![allow(dead_code)]
type Size = i32;
#[derive(Default, Clone)]
pub struct Brr<T> {
    pub left: Vec<T>,
    pub right: Vec<T>,
}
impl<T: Clone + Default> Brr<T> {
    pub fn new() -> Self {
        return Brr {
            left: vec![T::default(); 1],
            right: Vec::new(),
        };
    }
    fn offset_left(&self) -> Size {
        return (self.left.len() - 1) as Size * -1;
    }
    fn offset_right(&self) -> Size {
        return self.right.len() as Size;
    }
    pub fn is_empty(self) -> bool {
        return self.length() == 0;
    }
    pub fn transform(
        &mut self,
        callback: fn(acc: Brr<T>, item: &T, index: usize) -> Brr<T>,
    ) -> Brr<T> {
        let mut result: Brr<T> = Brr::new();
        result.from_vec(vec![self.first().unwrap().clone()]);
        for index in 1..self.length() {
            let current = self.get(index).unwrap();
            result = callback(result, current, index);
        }
        return result;
    }
    pub fn slice(&self, mut start: i32, mut end: i32) -> Brr<T> {
        let mut slice = Brr::new();
        let len = self.length() as i32;
        if end == 0 {
            end = len
        }
        if start > len - 1 {
            start = len - 1
        }
        let slice_len = end - start;
        let half = ((slice_len / 2) as f64).floor() as usize;
        for i in (0..half).rev() {
            slice.prepend(self.get(start as usize + i).unwrap().clone());
        }
        for i in half..slice_len as usize {
            slice.append(self.get(start as usize + i).unwrap().clone());
        }
        return slice;
    }
    pub fn clear(&mut self) -> &mut Self {
        self.left.clear();
        self.right.clear();
        self.left.push(T::default());
        return self;
    }
    pub fn length(&self) -> usize {
        return self.left.len() + self.right.len() - 1;
    }
    pub fn get(&self, idx: usize) -> Option<&T> {
        if self.length() == 0 {
            return None;
        } else {
            let offset_index = idx as Size + self.offset_left();
            if offset_index >= 0 {
                let index = offset_index as usize;
                return Some(&self.right[index]);
            } else {
                let index = (offset_index * -1) as usize;
                return Some(&self.left[index]);
            }
        }
    }
    pub fn set(&mut self, idx: usize, val: T) {
        let len = self.length();
        if idx < len {
            let offset_index = idx as Size + self.offset_left();
            if offset_index >= 0 {
                let index = offset_index as usize;
                self.right[index] = val;
            } else {
                let index = (offset_index * -1) as usize;
                self.left[index] = val;
            }
        }
    }
    pub fn append(&mut self, item: T) -> &mut Self {
        self.right.push(item);
        return self;
    }
    pub fn prepend(&mut self, item: T) -> &mut Self {
        self.left.push(item);
        return self;
    }
    pub fn tail(&mut self) -> &mut Self {
        let len = self.length();
        if len == 0 {
            return self;
        }
        if len == 1 {
            return self.clear();
        }
        if self.left.len() > 1 {
            self.left.pop();
            return self;
        } else {
            self.balance();
            self.left.pop();
            return self;
        }
    }
    pub fn head(&mut self) -> &mut Self {
        let len = self.length();
        if len == 0 {
            return self;
        }
        if len == 1 {
            return self.clear();
        }
        if self.right.len() > 0 {
            self.right.pop();
            return self;
        } else {
            self.balance();
            self.right.pop();
            return self;
        }
    }
    pub fn to_vec(&self) -> Vec<T> {
        let mut out: Vec<T> = Vec::new();
        let len = self.length();
        for idx in 0..len {
            out.push(self.get(idx).unwrap().clone())
        }
        return out;
    }
    pub fn balance(&mut self) -> &mut Self {
        let items = self.to_vec();
        return self.from_vec(items);
    }
    pub fn from_vec(&mut self, items: Vec<T>) -> &mut Self {
        self.clear();
        let len = items.len();
        if len == 1 {
            self.right.push(items[0].clone());
            return self;
        }
        let half = ((len / 2) as f64).floor() as usize;
        let mut left = half - 1;
        let mut right = half;

        loop {
            self.left.push(items[left].clone());
            if left == 0 {
                break;
            } else {
                left = left - 1
            }
        }
        loop {
            self.right.push(items[right].clone());
            right = right + 1;
            if right == len {
                break;
            }
        }
        return self;
    }
    pub fn first(&self) -> Option<&T> {
        return self.get(0);
    }
    pub fn last(&self) -> Option<&T> {
        return self.get(self.length() - 1);
    }
    pub fn map(&mut self, callback: fn(item: &T, index: usize) -> T) -> Brr<T> {
        let mut out: Brr<T> = Brr::new();
        let len = self.length();
        let half = ((len / 2) as f64).floor() as usize;
        let mut left = half - 1;
        let mut right = half;
        loop {
            out.left.push(callback(self.get(left).unwrap(), left));
            if left == 0 {
                break;
            } else {
                left = left - 1
            }
        }
        loop {
            out.right.push(callback(self.get(right).unwrap(), right));
            right = right + 1;
            if right == len {
                break;
            }
        }
        return out;
    }
    pub fn filter(&mut self, callback: fn(item: &T, index: usize) -> bool) -> Brr<T> {
        let mut out = Brr::new();
        for index in 0..self.length() {
            let current = self.get(index).unwrap();
            if callback(current, index) {
                out.append(current.clone());
            }
        }
        out.balance();
        return out;
    }
    pub fn rotate_left(&mut self, n: usize) -> &mut Self {
        let mut rot = n % self.length();
        loop {
            if rot == 0 {
                break;
            }
            if self.left.len() - 1 == 0 {
                self.balance();
            }
            self.append(self.first().unwrap().clone());
            self.tail();
            rot = rot - 1
        }
        return self;
    }
    pub fn rotate_right(&mut self, n: usize) -> &mut Self {
        let mut rot = n % self.length();
        loop {
            if rot == 0 {
                break;
            }
            if self.right.len() == 0 {
                self.balance();
            }
            self.prepend(self.last().unwrap().clone());
            self.head();
            rot = rot - 1
        }
        return self;
    }
    pub fn cut(&mut self) -> T {
        let last = self.last().unwrap().clone();
        self.head();
        return last;
    }
    pub fn chop(&mut self) -> T {
        let first = self.first().unwrap().clone();
        self.tail();
        return first;
    }
    pub fn insert(&mut self, idx: usize, value: T) -> &Self {
        let length  = self.length();
        if length < idx {
            return self;
        } else if length == idx {
            self.append(value);
            return self;
        } else if idx == 0 {
            self.prepend(value);
            return self;
        }
        let offset_index = idx as Size + self.offset_left();
        if offset_index > 0 {
            let len = length - idx;
            self.rotate_right(len);
            self.append(value);
            for _ in 0..(len - 1) {
                let item = self.chop();
                self.append(item);
            }
        } else {
            self.rotate_left(idx);
            self.prepend(value);
            for _ in 0..idx {
                let item = self.cut();
                self.prepend(item);
            }
        }
        return self;
    }
    pub fn insert_many(&mut self, idx: usize, values: Vec<T>) -> &Self {
        let length  = self.length();
        if length < idx {
            return self;
        } else if length == idx {
            for i in 0..values.len() {
                self.append(values[i].clone());
            }
            return self;
        } else if idx == 0 {
            for i in 0..values.len() {
                self.prepend(values[i].clone());
            }
            return self;
        }
        let offset_index = idx as Size + self.offset_left();
        if offset_index > 0 {
            let len = length - idx;
            self.rotate_right(len);
            for i in 0..values.len() {
                self.append(values[i].clone());
            }
            for _ in 0..len {
                let item = self.chop();
                self.append(item);
            }
        } else {
            self.rotate_left(idx);
            for i in 0..values.len() {
                self.prepend(values[i].clone());
            }
            for _ in 0..idx {
                let item = self.cut();
                self.prepend(item);
            }
        }
        return self;
    }
    pub fn remove(&mut self, idx: usize) -> &Self {
        let length  = self.length();
        if length < idx {
            return self
        } else if length == idx {
            self.head();
            return self;
        }
        let len = length - idx - 1;
        let offset_index = idx as Size + self.offset_left();
        let is_close_to_right = offset_index > 0;
        if is_close_to_right {
            self.rotate_right(len);
            self.head();
            for _ in 0..len {
                let item = self.chop();
                self.append(item);
            }
        } else {
            self.rotate_left(idx);
            self.tail();
            for _ in 0..idx {
                let item = self.cut();
                self.prepend(item);
            }
        }
        return self;
    }
    pub fn remove_many(&mut self, idx: usize, mut amount: usize) -> &Self {
        let length  = self.length();
        if length < idx + amount {
            return self;
        } else if length == idx + amount {
            self.head();
            return self;
        }
        let len = length - idx - 1;
        amount = std::cmp::min(len, amount);
        let offset_index = idx as Size + self.offset_left();
        let is_close_to_right = offset_index > 0;
        if is_close_to_right {
            self.rotate_right(len);
            for _ in 0..amount {
                self.head();
            }
            for _ in 0..len {
                let item = self.chop();
                self.append(item);
            }
        } else {
            self.rotate_left(idx);
            for _ in 0..amount {
                self.tail();
            }
            for _ in 0..idx {
                let item = self.cut();
                self.prepend(item);
            }
        }
        return self;
    }
}
