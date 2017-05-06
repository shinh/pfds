// 4.2 Stream

use std::fmt::Debug;

use lazy::Thunk;

#[derive(Clone, Debug)]
enum StreamNode<'a, T: 'a + Clone + Debug> {
    Nil,
    Cons(T, Thunk<'a, StreamNode<'a, T>>)
}

#[derive(Clone, Debug)]
pub struct Stream<'a, T: 'a + Clone + Debug> {
    head: Thunk<'a, StreamNode<'a, T>>
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
    fn new(t: Thunk<'a, StreamNode<'a, T>>) -> Self {
        Stream { head: t }
    }

    fn eval(&self) -> StreamNode<'a, T> {
        self.head.eval()
    }

    pub fn empty() -> Self {
        Stream::new(lazy!(StreamNode::Nil))
    }

    pub fn push(&self, v: T) -> Self {
        let tail = self.head.clone();
        Stream::new(lazy!(
            StreamNode::Cons(v.clone(), tail.clone())
        ))
    }

    pub fn make<I>(it: I) -> Self
        where I: DoubleEndedIterator<Item=T> {
        let mut r = Stream::empty();
        for v in it.rev() {
            r = r.push(v);
        }
        r
    }

    pub fn iter(&self) -> Self {
        self.clone()
    }

    pub fn head(&self) -> Result<T, &str> {
        match self.eval() {
            StreamNode::Nil => Err("head for empty stream"),
            StreamNode::Cons(v, _) => Ok(v),
        }
    }

    pub fn concat(&self, t: Self) -> Self {
        match self.eval() {
            StreamNode::Nil => t,
            StreamNode::Cons(v, s) => Stream::new(lazy!(
                StreamNode::Cons(
                    v.clone(), Stream::new(s.clone()).concat(t.clone()).head)
            ))
        }
    }

    pub fn take_n(&self, n: usize) -> Self {
        if n == 0 {
            return Self::empty();
        }
        match self.eval() {
            StreamNode::Nil => Self::empty(),
            StreamNode::Cons(v, s) => Stream::new(lazy!(
                StreamNode::Cons(v.clone(),
                                 Stream::new(s.clone()).take_n(n-1).head)
            ))
        }
    }

    fn reverse_impl(s: Thunk<'a, StreamNode<'a, T>>,
                    r: Thunk<'a, StreamNode<'a, T>>)
                    -> Thunk<'a, StreamNode<'a, T>> {
        match s.eval() {
            StreamNode::Nil => r,
            StreamNode::Cons(v, s) => {
                let c = lazy!(StreamNode::Cons(v.clone(), r.clone()));
                lazy!(Self::reverse_impl(s.clone(), c.clone()).eval())
            }
        }
    }

    pub fn reverse(&self) -> Self {
        Stream::new(Self::reverse_impl(
            self.head.clone(), lazy!(StreamNode::Nil)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream() {
        let s = Stream::make(1..4);
        assert_eq!(1, s.head().unwrap());
        assert_eq!(vec![1,2,3], s.iter().collect::<Vec<i32>>());

        let s = Stream::make(1..4).concat(Stream::make(5..7));
        assert_eq!(vec![1,2,3,5,6], s.iter().collect::<Vec<i32>>());

        let s = Stream::make(1..4).reverse();
        assert_eq!(vec![3,2,1], s.iter().collect::<Vec<i32>>());

        let s = Stream::make(1..4).concat(Stream::make(5..10));
        assert_eq!(vec![1,2,3,5,6], s.take_n(5).iter().collect::<Vec<i32>>());
    }
}
