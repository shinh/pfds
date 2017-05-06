use std::boxed::Box;
use std::cell::RefCell;
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

struct Impl<'a, T: 'a + Clone + Debug> {
    thunk: Box<Fn() -> T + 'a>,
    value: Option<T>,
}

#[derive(Clone, Debug)]
pub struct Thunk<'a, T: 'a + Clone + Debug> {
    imp: Rc<RefCell<Impl<'a, T>>>
}

impl<'a, T: 'a + Clone + Debug> Debug for Impl<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.value {
            Some(ref v) => f.write_fmt(format_args!("Impl({:?})", v)),
            None => f.write_str("Impl(*todo*)"),
        }
    }
}

impl<'a, T: Clone + Debug> Thunk<'a, T> {
    pub fn new<F>(t: F) -> Self
        where F: Fn() -> T + 'a {
        Thunk {
            imp: Rc::new(RefCell::new(Impl {
                thunk: Box::new(t),
                value: None
            }))
        }
    }

    pub fn eval(&self) -> T {
        let mut imp = self.imp.borrow_mut();
        let value = match imp.value {
            Some(ref value) => {
                return value.clone()
            }
            None => {
                let value = (imp.thunk)();
                value
            }
        };
        imp.value = Some(value.clone());
        value
    }
}

#[macro_export]
macro_rules! lazy {
    ($e:expr) => { Thunk::new(move || { $e }) }
}

#[cfg(test)]
mod tests {
    use super::*;

    static mut V: i32 = 7;
    fn inc() -> i32 {
        unsafe {
            V += 1;
            V
        }
    }

    #[test]
    fn test_lazy() {
        let result = lazy!(2 + 3);
        assert_eq!(5, result.eval());
        assert_eq!(5, result.eval());

        let result = lazy!(inc());
        assert_eq!(8, result.eval());
        assert_eq!(8, result.eval());
    }
}
