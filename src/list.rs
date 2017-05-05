use std::fmt::Debug;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug)]
enum PfListNode<T: Copy + Display + Debug> {
    Empty,
    Node {
        value: T,
        next: Rc<PfListNode<T>>
    }
}

pub struct PfList<T: Copy + Display + Debug> {
    head: Rc<PfListNode<T>>
}

impl<T: Copy + Display + Debug> PfList<T> {
    pub fn is_empty(&self) -> bool {
        if let PfListNode::Empty = *self.head {
            return true;
        } else {
            return false;
        }
    }

    pub fn new() -> Self {
        PfList { head: Rc::new(PfListNode::Empty) }
    }

    pub fn push(&self, v: T) -> Self {
        PfList {
            head: Rc::new(PfListNode::Node {
                value: v,
                next: self.head.clone()
            })
        }
    }

    pub fn pop(&self) -> Result<(T, Self), &str> {
        match *self.head {
            PfListNode::Empty => Err("pop for empty"),
            PfListNode::Node { ref value, ref next } =>
                Ok((value.clone(), PfList { head: next.clone() }))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let l1 = PfList::<i32>::new();
        assert!(l1.is_empty());
        let l2 = l1.push(42);
        assert!(!l2.is_empty());
        let l3 = l2.push(4);
        let (v4, l4) = l3.pop().unwrap();
        assert_eq!(4, v4);
        let (v5, _) = l4.pop().unwrap();
        assert_eq!(42, v5);
        let (v6, _) = l2.pop().unwrap();
        assert_eq!(42, v6);
    }
}
