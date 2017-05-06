// 4.2 Stream

use std::fmt::Debug;
use std::rc::Rc;

use lazy::Thunk;

#[derive(Clone, Debug)]
enum StreamNode<'a, T: 'a + Clone + Debug> {
    Nil,
    Cons(T, Rc<Thunk<'a, StreamNode<'a, T>>>)
}

#[derive(Clone, Debug)]
pub struct Stream<'a, T: 'a + Clone + Debug> {
    head: Rc<Thunk<'a, StreamNode<'a, T>>>
}

impl<'a, T: 'a + Clone + Debug> Iterator for Stream<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        match self.eval() {
            StreamNode::Nil => None,
            StreamNode::Cons(v, t) => {
                self.head = t.clone();
                Some(v.clone())
            }
        }
    }
}

impl<'a, T: 'a + Clone + Debug> Stream<'a, T> {
    fn new(t: Rc<Thunk<'a, StreamNode<'a, T>>>) -> Self {
        Stream { head: t }
    }

    fn eval(&mut self) -> StreamNode<'a, T> {
        self.head.eval()
    }

    pub fn empty() -> Self {
        Stream::new(Rc::new(lazy!(StreamNode::Nil)))
    }

    pub fn push(&mut self, v: T) -> Self {
        let tail = self.head.clone();
        Stream::new(Rc::new(lazy!(
            StreamNode::Cons(v.clone(), tail.clone())
        )))
    }

    pub fn make<I>(it: I) -> Self
        where I: DoubleEndedIterator<Item=T> {
        let mut r = Stream::empty();
        for v in it.rev() {
            r = r.push(v);
        }
        r
    }

    pub fn iter(&mut self) -> Self {
        self.clone()
    }

    pub fn head(&mut self) -> Result<T, &str> {
        match self.eval() {
            StreamNode::Nil => Err("head for empty stream"),
            StreamNode::Cons(v, _) => Ok(v),
        }
    }

    pub fn concat(&mut self, t: Self) -> Self {
        match self.eval() {
            StreamNode::Nil => t,
            StreamNode::Cons(v, s) => Stream::new(Rc::new(lazy!(
                StreamNode::Cons(
                    v.clone(), Stream::new(s.clone()).concat(t.clone()).head)
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream() {
        let mut s = Stream::make(1..4);
        assert_eq!(1, s.head().unwrap());
        assert_eq!(vec![1,2,3], s.iter().collect::<Vec<i32>>());
    }
}
