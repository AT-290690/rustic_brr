#![allow(dead_code)]
use std::cmp::min;
type Size = i32;

#[derive(Default, Clone)]
pub struct Brr<T> {
    pub left: Vec<T>,
    pub right: Vec<T>,
}

impl<T: Clone + Default> Brr<T> {
    pub fn new() -> Self {
        return Brr {
            left: Vec::from([T::default()]),
            right: Vec::new(),
        };
    }
    pub fn make(self, items: Vec<T>) -> Brr<T> {
        let mut left_branch = Vec::from([T::default()]);
        let mut right_branch = Vec::new();
        let len = items.len();
        if len == 0 {
        } else if len == 1 {
            right_branch.push(items[0].clone());
        } else {
            let len = items.len();
            let half = (len as f64 * 0.5).floor() as usize;
            let mut left = half - 1;
            let mut right = half;

            loop {
                left_branch.push(items[left].clone());
                if left == 0 {
                    break;
                } else {
                    left -= 1
                }
            }
            loop {
                right_branch.push(items[right].clone());
                right += 1;
                if right == len {
                    break;
                }
            }
        }

        return Brr {
            left: left_branch,
            right: right_branch,
        };
    }
    fn offset_left(&self) -> Size {
        return -((self.left.len() - 1) as Size);
    }
    fn offset_right(&self) -> Size {
        return self.right.len() as Size;
    }
    pub fn is_empty(self) -> bool {
        return self.length() == 0;
    }
    /// Calls the specified callback function for all the elements in an array.
    /// The return value of the callback function is the accumulated result,
    /// and is provided as an argument in the next call to the callback function.
    ///
    /// callback_fn — A function that accepts 3 arguments (accumulator, value, index)
    ///
    /// The transform method calls the callback_fn function one time for each element in the array.
    /// The result is a new array
    pub fn transform<F: Fn(Brr<T>, &T, usize) -> Brr<T>>(&mut self, callback: F) -> Brr<T> {
        let mut result: Brr<T> = Brr::new();
        let length = self.length();
        if length == 0 {
            return result;
        }
        for index in 0..length {
            let current = self.get(index).unwrap();
            result = callback(result, current, index);
        }
        return result;
    }
    pub fn fold<B, F: Fn(B, &T, usize) -> B>(&self, mut result: B, callback: F) -> B {
        let length = self.length();
        if length == 0 {
            return result;
        }
        for index in 0..length {
            let current = self.get(index).unwrap();
            result = callback(result, current, index);
        }
        return result;
    }
    pub fn to<B, F: Fn(B, &T) -> B>(&self, mut result: B, callback: F) -> B {
        let length = self.length();
        if length == 0 {
            return result;
        }
        for index in 0..length {
            let current = self.get(index).unwrap();
            result = callback(result, current);
        }
        return result;
    }
    pub fn find<F: Fn(&T) -> bool>(&self, callback: F) -> Option<&T> {
        let length = self.length();
        for i in 0..length {
            match self.get(i) {
                Some(f) => {
                    if callback(f) {
                        return Some(f);
                    }
                }
                None => return None,
            }
        }
        return None;
    }
    pub fn find_index<F: Fn(&T) -> bool>(&self, callback: F) -> Option<usize> {
        let length = self.length();
        for i in 0..length {
            match self.get(i) {
                Some(f) => {
                    if callback(f) {
                        return Some(i);
                    }
                }
                None => return None,
            }
        }
        return None;
    }
    pub fn concat(&mut self, other: &Brr<T>) -> &mut Self {
        for i in 0..other.length() {
            self.append(other.get(i).unwrap().clone());
        }
        self.balance();
        return self;
    }
    /// Reverses the elements in an array in place.
    ///
    /// This method mutates the array and returns a reference to the same array.
    pub fn reverse(&mut self) -> &mut Self {
        let mut left = self.left.clone();
        let mut right = self.right.clone();
        right.insert(0, left.remove(0));
        self.left = right;
        self.right = left;
        return self;
    }
    pub fn some<F: Fn(&T) -> bool>(&mut self, callback: F) -> bool {
        for index in 0..self.length() {
            let current = self.get(index).unwrap();
            if callback(current) {
                return true;
            }
        }
        return false;
    }
    pub fn every<F: Fn(&T) -> bool>(&mut self, callback: F) -> bool {
        let length = self.length();
        for index in 0..length {
            let current = self.get(index).unwrap();
            if index == length || !callback(current) {
                return false;
            }
        }
        return true;
    }
    /// Returns a copy of a section of an array
    /// from start to end
    ///
    /// Accepts 2 arguments (start, end)
    ///
    /// This is exclusive of the element at the index 'end'.
    ///
    /// Returns a new array
    pub fn slice(&self, mut start: Size, mut end: Size) -> Brr<T> {
        let mut slice = Brr::new();
        let len = self.length() as Size;
        if end == 0 {
            end = len
        }
        if start > len - 1 {
            start = len - 1
        }
        let slice_len = end - start;
        let half = (slice_len as f64 * 0.5).floor() as usize;
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
        let length = self.length();
        if length == 0 || length <= idx {
            return None;
        } else {
            let offset_index = idx as Size + self.offset_left();
            if offset_index >= 0 {
                let index = offset_index as usize;
                return Some(&self.right[index]);
            } else {
                let index = (-offset_index) as usize;
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
                let index = (-offset_index) as usize;
                self.left[index] = val;
            }
        }
    }
    pub fn at(&self, rel_idx: Size) -> Option<&T> {
        let len = self.length();
        if len == 0 {
            return None;
        } else {
            let idx = if rel_idx < 0 {
                len as Size + rel_idx
            } else {
                rel_idx
            };
            let offset_index = idx + self.offset_left();
            if offset_index >= 0 {
                let index = offset_index as usize;
                return Some(&self.right[index]);
            } else {
                let index = (-offset_index) as usize;
                return Some(&self.left[index]);
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
        if !self.right.is_empty() {
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
        if len == 0 {
            return self;
        } else if len == 1 {
            self.right.push(items[0].clone());
            return self;
        }
        let half = (len as f64 * 0.5).floor() as usize;
        let mut left = half - 1;
        let mut right = half;

        loop {
            self.left.push(items[left].clone());
            if left == 0 {
                break;
            } else {
                left -= 1
            }
        }
        loop {
            self.right.push(items[right].clone());
            right += 1;
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
        let length = self.length();
        if length == 0 {
            return None;
        }
        return self.get(length - 1);
    }
    pub fn for_each<F: FnMut(&T, usize)>(&mut self, mut callback: F) -> &mut Self {
        let len = self.length();
        let half = (len as f64 * 0.5).floor() as usize;
        let mut left = half - 1;
        let mut right = half;
        loop {
            callback(self.get(left).unwrap(), left);
            if left == 0 {
                break;
            } else {
                left -= 1
            }
        }
        loop {
            callback(self.get(right).unwrap(), right);
            right += 1;
            if right == len {
                break;
            }
        }
        return self;
    }
    /// Calls a defined callback function on each element of an array,
    /// and returns an array that contains the results.
    ///
    /// callback_fn — A function that accepts 2 arguments (value, index)
    ///
    /// The result is a new array
    pub fn map<F: Fn(&T) -> T>(&mut self, callback: F) -> Brr<T> {
        let mut out: Brr<T> = Brr::new();
        let len = self.length();
        let half = (len as f64 * 0.5).floor() as usize;
        let mut left = half - 1;
        let mut right = half;
        loop {
            out.left.push(callback(self.get(left).unwrap()));
            if left == 0 {
                break;
            } else {
                left -= 1
            }
        }
        loop {
            out.right.push(callback(self.get(right).unwrap()));
            right += 1;
            if right == len {
                break;
            }
        }
        return out;
    }
    pub fn iterate<F: Fn(&T, usize) -> T>(&mut self, callback: F) -> Brr<T> {
        let mut out: Brr<T> = Brr::new();
        let len = self.length();
        let half = (len as f64 * 0.5).floor() as usize;
        let mut left = half - 1;
        let mut right = half;
        loop {
            out.left.push(callback(self.get(left).unwrap(), left));
            if left == 0 {
                break;
            } else {
                left -= 1
            }
        }
        loop {
            out.right.push(callback(self.get(right).unwrap(), right));
            right += 1;
            if right == len {
                break;
            }
        }
        return out;
    }
    /// Returns the elements of an array that meet the condition specified in a callback function.
    ///
    /// predicate — A function that accepts 2 arguments (value, index)
    ///
    /// The filter method calls the predicate function one time for each element in the array.
    ///
    /// The result is a new array
    pub fn filter<F: Fn(&T, usize) -> bool>(&mut self, callback: F) -> Brr<T> {
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
    pub fn select<F: Fn(&T) -> bool>(&mut self, callback: F) -> Brr<T> {
        let mut out = Brr::new();
        for index in 0..self.length() {
            let current = self.get(index).unwrap();
            if callback(current) {
                out.append(current.clone());
            }
        }
        out.balance();
        return out;
    }
    pub fn except<F: Fn(&T) -> bool>(&mut self, callback: F) -> Brr<T> {
        let mut out = Brr::new();
        for index in 0..self.length() {
            let current = self.get(index).unwrap();
            if !callback(current) {
                out.append(current.clone());
            }
        }
        out.balance();
        return out;
    }
    pub fn zip(self, other: Brr<T>) -> Brr<Brr<T>> {
        let mut output: Brr<Brr<T>> = Brr::new();
        for index in 0..self.length() {
            let mut out = Brr::new();
            out.prepend(self.get(index).unwrap().clone());
            out.append(other.get(index).unwrap().clone());
            output.append(out);
        }
        output.balance();
        return output;
    }
    pub fn adjacent_difference<F: Fn(&T, &T) -> T>(self, callback: F) -> Brr<T> {
        let len = self.length();
        if len == 1 {
            return self;
        };
        let mut result: Brr<T> = Brr::new();
        result.append(self.get(0).unwrap().clone());
        for i in 1..len {
            result.append(callback(self.get(i - 1).unwrap(), self.get(i).unwrap()));
        }
        result.balance();
        return result;
    }
    pub fn adjacent_find<F: Fn(&T, &T) -> bool>(self, callback: F) -> Option<T> {
        let len = self.length();
        if len == 1 {
            return None;
        };
        for i in 1..len {
            let found = self.get(i).unwrap();
            if callback(self.get(i - 1).unwrap(), found) {
                return Some(found.clone());
            }
        }
        return None;
    }
    pub fn adjacent_find_index<F: Fn(&T, &T) -> bool>(self, callback: F) -> Option<usize> {
        let len = self.length();
        for i in 1..len {
            if callback(self.get(i - 1).unwrap(), self.get(i).unwrap()) {
                return Some(i);
            }
        }
        return None;
    }
    pub fn swap(&mut self, i: usize, j: usize) -> &mut Self {
        let temp = self.get(i).unwrap().clone();
        self.set(i, self.get(j).unwrap().clone());
        self.set(j, temp.clone());
        return self;
    }
    pub fn swap_remove_right(&mut self, index: usize) -> &mut Self {
        let val = self.cut();
        self.set(index, val);
        return self;
    }
    pub fn swap_remove_left(&mut self, index: usize) -> &mut Self {
        let val = self.chop();
        self.set(index, val);
        return self;
    }
    pub fn count<F: Fn(&T) -> bool>(&mut self, callback: F) -> usize {
        let mut out = 0;
        for index in 0..self.length() {
            let current = self.get(index).unwrap();
            if callback(current) {
                out += 1;
            }
        }
        return out;
    }
    pub fn rotate(&mut self, dir: Size) -> &mut Self {
        match dir < 0 {
            true => {
                let n = -dir;
                self.rotate_left(n as usize);
            }
            false => {
                let n = dir;
                self.rotate_right(n as usize);
            }
        }
        return self;
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
            let first = self.chop();
            self.append(first);
            rot -= 1
        }
        return self;
    }
    pub fn rotate_right(&mut self, n: usize) -> &mut Self {
        let mut rot = n % self.length();
        loop {
            if rot == 0 {
                break;
            }
            if self.right.is_empty() {
                self.balance();
            }
            let last = self.cut();
            self.prepend(last);
            rot -= 1
        }
        return self;
    }
    pub fn cut(&mut self) -> T {
        match !self.right.is_empty() {
            true => {
                if self.right.len() > 0 {
                    return self.right.pop().unwrap();
                } else {
                    self.balance();
                    return self.right.pop().unwrap();
                }
            }
            false => return T::default(),
        }
    }
    pub fn chop(&mut self) -> T {
        match !self.left.is_empty() {
            true => {
                if self.left.len() > 1 {
                    return self.left.pop().unwrap();
                } else {
                    self.balance();
                    return self.left.pop().unwrap();
                }
            }
            false => T::default(),
        }
    }
    pub fn insert(&mut self, idx: usize, value: T) -> &Self {
        let length = self.length();
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
        let length = self.length();
        if length < idx {
            return self;
        } else if length == idx {
            for item in values {
                self.append(item);
            }
            return self;
        } else if idx == 0 {
            for item in values {
                self.prepend(item);
            }
            return self;
        }
        let offset_index = idx as Size + self.offset_left();
        if offset_index > 0 {
            let len = length - idx;
            self.rotate_right(len);
            for item in values {
                self.append(item);
            }
            for _ in 0..len {
                let item = self.chop();
                self.append(item);
            }
        } else {
            self.rotate_left(idx);
            for item in values {
                self.prepend(item);
            }
            for _ in 0..idx {
                let item = self.cut();
                self.prepend(item);
            }
        }
        return self;
    }
    pub fn remove(&mut self, idx: usize) -> &Self {
        let length = self.length();
        if length < idx {
            return self;
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
        let length = self.length();
        if length < idx + amount {
            return self;
        } else if length == idx + amount {
            self.head();
            return self;
        }
        let len = length - idx - 1;
        amount = min(len, amount);
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
    pub fn partition(&self, groups: usize) -> Brr<Brr<T>> {
        let mut result: Brr<Brr<T>> = Brr::new();
        let length = self.length();
        if length == 0 || groups == 0 {
            return result;
        }
        for index in 0..length {
            if index % groups == 0 {
                let mut part = Brr::new();
                let half = (groups as f64 * 0.5).round() as usize;
                if half == 0 {
                    return result;
                }
                for i in (0..half).rev() {
                    if let Some(current) = self.get(index + i) {
                        part.prepend(current.clone());
                    }
                }
                for i in half..groups {
                    if let Some(current) = self.get(index + i) {
                        part.append(current.clone());
                    }
                }
                result.append(part);
            }
        }
        result.balance();
        return result;
    }
    pub fn partition_if<F: Fn(&T, usize) -> bool>(&self, callback: F) -> Brr<Brr<T>> {
        let mut result: Brr<Brr<T>> = Brr::new();
        let mut left = Brr::new();
        let mut right = Brr::new();

        let length = self.length();
        if length == 0 {
            return result;
        }
        for index in 0..length {
            if let Some(current) = self.get(index) {
                if callback(current, index) {
                    left.append(current.clone());
                } else {
                    right.append(current.clone());
                }
            }
        }
        left.balance();
        right.balance();
        result.prepend(left);
        result.append(right);
        return result;
    }
}

impl<T: Clone + Default> FromIterator<T> for Brr<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut collection: Brr<T> = Brr::new();
        for item in iter {
            collection.append(item);
        }
        collection.balance();
        return collection;
    }
}

impl<T: Clone + Default> IntoIterator for Brr<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        return self.to_vec().into_iter();
    }
}

#[macro_export]
macro_rules! brr {
    ($arg:expr; $n:expr) => {
        brr::Brr::make(brr::Brr::new(), vec![$arg; $n])
    };
    ($($args:expr),*) => {
        brr::Brr::make(brr::Brr::new(), Vec::from([$($args),*]))
    };
    () => {
        brr::Brr::new()
    }
}
pub fn flat<T: Clone + Default>(brr: Brr<Brr<T>>) -> Brr<T> {
    let mut out: Vec<T> = Vec::new();
    for i in 0..brr.length() {
        out.extend(brr.get(i).unwrap().to_vec())
    }
    let mut result = Brr::new();
    result.from_vec(out);
    return result;
}
pub fn concat<T: Clone + Default>(arr: Vec<Brr<T>>) -> Brr<T> {
    let mut out = Brr::new();
    for i in 0..arr.len() {
        out.concat(arr.get(i).unwrap());
    }
    return out;
}
pub fn range(start: usize, end: usize) -> Brr<usize> {
    let mut out: Brr<usize> = Brr::new();
    for i in start..(end / 2) {
        out.prepend(i);
    }
    for i in (end / 2)..(end + 1) {
        out.append(i);
    }
    return out;
}
pub fn zip<A: Clone + Default, B: Clone + Default>(a: Brr<A>, b: Brr<B>) -> Brr<(A, B)> {
    let mut output: Brr<(A, B)> = Brr::new();
    let end = a.length();
    for i in 0..(end / 2) {
        output.prepend((a.get(i).unwrap().clone(), b.get(i).unwrap().clone()));
    }
    for i in (end / 2)..end {
        output.append((a.get(i).unwrap().clone(), b.get(i).unwrap().clone()));
    }
    return output;
}
