use std::fmt::Debug;
use std::fmt::Display;

use lazy::Thunk;
use list::PfList;
use stream::Stream;

pub trait PfQueue<T: Clone + Display + Debug>: Clone + Sized {
    fn new() -> Self;
    fn is_empty(&self) -> bool;
    fn snoc(&self, v: T) -> Self;
    fn head(&self) -> Result<T, &str>;
    fn tail(&self) -> Result<Self, &str>;
}

// 5.2 Batched queue
#[derive(Clone, Debug)]
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

// 6.3.2 Banker queue
#[derive(Clone, Debug)]
pub struct PfBankerQueue<'a, T: 'a + Clone + Display + Debug> {
    fl: i32,
    f: Stream<'a, T>,
    rl: i32,
    r: Stream<'a, T>,
}

impl<'a, T: 'a + Clone + Display + Debug> PfBankerQueue<'a, T> {
    fn check(fl: i32, f: Stream<'a, T>, rl: i32, r: Stream<'a, T>) -> Self {
        if rl < fl {
            Self {
                fl: fl,
                f: f.clone(),
                rl: rl,
                r: r.clone(),
            }
        } else {
            Self {
                fl: fl + rl,
                f: f.concat(r.reverse()),
                rl: 0,
                r: Stream::empty(),
            }
        }
    }
}

impl<'a, T: 'a + Clone + Display + Debug> PfQueue<T> for PfBankerQueue<'a, T> {
    fn new() -> Self {
        Self {
            fl: 0,
            f: Stream::empty(),
            rl: 0,
            r: Stream::empty()
        }
    }

    fn is_empty(&self) -> bool {
        self.fl == 0
    }

    fn snoc(&self, v: T) -> Self {
        Self::check(self.fl, self.f.clone(), self.rl + 1, self.r.push(v))
    }

    fn head(&self) -> Result<T, &str> {
        match self.f.head() {
            Ok(v) => Ok(v),
            Err(_) => Err("head for empty banker queue")
        }
    }

    fn tail(&self) -> Result<Self, &str> {
        match self.f.tail() {
            Ok(f) => Ok(Self::check(self.fl - 1, f, self.rl, self.r.clone())),
            Err(_) => Err("tail for empty banker queue")
        }
    }
}

// 7.2 Real time queue
#[derive(Clone, Debug)]
pub struct PfRealTimeQueue<'a, T: 'a + Clone + Display + Debug> {
    f: Stream<'a, T>,
    r: PfList<T>,
    s: Stream<'a, T>
}

impl<'a, T: 'a + Clone + Display + Debug> PfRealTimeQueue<'a, T> {
    fn rotate(f: Stream<'a, T>, r: PfList<T>, a: Stream<'a, T>)
              -> Stream<'a, T> {
        let (y, ys) = r.pop().unwrap();
        let na = a.push(y);
        match f.pop() {
            Ok((x, xs)) => {
                Stream::new(lazy!(
                    Self::rotate(xs.clone(), ys.clone(), na.clone()).eval()
                )).push(x.clone())
            }
            Err(_) => {
                na
            }
        }
    }

    fn exec(f: Stream<'a, T>, r: PfList<T>, s: Stream<'a, T>) -> Self {
        match s.tail() {
            Ok(t) => {
                return Self {
                    f: f.clone(),
                    r: r.clone(),
                    s: t.clone()
                }
            }
            Err(_) => {
                let fd = Self::rotate(f.clone(),
                                      r.clone(),
                                      Stream::empty());
                return Self {
                    f: fd.clone(),
                    r: PfList::new(),
                    s: fd.clone()
                }
            }
        }
    }
}

impl<'a, T: 'a + Clone + Display + Debug> PfQueue<T>
    for PfRealTimeQueue<'a, T> {
    fn new() -> Self {
        Self {
            f: Stream::empty(),
            r: PfList::new(),
            s: Stream::empty()
        }
    }

    fn is_empty(&self) -> bool {
        self.f.head().is_err()
    }

    fn snoc(&self, v: T) -> Self {
        let r = self.r.push(v);
        Self::exec(self.f.clone(), r, self.s.clone())
    }

    fn head(&self) -> Result<T, &str> {
        match self.f.head() {
            Ok(v) => Ok(v),
            Err(_) => Err("head for empty real time queue")
        }
    }

    fn tail(&self) -> Result<Self, &str> {
        match self.f.tail() {
            Ok(t) => Ok(Self::exec(t, self.r.clone(), self.s.clone())),
            Err(_) => Err("tail for empty real time queue")
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

    #[test]
    fn test_pf_banker_queue() {
        test_pf_queue::<PfBankerQueue<i32>>();
    }

    #[test]
    fn test_pf_real_time_queue() {
        test_pf_queue::<PfRealTimeQueue<i32>>();
    }
}
