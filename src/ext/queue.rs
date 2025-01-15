use std::fmt::{self, Debug};
use std::mem::MaybeUninit;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum QueueError {
    QueueFull,
    QueueAlloc,
}

impl fmt::Display for QueueError {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        match self {
            QueueError::QueueFull => write!(f, "Queue ran out of space."),
            QueueError::QueueAlloc => write!(f, "Queue allocation failure."),
        }
    }
}

pub trait Queue<T> {
    fn back(&self) -> Option<&T>;
    fn back_mut(&mut self) -> Option<&mut T>;
    fn peek(&self) -> Option<&T>;
    fn peek_mut(&mut self) -> Option<&mut T>;
    fn get(&self, idx:usize) -> Option<&T>;
    fn get_mut(&mut self, idx:usize) -> Option<&mut T>;

    fn push(&mut self, val:T) -> Result<(), QueueError>;
    fn try_push(&mut self, val:T) -> bool;
    fn ext<I:IntoIterator<Item=T>>(&mut self,it:I) -> Result<(),QueueError>;
    fn try_ext<I:IntoIterator<Item = T>>(&mut self, iter:I) -> bool;
    fn pop(&mut self) -> Option<T>;
    fn drain(&mut self) -> Vec<T>;
    fn drain_part(&mut self, num:usize) -> Vec<T>;
    fn clear(&mut self);

    fn capacity(&self) -> usize;
    fn remaining(&self) -> usize;
    fn size(&self) -> usize;

    fn full(&self) -> bool;
    fn empty(&self) -> bool;
}

pub struct StaticQueue<T, const N:usize> {
    pub(crate) data:[MaybeUninit<T>; N],
    pub(crate) head:usize,
    pub(crate) tail:usize,
    pub(crate) size:usize,
}

impl <T:Debug, const N:usize> Debug for StaticQueue<T, N> {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StaticQueue")
            .field("capacity", &N)
            .field("size", &self.size())
            .field("elements", &self.iter().collect::<Vec<_>>())
            .finish()
    }
}

impl<T, const N:usize> Default for StaticQueue<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N:usize> StaticQueue<T, N> {
    pub fn new() -> Self {
        assert!(N > 0, "Queue length must not be 0.");
        Self {
            data:unsafe{MaybeUninit::uninit().assume_init()},
            head:0,
            tail:0,
            size:0,
        }
    }

    pub fn iter(&self) -> Iter<'_, T, N> {
        Iter { queue: self, idx: 0 }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T, N> {
        IterMut { queue: self, idx: 0 }
    }
}

impl<T, const N:usize> Queue<T> for StaticQueue<T, N> {
    fn back(&self) -> Option<&T> {
        if self.empty() { None } else {
            let idx:usize = (self.tail + N - 1) % N;
            Some(unsafe{self.data[idx].assume_init_ref()})
        }
    }

    fn back_mut(&mut self) -> Option<&mut T> {
        if self.empty() { None } else {
            let idx:usize = (self.tail + N - 1) % N;
            Some(unsafe{self.data[idx].assume_init_mut()})
        }
    }

    fn peek(&self) -> Option<&T> {
        if self.empty() { None } else {
            Some(unsafe{self.data[self.head].assume_init_ref()})
        }
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        if self.empty() { None } else {
            Some(unsafe{self.data[self.head].assume_init_mut()})
        }
    }

    fn get(&self, idx:usize) -> Option<&T> {
        if idx >= self.size { None } else {
            Some(unsafe{self.data[(self.head + idx) % N].assume_init_ref()})
        }
    }

    fn get_mut(&mut self, idx:usize) -> Option<&mut T> {
        if idx >= self.size { None } else {
            Some(unsafe{self.data[(self.head + idx) % N].assume_init_mut()})
        }
    }

    fn push(&mut self, val:T) -> Result<(), QueueError> {
        if self.full() { return Err(QueueError::QueueFull) }
        self.data[self.tail].write(val);
        self.tail = (self.tail + 1) % N;
        self.size += 1;
        Ok(())
    }

    fn try_push(&mut self, val:T) -> bool {
        !self.full() && self.push(val).is_ok()
    }

    fn ext<I:IntoIterator<Item=T>>(&mut self,it:I) -> Result<(),QueueError> {
        for i in it { self.push(i)?; } Ok(())
    }

    fn try_ext<I:IntoIterator<Item=T>>(&mut self,it:I) -> bool {
        for i in it { if self.push(i).is_err() { return false; } } true
    }

    fn pop(&mut self) -> Option<T> {
        if self.empty() { return None; }
        let v = unsafe{self.data[self.head].assume_init_read()};
        self.head = (self.head + 1) % N;
        self.size -= 1;
        Some(v)
    }

    fn drain(&mut self) -> Vec<T> {
        let mut v = Vec::with_capacity(self.size);
        while let Some(val) = self.pop() { v.push(val); }
        v
    }

