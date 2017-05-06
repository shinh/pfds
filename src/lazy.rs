use std::boxed::Box;

pub struct Thunk<'a, T: 'a + Clone> {
    thunk: Box<Fn() -> T + 'a>,
    value: Option<T>,
}

impl<'a, T: Clone> Thunk<'a, T> {
    pub fn new<F>(t: F) -> Self
        where F: Fn() -> T + 'a {
        Thunk {
            thunk: Box::new(t),
            value: None }
    }

    pub fn eval(&mut self) -> T {
        match self.value {
            Some(ref value) => value.clone(),
            None => {
                let value = (self.thunk)();
                self.value = Some(value.clone());
                value
            }
        }
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
        let mut result = lazy!(2 + 3);
        assert_eq!(5, result.eval());
        assert_eq!(5, result.eval());

        let mut result = lazy!(inc());
        assert_eq!(8, result.eval());
        assert_eq!(8, result.eval());
    }
}
