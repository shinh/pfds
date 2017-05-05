// 3.1 Leftish heap

use std::fmt::Debug;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
pub enum PfLeftistHeap<T: Ord + Copy + Display + Debug> {
    Empty,
    Node {
        rank: i32,
        value: T,
        left: Rc<PfLeftistHeap<T>>,
        right: Rc<PfLeftistHeap<T>>,
    }
}

impl<T: Ord + Copy + Display + Debug> PfLeftistHeap<T> {
    pub fn new() -> Self {
        PfLeftistHeap::Empty
    }

    pub fn is_empty(&self) -> bool {
        if let PfLeftistHeap::Empty = *self {
            return true;
        } else {
            return false;
        }
    }

    fn rank(&self) -> i32 {
        match *self {
            PfLeftistHeap::Empty => 0,
            PfLeftistHeap::Node {
                rank, value: _, left: _, right: _
            } => rank
        }
    }

    fn make_heap(x: T, a: Rc<Self>, b: Rc<Self>) -> Self {
        if a.rank() >= b.rank() {
            PfLeftistHeap::Node {
                rank: b.rank() + 1,
                value: x,
                left: a,
                right: b
            }
        } else {
            PfLeftistHeap::Node {
                rank: a.rank() + 1,
                value: x,
                left: b,
                right: a
            }
        }
    }

    fn copy(&self) -> Self {
        match self {
            &PfLeftistHeap::Empty => PfLeftistHeap::Empty,
            &PfLeftistHeap::Node {
                rank, ref value, ref left, ref right
            } => PfLeftistHeap::Node {
                rank: rank,
                value: value.clone(),
                left: left.clone(),
                right: right.clone()
            }
        }
    }

    pub fn insert(&self, v: T) -> Self {
        self.merge(&PfLeftistHeap::Node {
            rank: 1,
            value: v,
            left: Rc::new(PfLeftistHeap::Empty),
            right: Rc::new(PfLeftistHeap::Empty)
        })
    }

    pub fn merge(&self, h: &Self) -> Self {
        match (self, h) {
            (&PfLeftistHeap::Empty, h) => h.copy(),
            (h, &PfLeftistHeap::Empty) => h.copy(),
            (&PfLeftistHeap::Node { rank: _, value: v1, left: ref a1, right: ref b1 },
             &PfLeftistHeap::Node { rank: _, value: v2, left: ref a2, right: ref b2 }
            ) => {
                if v1 < v2 {
                    PfLeftistHeap::make_heap(v1, a1.clone(), Rc::new(b1.merge(h)))
                } else {
                    PfLeftistHeap::make_heap(v2, a2.clone(), Rc::new(self.merge(b2)))
                }
            }
        }
    }

    pub fn find_min(&self) -> Result<T, &str> {
        match self {
            &PfLeftistHeap::Empty => Err("find_min for empty"),
            &PfLeftistHeap::Node { rank: _, value: v, left: _, right: _ } =>
                Ok(v)
        }
    }

    pub fn delete_min(&self) -> Result<Self, &str> {
        match self {
            &PfLeftistHeap::Empty => Err("delete_min for empty"),
            &PfLeftistHeap::Node { rank: _, value: _, left: ref a, right: ref b } => {
                Ok((*a).merge(&*b))
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let h1 = PfLeftistHeap::<i32>::new();
        assert!(h1.is_empty());
        let h2 = h1.insert(42);
        assert_eq!(42, h2.find_min().unwrap());
        let h3 = h2.insert(3);
        assert_eq!(3, h3.find_min().unwrap());
        let h4 = h3.insert(8);
        assert_eq!(3, h4.find_min().unwrap());

        let h5 = h4.delete_min().unwrap();
        assert_eq!(8, h5.find_min().unwrap());
        let h6 = h5.delete_min().unwrap();
        assert_eq!(42, h6.find_min().unwrap());

        assert_eq!(42, h2.find_min().unwrap());
        assert_eq!(3, h3.find_min().unwrap());
        assert_eq!(3, h4.find_min().unwrap());
        assert_eq!(8, h5.find_min().unwrap());
        assert_eq!(42, h6.find_min().unwrap());
    }
}