    fn drain_part(&mut self, num:usize) -> Vec<T> {
        let mut v = Vec::with_capacity(num);
        for _ in 0..num.min(self.size) {
            if let Some(val) = self.pop() { v.push(val); } else { break; }
        }
        v
    }

    fn clear(&mut self) {
        while self.pop().is_some() {}
    }

    fn capacity(&self) -> usize {
        N
    }

    fn remaining(&self) -> usize {
        self.capacity() - self.size()
    }

    fn size(&self) -> usize {
        self.size
    }

    fn full(&self) -> bool {
        self.size() == self.capacity()
    }

    fn empty(&self) -> bool {
        self.size() == 0
    }
}

pub struct Iter<'a, T, const N:usize> {
    queue:&'a StaticQueue<T, N>,
    idx:usize,
}

impl<'a, T, const N:usize> Iterator for Iter<'a, T, N> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.queue.size() { return None; }
        let idx = (self.queue.head + self.idx) % N;
        self.idx += 1;
        Some(unsafe{self.queue.data[idx].assume_init_ref()})
    }
}

pub struct IterMut<'a, T, const N:usize> {
    queue:&'a mut StaticQueue<T, N>,
    idx:usize,
}

impl<'a, T, const N:usize> Iterator for IterMut<'a, T, N> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.queue.size() { return None; }
        let idx = (self.queue.head + self.idx) % N;
        self.idx += 1;
        unsafe {Some(&mut *self.queue.data[idx].as_mut_ptr())}
    }
}

pub struct DynamicQueue<T, const N:usize> {
    data:VecDeque<T>,
    min_size:usize,
}

impl<T, const N:usize> Default for DynamicQueue<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N:usize> DynamicQueue<T, N> {
    pub fn new() -> Self {
        assert!(N > 0, "Queue length must not be 0.");
        Self {
            data:VecDeque::with_capacity(N),
            min_size:N,
        }
    }

    fn adjust_size(&mut self) {
        if self.data.len() > self.min_size { self.data.shrink_to_fit(); }
    }
}

impl<T, const N:usize> Queue<T> for DynamicQueue<T, N> {
    fn back(&self) -> Option<&T> {
        self.data.back()
    }

    fn back_mut(&mut self) -> Option<&mut T> {
        self.data.back_mut()
    }

    fn peek(&self) -> Option<&T> {
        self.data.front()
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.front_mut()
    }

    fn get(&self, idx:usize) -> Option<&T> {
        self.data.get(idx)
    }

    fn get_mut(&mut self, idx:usize) -> Option<&mut T> {
        self.data.get_mut(idx)
    }

    fn push(&mut self, val:T) -> Result<(), QueueError> {
        if self.data.len() == self.data.capacity() {
            self.data.reserve(1);
        }
        self.data.push_back(val);
        Ok(())
    }

    fn try_push(&mut self, val:T) -> bool {
        self.push(val).is_ok()
    }

    fn ext<I:IntoIterator<Item=T>>(&mut self,it:I) -> Result<(), QueueError> {
        for i in it { self.push(i)?; } Ok(())
    }

    fn try_ext<I:IntoIterator<Item=T>>(&mut self,it:I) -> bool {
        for i in it { if self.push(i).is_err() { return false; } } true
    }

    fn pop(&mut self) -> Option<T> {
        let val = self.data.pop_front();
        self.adjust_size();
        val
    }

    fn drain(&mut self) -> Vec<T> {
        let val = self.data.drain(..).collect();
        self.adjust_size();
        val
    }

    fn drain_part(&mut self, num:usize) -> Vec<T> {
        let val: Vec<T> = self.data.drain(..num.min(self.size())).collect();
        self.adjust_size();
        val
    }

    fn clear(&mut self) {
        self.data.clear();
        self.adjust_size();
    }

    fn capacity(&self) -> usize {
        N
    }

    fn remaining(&self) -> usize {
        self.capacity() - self.size()
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn full(&self) -> bool {
        self.size() == self.capacity()
    }

    fn empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_queue_core() {
        let mut queue = StaticQueue::<i32, 3>::new();
        
        assert_eq!(queue.capacity(), 3);

        assert!(queue.push(0).is_ok());
        assert!(queue.ext(vec![1, 2]).is_ok());
        assert!(queue.push(3).is_err());

        assert_eq!(queue.full(), true);
        assert_eq!(queue.empty(), false);
        assert_eq!(queue.size(), 3);

        assert_eq!(queue.pop(), Some(0));
        assert_eq!(queue.remaining(), 1);
        assert_eq!(queue.drain(), vec![1, 2]);
        assert_eq!(queue.pop(), None);

        assert!(queue.ext(vec![0, 1, 2]).is_ok());
        assert_eq!(queue.drain_part(2), vec![0, 1]);
        assert_eq!(queue.try_push(3), true);
        assert_eq!(queue.drain_part(5), vec![2, 3]);
    }
}
