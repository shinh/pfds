use std::fmt::Debug;
use std::fmt::Display;
use std::rc::Rc;

use list::PfList;

trait PfHeap<T: Ord + Clone + Display + Debug>: Sized {
    fn new() -> Self;
    fn is_empty(&self) -> bool;
    fn insert(&self, v: T) -> Self;
    fn merge(&self, h: &Self) -> Self;
    fn find_min(&self) -> Result<T, &str>;
    fn delete_min(&self) -> Result<Self, &str>;
}

// 3.1 Leftish heap
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
    fn new() -> Self {
        PfLeftistHeap::Empty
    }

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
            &PfLeftistHeap::Empty => Err("find_min for empty leftish heap"),
            &PfLeftistHeap::Node { ref value, .. } => Ok(value.clone())
        }
    }

    fn delete_min(&self) -> Result<Self, &str> {
        match self {
            &PfLeftistHeap::Empty => Err("delete_min for empty leftish heap"),
            &PfLeftistHeap::Node { ref left, ref right, .. } =>
                Ok(left.merge(right))
        }
    }
}

// 5.5 Pairing heap
#[derive(Debug)]
pub enum PfPairingHeap<T: Ord + Clone + Display + Debug> {
    Empty,
    Node {
        value: T,
        children: PfList<PfPairingHeap<T>>
    }
}

impl<T: Ord + Clone + Display + Debug> PfPairingHeap<T> {
    fn merge_pairs(hs: PfList<PfPairingHeap<T>>) -> Self {
        match hs.pop() {
            Ok((h1, hs)) => {
                match hs.pop() {
                    Ok((h2, hs)) => {
                        h1.merge(&h2).merge(&Self::merge_pairs(hs))
                    }
                    Err(_) => h1
                }
            }
            Err(_) => PfPairingHeap::Empty
        }
    }
}

impl<T: Ord + Clone + Display + Debug> Clone for PfPairingHeap<T> {
    fn clone(&self) -> Self {
        match self {
            &PfPairingHeap::Empty => PfPairingHeap::Empty,
            &PfPairingHeap::Node {
                ref value, ref children
            } => PfPairingHeap::Node {
                value: value.clone(),
                children: children.clone()
            }
        }
    }
}

impl<T: Ord + Clone + Display + Debug> PfHeap<T> for PfPairingHeap<T> {
    fn new() -> Self {
        PfPairingHeap::Empty
    }

    fn is_empty(&self) -> bool {
        if let PfPairingHeap::Empty = *self {
            return true;
        } else {
            return false;
        }
    }

    fn insert(&self, v: T) -> Self {
        self.merge(&PfPairingHeap::Node {
            value: v,
            children: PfList::new()
        })
    }

    fn merge(&self, h: &Self) -> Self {
        use self::PfPairingHeap::*;
        match (self, h) {
            (&Empty, h) => h.clone(),
            (h, &Empty) => h.clone(),
            (&Node { value: ref v1, children: ref hs1 },
             &Node { value: ref v2, children: ref hs2 }
            ) => {
                if v1 < v2 {
                    PfPairingHeap::Node {
                        value: v1.clone(),
                        children: hs1.push(Node {
                            value: v2.clone(),
                            children: hs2.clone()
                        })
                    }
                } else {
                    PfPairingHeap::Node {
                        value: v2.clone(),
                        children: hs2.push(Node {
                            value: v1.clone(),
                            children: hs1.clone()
                        })
                    }
                }
            }
        }
    }

    fn find_min(&self) -> Result<T, &str> {
        match self {
            &PfPairingHeap::Empty => Err("find_min for empty pairing heap"),
            &PfPairingHeap::Node { ref value, .. } => Ok(value.clone())
        }
    }

    fn delete_min(&self) -> Result<Self, &str> {
        match self {
            &PfPairingHeap::Empty => Err("delete_min for empty pairing heap"),
            &PfPairingHeap::Node { ref children, .. } =>
                Ok(Self::merge_pairs(children.clone()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_pf_heap<Heap: PfHeap<i32>>() {
        let h1 = Heap::new();
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

    #[test]
    fn test_pf_leftish_heap() {
        test_pf_heap::<PfLeftistHeap<i32>>();
    }

    #[test]
    fn test_pf_pairing_heap() {
        test_pf_heap::<PfPairingHeap<i32>>();
    }
}
