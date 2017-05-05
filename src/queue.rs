use std::fmt::Debug;
use std::fmt::Display;

use list::PfList;

pub trait PfQueue<T: Clone + Display + Debug>: Sized {
    fn new() -> Self;
    fn is_empty(&self) -> bool;
    fn snoc(&self, v: T) -> Self;
    fn head(&self) -> Result<T, &str>;
    fn tail(&self) -> Result<Self, &str>;
}

// 5.2 Batched queue
#[derive(Debug)]
pub struct PfBatchedQueue<T: Clone + Display + Debug> {
    f: PfList<T>,
    r: PfList<T>,
}

impl<T: Clone + Display + Debug> PfBatchedQueue<T> {
    fn checkf(f: &PfList<T>, r: &PfList<T>) -> Self {
        if f.is_empty() {
            Self { f: r.rev(), r: PfList::new() }
        } else {
            Self { f: f.clone(), r: r.clone() }
        }
    }
}

impl<T: Clone + Display + Debug> PfQueue<T> for PfBatchedQueue<T> {
    fn new() -> Self {
        Self {
            f: PfList::new(),
            r: PfList::new()
        }
    }

    fn is_empty(&self) -> bool {
        self.f.is_empty()
    }

    fn snoc(&self, v: T) -> Self {
        Self::checkf(&self.f, &self.r.push(v))
    }

    fn head(&self) -> Result<T, &str> {
        match self.f.pop() {
            Ok((v, _)) => Ok(v),
            Err(_) => Err("head for empty batched queue")
        }
    }

    fn tail(&self) -> Result<Self, &str> {
        match self.f.pop() {
            Ok((_, f)) => Ok(Self::checkf(&f, &self.r)),
            Err(_) => Err("tail for empty batched queue")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_pf_queue<Queue: PfQueue<i32>>() {
        let q1 = Queue::new();
        assert!(q1.is_empty());
        let q2 = q1.snoc(42);
        assert_eq!(42, q2.head().unwrap());
        let q3 = q2.snoc(1);
        let q4 = q3.snoc(2);
        assert_eq!(42, q4.head().unwrap());
        assert_eq!(1, q4.tail().unwrap().head().unwrap());
        assert_eq!(2, q4.tail().unwrap().tail().unwrap().head().unwrap());
    }

    #[test]
    fn test_pf_batched_queue() {
        test_pf_queue::<PfBatchedQueue<i32>>();
    }
}
