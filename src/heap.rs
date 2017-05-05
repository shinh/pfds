// 3.1 Leftish heap

use std::fmt::Debug;
use std::fmt::Display;
use std::rc::Rc;

trait PfHeap<T: Ord + Clone + Display + Debug>: Sized {
    fn is_empty(&self) -> bool;
    fn insert(&self, v: T) -> Self;
    fn merge(&self, h: &Self) -> Self;
    fn find_min(&self) -> Result<T, &str>;
    fn delete_min(&self) -> Result<Self, &str>;
}

#[derive(Debug)]
pub enum PfLeftistHeap<T: Ord + Clone + Display + Debug> {
    Empty,
    Node {
        rank: i32,
        value: T,
        left: Rc<PfLeftistHeap<T>>,
        right: Rc<PfLeftistHeap<T>>,
    }
}

impl<T: Ord + Clone + Display + Debug> PfLeftistHeap<T> {
    pub fn new() -> Self {
        PfLeftistHeap::Empty
    }

    fn rank(&self) -> i32 {
        match self {
            &PfLeftistHeap::Empty => 0,
            &PfLeftistHeap::Node { rank, .. } => rank
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
}

impl<T: Ord + Clone + Display + Debug> Clone for PfLeftistHeap<T> {
    fn clone(&self) -> Self {
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
}

impl<T: Ord + Clone + Display + Debug> PfHeap<T> for PfLeftistHeap<T> {
    fn is_empty(&self) -> bool {
        if let PfLeftistHeap::Empty = *self {
            return true;
        } else {
            return false;
        }
    }

    fn insert(&self, v: T) -> Self {
        self.merge(&PfLeftistHeap::Node {
            rank: 1,
            value: v,
            left: Rc::new(PfLeftistHeap::Empty),
            right: Rc::new(PfLeftistHeap::Empty)
        })
    }

    fn merge(&self, h: &Self) -> Self {
        use self::PfLeftistHeap::*;
        match (self, h) {
            (&Empty, h) => h.clone(),
            (h, &Empty) => h.clone(),
            (&Node { rank: _, value: ref v1, left: ref a1, right: ref b1 },
             &Node { rank: _, value: ref v2, left: ref a2, right: ref b2 }
            ) => {
                if v1 < v2 {
                    PfLeftistHeap::make_heap(v1.clone(), a1.clone(),
                                             Rc::new(b1.merge(h)))
                } else {
                    PfLeftistHeap::make_heap(v2.clone(), a2.clone(),
                                             Rc::new(self.merge(b2)))
                }
            }
        }
    }

    fn find_min(&self) -> Result<T, &str> {
        match self {
            &PfLeftistHeap::Empty => Err("find_min for empty"),
            &PfLeftistHeap::Node { ref value, .. } => Ok(value.clone())
        }
    }

    fn delete_min(&self) -> Result<Self, &str> {
        match self {
            &PfLeftistHeap::Empty => Err("delete_min for empty"),
            &PfLeftistHeap::Node { ref left, ref right, .. } =>
                Ok(left.merge(right))
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
